// Copyright (C) 2021-2024 EpicChain Labs.

//
// ConsensusService.Check.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.IO;
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Plugins.DBFTPlugin.Messages;
using EpicChain.Plugins.DBFTPlugin.Types;
using System;
using System.Linq;

namespace EpicChain.Plugins.DBFTPlugin.Consensus
{
    partial class ConsensusService
    {
        private bool CheckPrepareResponse()
        {
            if (context.TransactionHashes.Length == context.Transactions.Count)
            {
                // if we are the primary for this view, but acting as a backup because we recovered our own
                // previously sent prepare request, then we don't want to send a prepare response.
                if (context.IsPrimary || context.WatchOnly) return true;

                // Check maximum block size via Native Contract policy
                if (context.GetExpectedBlockSize() > dbftSettings.MaxBlockSize)
                {
                    Log($"Rejected block: {context.Block.Index} The size exceed the policy", LogLevel.Warning);
                    RequestChangeView(ChangeViewReason.BlockRejectedByPolicy);
                    return false;
                }
                // Check maximum block system fee via Native Contract policy
                if (context.GetExpectedBlockSystemFee() > dbftSettings.MaxBlockSystemFee)
                {
                    Log($"Rejected block: {context.Block.Index} The system fee exceed the policy", LogLevel.Warning);
                    RequestChangeView(ChangeViewReason.BlockRejectedByPolicy);
                    return false;
                }

                // Timeout extension due to prepare response sent
                // around 2*15/M=30.0/5 ~ 40% block time (for M=5)
                ExtendTimerByFactor(2);

                Log($"Sending {nameof(PrepareResponse)}");
                localNode.Tell(new LocalNode.SendDirectly { Inventory = context.MakePrepareResponse() });
                CheckPreparations();
            }
            return true;
        }

        private void CheckCommits()
        {
            if (context.CommitPayloads.Count(p => context.GetMessage(p)?.ViewNumber == context.ViewNumber) >= context.M && context.TransactionHashes.All(p => context.Transactions.ContainsKey(p)))
            {
                block_received_index = context.Block.Index;
                block_received_time = TimeProvider.Current.UtcNow;
                Block block = context.CreateBlock();
                Log($"Sending {nameof(Block)}: height={block.Index} hash={block.Hash} tx={block.Transactions.Length}");
                blockchain.Tell(block);
            }
        }

        private void CheckExpectedView(byte viewNumber)
        {
            if (context.ViewNumber >= viewNumber) return;
            var messages = context.ChangeViewPayloads.Select(p => context.GetMessage<ChangeView>(p)).ToArray();
            // if there are `M` change view payloads with NewViewNumber greater than viewNumber, then, it is safe to move
            if (messages.Count(p => p != null && p.NewViewNumber >= viewNumber) >= context.M)
            {
                if (!context.WatchOnly)
                {
                    ChangeView message = messages[context.MyIndex];
                    // Communicate the network about my agreement to move to `viewNumber`
                    // if my last change view payload, `message`, has NewViewNumber lower than current view to change
                    if (message is null || message.NewViewNumber < viewNumber)
                        localNode.Tell(new LocalNode.SendDirectly { Inventory = context.MakeChangeView(ChangeViewReason.ChangeAgreement) });
                }
                InitializeConsensus(viewNumber);
            }
        }

        private void CheckPreparations()
        {
            if (context.PreparationPayloads.Count(p => p != null) >= context.M && context.TransactionHashes.All(p => context.Transactions.ContainsKey(p)))
            {
                ExtensiblePayload payload = context.MakeCommit();
                Log($"Sending {nameof(Commit)}");
                context.Save();
                localNode.Tell(new LocalNode.SendDirectly { Inventory = payload });
                // Set timer, so we will resend the commit in case of a networking issue
                ChangeTimer(TimeSpan.FromMilliseconds(EpicChainSystem.Settings.MillisecondsPerBlock));
                CheckCommits();
            }
        }
    }
}
