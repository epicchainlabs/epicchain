// Copyright (C) 2021-2024 EpicChain Labs.

//
// MainService.Vote.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.ConsoleService;
using EpicChain.Cryptography.ECC;
using EpicChain.Extensions;
using EpicChain.Json;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.VM.Types;
using EpicChain.Wallets;
using System;
using System.Numerics;

namespace EpicChain.CLI
{
    partial class MainService
    {
        /// <summary>
        /// Process "register candidate" command
        /// </summary>
        /// <param name="account">register account scriptHash</param>
        [ConsoleCommand("register candidate", Category = "Vote Commands")]
        private void OnRegisterCandidateCommand(UInt160 account)
        {
            var testEpicPulse = NativeContract.EpicChain.GetRegisterPrice(EpicChainSystem.StoreView) + (BigInteger)Math.Pow(10, NativeContract.EpicPulse.Decimals) * 10;
            if (NoWallet()) return;
            WalletAccount currentAccount = CurrentWallet!.GetAccount(account);

            if (currentAccount == null)
            {
                ConsoleHelper.Warning("This address isn't in your wallet!");
                return;
            }
            else
            {
                if (currentAccount.Lock || currentAccount.WatchOnly)
                {
                    ConsoleHelper.Warning("Locked or WatchOnly address.");
                    return;
                }
            }

            ECPoint? publicKey = currentAccount.GetKey()?.PublicKey;
            byte[] script;
            using (ScriptBuilder scriptBuilder = new())
            {
                scriptBuilder.EmitDynamicCall(NativeContract.EpicChain.Hash, "registerCandidate", publicKey);
                script = scriptBuilder.ToArray();
            }

            SendTransaction(script, account, (long)testEpicPulse);
        }

        /// <summary>
        /// Process "unregister candidate" command
        /// </summary>
        /// <param name="account">unregister account scriptHash</param>
        [ConsoleCommand("unregister candidate", Category = "Vote Commands")]
        private void OnUnregisterCandidateCommand(UInt160 account)
        {
            if (NoWallet()) return;
            WalletAccount currentAccount = CurrentWallet!.GetAccount(account);

            if (currentAccount == null)
            {
                ConsoleHelper.Warning("This address isn't in your wallet!");
                return;
            }
            else
            {
                if (currentAccount.Lock || currentAccount.WatchOnly)
                {
                    ConsoleHelper.Warning("Locked or WatchOnly address.");
                    return;
                }
            }

            ECPoint? publicKey = currentAccount?.GetKey()?.PublicKey;
            byte[] script;
            using (ScriptBuilder scriptBuilder = new())
            {
                scriptBuilder.EmitDynamicCall(NativeContract.EpicChain.Hash, "unregisterCandidate", publicKey);
                script = scriptBuilder.ToArray();
            }

            SendTransaction(script, account);
        }

        /// <summary>
        /// Process "vote" command
        /// </summary>
        /// <param name="senderAccount">Sender account</param>
        /// <param name="publicKey">Voting publicKey</param>
        [ConsoleCommand("vote", Category = "Vote Commands")]
        private void OnVoteCommand(UInt160 senderAccount, ECPoint publicKey)
        {
            if (NoWallet()) return;
            byte[] script;
            using (ScriptBuilder scriptBuilder = new())
            {
                scriptBuilder.EmitDynamicCall(NativeContract.EpicChain.Hash, "vote", senderAccount, publicKey);
                script = scriptBuilder.ToArray();
            }

            SendTransaction(script, senderAccount);
        }

        /// <summary>
        /// Process "unvote" command
        /// </summary>
        /// <param name="senderAccount">Sender account</param>
        [ConsoleCommand("unvote", Category = "Vote Commands")]
        private void OnUnvoteCommand(UInt160 senderAccount)
        {
            if (NoWallet()) return;
            byte[] script;
            using (ScriptBuilder scriptBuilder = new())
            {
                scriptBuilder.EmitDynamicCall(NativeContract.EpicChain.Hash, "vote", senderAccount, null);
                script = scriptBuilder.ToArray();
            }

            SendTransaction(script, senderAccount);
        }

