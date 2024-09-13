// Copyright (C) 2021-2024 EpicChain Labs.

//
// VerificationService.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
// distributed as free software under the MIT License, allowing for wide usage and modification
// with minimal restrictions. For comprehensive details regarding the license, please refer to
// the LICENSE file located in the root directory of the repository or visit
// http://www.opensource.org/licenses/mit-license.php.
//
// EpicChain Labs is dedicated to fostering innovation and development in the blockchain space,
// and we believe in the open-source philosophy as a way to drive progress and collaboration.
// This file, along with all associated code and documentation, is provided with the intention of
// supporting and enhancing the development community.
//
// Redistribution and use of this file in both source and binary forms, with or without
// modifications, are permitted. We encourage users to contribute to the project and respect the
// guidelines outlined in the LICENSE file. By using this software, you agree to the terms and
// conditions specified in the MIT License, ensuring the continuation of free and open software
// practices.


using Akka.Actor;
using Akka.Util.Internal;
using EpicChain.IO;
using EpicChain.Ledger;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Plugins.StateService.Network;
using EpicChain.Wallets;
using System;
using System.Collections.Concurrent;
using System.Linq;

namespace EpicChain.Plugins.StateService.Verification
{
    public class VerificationService : UntypedActor
    {
        public class ValidatedRootPersisted { public uint Index; }
        public class BlockPersisted { public uint Index; }
        public const int MaxCachedVerificationProcessCount = 10;
        private class Timer { public uint Index; }
        private static readonly uint TimeoutMilliseconds = StatePlugin._system.Settings.MillisecondsPerBlock;
        private static readonly uint DelayMilliseconds = 3000;
        private readonly Wallet wallet;
        private readonly ConcurrentDictionary<uint, VerificationContext> contexts = new ConcurrentDictionary<uint, VerificationContext>();

        public VerificationService(Wallet wallet)
        {
            this.wallet = wallet;
            StatePlugin._system.ActorSystem.EventStream.Subscribe(Self, typeof(Blockchain.RelayResult));
        }

        private void SendVote(VerificationContext context)
        {
            if (context.VoteMessage is null) return;
            Utility.Log(nameof(VerificationService), LogLevel.Info, $"relay vote, height={context.RootIndex}, retry={context.Retries}");
            StatePlugin._system.Blockchain.Tell(context.VoteMessage);
        }

        private void OnStateRootVote(Vote vote)
        {
            if (contexts.TryGetValue(vote.RootIndex, out VerificationContext context) && context.AddSignature(vote.ValidatorIndex, vote.Signature.ToArray()))
            {
                CheckVotes(context);
            }
        }

        private void CheckVotes(VerificationContext context)
        {
            if (context.IsSender && context.CheckSignatures())
            {
                if (context.StateRootMessage is null) return;
                Utility.Log(nameof(VerificationService), LogLevel.Info, $"relay state root, height={context.StateRoot.Index}, root={context.StateRoot.RootHash}");
                StatePlugin._system.Blockchain.Tell(context.StateRootMessage);
            }
        }

        private void OnBlockPersisted(uint index)
        {
            if (MaxCachedVerificationProcessCount <= contexts.Count)
            {
                contexts.Keys.OrderBy(p => p).Take(contexts.Count - MaxCachedVerificationProcessCount + 1).ForEach(p =>
                {
                    if (contexts.TryRemove(p, out var value))
                    {
                        value.Timer.CancelIfNotNull();
                    }
                });
            }
            var p = new VerificationContext(wallet, index);
            if (p.IsValidator && contexts.TryAdd(index, p))
            {
                p.Timer = Context.System.Scheduler.ScheduleTellOnceCancelable(TimeSpan.FromMilliseconds(DelayMilliseconds), Self, new Timer
                {
                    Index = index,
                }, ActorRefs.NoSender);
                Utility.Log(nameof(VerificationContext), LogLevel.Info, $"new validate process, height={index}, index={p.MyIndex}, ongoing={contexts.Count}");
            }
        }

        private void OnValidatedRootPersisted(uint index)
        {
            Utility.Log(nameof(VerificationService), LogLevel.Info, $"persisted state root, height={index}");
            foreach (var i in contexts.Where(i => i.Key <= index))
            {
                if (contexts.TryRemove(i.Key, out var value))
                {
                    value.Timer.CancelIfNotNull();
                }
            }
        }

        private void OnTimer(uint index)
        {
            if (contexts.TryGetValue(index, out VerificationContext context))
            {
                SendVote(context);
                CheckVotes(context);
                context.Timer.CancelIfNotNull();
                context.Timer = Context.System.Scheduler.ScheduleTellOnceCancelable(TimeSpan.FromMilliseconds(TimeoutMilliseconds << context.Retries), Self, new Timer
                {
                    Index = index,
                }, ActorRefs.NoSender);
                context.Retries++;
            }
        }

        private void OnVoteMessage(ExtensiblePayload payload)
        {
            if (payload.Data.Length == 0) return;
            if ((MessageType)payload.Data.Span[0] != MessageType.Vote) return;
            Vote message;
            try
            {
                message = payload.Data[1..].AsSerializable<Vote>();
            }
            catch (FormatException)
            {
                return;
            }
            OnStateRootVote(message);
        }

        protected override void OnReceive(object message)
        {
            switch (message)
            {
                case Vote v:
                    OnStateRootVote(v);
                    break;
                case BlockPersisted bp:
                    OnBlockPersisted(bp.Index);
                    break;
                case ValidatedRootPersisted root:
                    OnValidatedRootPersisted(root.Index);
                    break;
                case Timer timer:
                    OnTimer(timer.Index);
                    break;
                case Blockchain.RelayResult rr:
                    if (rr.Result == VerifyResult.Succeed && rr.Inventory is ExtensiblePayload payload && payload.Category == StatePlugin.StatePayloadCategory)
                    {
                        OnVoteMessage(payload);
                    }
                    break;
                default:
                    break;
            }
        }

        protected override void PostStop()
        {
            base.PostStop();
        }

        public static Props Props(Wallet wallet)
        {
            return Akka.Actor.Props.Create(() => new VerificationService(wallet));
        }
    }
}
