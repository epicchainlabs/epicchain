// Copyright (C) 2021-2024 EpicChain Labs.

//
// VerificationContext.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Cryptography;
using EpicChain.Cryptography.ECC;
using EpicChain.IO;
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Plugins.StateService.Network;
using EpicChain.Plugins.StateService.Storage;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.Wallets;
using System.Collections.Concurrent;
using System.IO;

namespace EpicChain.Plugins.StateService.Verification
{
    class VerificationContext
    {
        private const uint MaxValidUntilBlockIncrement = 100;
        private StateRoot root;
        private ExtensiblePayload rootPayload;
        private ExtensiblePayload votePayload;
        private readonly Wallet wallet;
        private readonly KeyPair keyPair;
        private readonly int myIndex;
        private readonly uint rootIndex;
        private readonly ECPoint[] verifiers;
        private int M => verifiers.Length - (verifiers.Length - 1) / 3;
        private readonly ConcurrentDictionary<int, byte[]> signatures = new ConcurrentDictionary<int, byte[]>();

        public int Retries;
        public bool IsValidator => myIndex >= 0;
        public int MyIndex => myIndex;
        public uint RootIndex => rootIndex;
        public ECPoint[] Verifiers => verifiers;
        public int Sender
        {
            get
            {
                int p = ((int)rootIndex - Retries) % verifiers.Length;
                return p >= 0 ? p : p + verifiers.Length;
            }
        }
        public bool IsSender => myIndex == Sender;
        public ICancelable Timer;
        public StateRoot StateRoot
        {
            get
            {
                if (root is null)
                {
                    using var snapshot = StateStore.Singleton.GetSnapshot();
                    root = snapshot.GetStateRoot(rootIndex);
                }
                return root;
            }
        }
        public ExtensiblePayload StateRootMessage => rootPayload;
        public ExtensiblePayload VoteMessage
        {
            get
            {
                if (votePayload is null)
                    votePayload = CreateVoteMessage();
                return votePayload;
            }
        }

        public VerificationContext(Wallet wallet, uint index)
        {
            this.wallet = wallet;
            Retries = 0;
            myIndex = -1;
            rootIndex = index;
            verifiers = NativeContract.QuantumGuardNexus.GetDesignatedByRole(StatePlugin._system.StoreView, Role.StateValidator, index);
            if (wallet is null) return;
            for (int i = 0; i < verifiers.Length; i++)
            {
                WalletAccount account = wallet.GetAccount(verifiers[i]);
                if (account?.HasKey != true) continue;
                myIndex = i;
                keyPair = account.GetKey();
                break;
            }
        }

        private ExtensiblePayload CreateVoteMessage()
        {
            if (StateRoot is null) return null;
            if (!signatures.TryGetValue(myIndex, out byte[] sig))
            {
                sig = StateRoot.Sign(keyPair, StatePlugin._system.Settings.Network);
                signatures[myIndex] = sig;
            }
            return CreatePayload(MessageType.Vote, new Vote
            {
                RootIndex = rootIndex,
                ValidatorIndex = myIndex,
                Signature = sig
            }, VerificationService.MaxCachedVerificationProcessCount);
        }

        public bool AddSignature(int index, byte[] sig)
        {
            if (M <= signatures.Count) return false;
            if (index < 0 || verifiers.Length <= index) return false;
            if (signatures.ContainsKey(index)) return false;
            Utility.Log(nameof(VerificationContext), LogLevel.Info, $"vote received, height={rootIndex}, index={index}");
            ECPoint validator = verifiers[index];
            byte[] hash_data = StateRoot?.GetSignData(StatePlugin._system.Settings.Network);
            if (hash_data is null || !Crypto.VerifySignature(hash_data, sig, validator))
            {
                Utility.Log(nameof(VerificationContext), LogLevel.Info, "incorrect vote, invalid signature");
                return false;
            }
            return signatures.TryAdd(index, sig);
        }

        public bool CheckSignatures()
        {
            if (StateRoot is null) return false;
            if (signatures.Count < M) return false;
            if (StateRoot.Witness is null)
            {
                Contract contract = Contract.CreateMultiSigContract(M, verifiers);
                ContractParametersContext sc = new(StatePlugin._system.StoreView, StateRoot, StatePlugin._system.Settings.Network);
                for (int i = 0, j = 0; i < verifiers.Length && j < M; i++)
                {
                    if (!signatures.TryGetValue(i, out byte[] sig)) continue;
                    sc.AddSignature(contract, verifiers[i], sig);
                    j++;
                }
                if (!sc.Completed) return false;
                StateRoot.Witness = sc.GetWitnesses()[0];
            }
            if (IsSender)
                rootPayload = CreatePayload(MessageType.StateRoot, StateRoot, MaxValidUntilBlockIncrement);
            return true;
        }

        private ExtensiblePayload CreatePayload(MessageType type, ISerializable payload, uint validBlockEndThreshold)
        {
            byte[] data;
            using (MemoryStream ms = new MemoryStream())
            using (BinaryWriter writer = new BinaryWriter(ms))
            {
                writer.Write((byte)type);
                payload.Serialize(writer);
                writer.Flush();
                data = ms.ToArray();
            }
            ExtensiblePayload msg = new ExtensiblePayload
            {
                Category = StatePlugin.StatePayloadCategory,
                ValidBlockStart = StateRoot.Index,
                ValidBlockEnd = StateRoot.Index + validBlockEndThreshold,
                Sender = Contract.CreateSignatureRedeemScript(verifiers[MyIndex]).ToScriptHash(),
                Data = data,
            };
            ContractParametersContext sc = new ContractParametersContext(StatePlugin._system.StoreView, msg, StatePlugin._system.Settings.Network);
            wallet.Sign(sc);
            msg.Witness = sc.GetWitnesses()[0];
            return msg;
        }
    }
}