        /// <summary>
        /// Process "get candidates"
        /// </summary>
        [ConsoleCommand("get candidates", Category = "Vote Commands")]
        private void OnGetCandidatesCommand()
        {
            if (!OnInvokeWithResult(NativeContract.EpicChain.Hash, "getCandidates", out StackItem result, null, null, false)) return;

            var resJArray = (VM.Types.Array)result;

            if (resJArray.Count > 0)
            {
                Console.WriteLine();
                ConsoleHelper.Info("Candidates:");

                foreach (var item in resJArray)
                {
                    var value = (VM.Types.Array)item;
                    if (value is null) continue;

                    Console.Write(((ByteString)value[0])?.GetSpan().ToHexString() + "\t");
                    Console.WriteLine(((Integer)value[1]).GetInteger());
                }
            }
        }

        /// <summary>
        /// Process "get committee"
        /// </summary>
        [ConsoleCommand("get committee", Category = "Vote Commands")]
        private void OnGetCommitteeCommand()
        {
            if (!OnInvokeWithResult(NativeContract.EpicChain.Hash, "getCommittee", out StackItem result, null, null, false)) return;

            var resJArray = (VM.Types.Array)result;

            if (resJArray.Count > 0)
            {
                Console.WriteLine();
                ConsoleHelper.Info("Committee:");

                foreach (var item in resJArray)
                {
                    Console.WriteLine(((ByteString)item)?.GetSpan().ToHexString());
                }
            }
        }

        /// <summary>
        /// Process "get next validators"
        /// </summary>
        [ConsoleCommand("get next validators", Category = "Vote Commands")]
        private void OnGetNextBlockValidatorsCommand()
        {
            if (!OnInvokeWithResult(NativeContract.EpicChain.Hash, "getNextBlockValidators", out StackItem result, null, null, false)) return;

            var resJArray = (VM.Types.Array)result;

            if (resJArray.Count > 0)
            {
                Console.WriteLine();
                ConsoleHelper.Info("Next validators:");

                foreach (var item in resJArray)
                {
                    Console.WriteLine(((ByteString)item)?.GetSpan().ToHexString());
                }
            }
        }

        /// <summary>
        /// Process "get accountstate"
        /// </summary>
        [ConsoleCommand("get accountstate", Category = "Vote Commands")]
        private void OnGetAccountState(UInt160 address)
        {
            string notice = "No vote record!";
            var arg = new JObject
            {
                ["type"] = "Hash160",
                ["value"] = address.ToString()
            };

            if (!OnInvokeWithResult(NativeContract.EpicChain.Hash, "getAccountState", out StackItem result, null, new JArray(arg))) return;
            Console.WriteLine();
            if (result.IsNull)
            {
                ConsoleHelper.Warning(notice);
                return;
            }
            var resJArray = (VM.Types.Array)result;
            if (resJArray is null)
            {
                ConsoleHelper.Warning(notice);
                return;
            }

            foreach (StackItem value in resJArray)
            {
                if (value.IsNull)
                {
                    ConsoleHelper.Warning(notice);
                    return;
                }
            }
            var publickey = ECPoint.Parse(((ByteString)resJArray[2])?.GetSpan().ToHexString(), ECCurve.Secp256r1);
            ConsoleHelper.Info("Voted: ", Contract.CreateSignatureRedeemScript(publickey).ToScriptHash().ToAddress(EpicChainSystem.Settings.AddressVersion));
            ConsoleHelper.Info("Amount: ", new BigDecimal(((Integer)resJArray[0]).GetInteger(), NativeContract.EpicChain.Decimals).ToString());
            ConsoleHelper.Info("Block: ", ((Integer)resJArray[1]).GetInteger().ToString());
        }
    }
}
