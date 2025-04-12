// =============================================================================================
//  © Copyright (C) 2021-2025 EpicChain Labs. All rights reserved.
// =============================================================================================
//
//  File: MainService.Vote.cs
//  Project: EpicChain Labs - Core Blockchain Infrastructure
//  Author: Xmoohad (Muhammad Ibrahim Muhammad)
//
// ---------------------------------------------------------------------------------------------
//  Description:
//  This file is an integral part of the EpicChain Labs ecosystem, a forward-looking, open-source
//  blockchain initiative founded by Xmoohad. The EpicChain project aims to create a robust,
//  decentralized, and developer-friendly blockchain infrastructure that empowers innovation,
//  transparency, and digital sovereignty.
//
// ---------------------------------------------------------------------------------------------
//  Licensing:
//  This file is distributed under the permissive MIT License, which grants anyone the freedom
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of this
//  software. These rights are granted with the understanding that the original license notice
//  and copyright attribution remain intact.
//
//  For the full license text, please refer to the LICENSE file included in the root directory of
//  this repository or visit the official MIT License page at:
//  ➤ https://opensource.org/licenses/MIT
//
// ---------------------------------------------------------------------------------------------
//  Community and Contribution:
//  EpicChain Labs is deeply rooted in the principles of open-source development. We believe that
//  collaboration, transparency, and inclusiveness are the cornerstones of sustainable technology.
//
//  This file, like all components of the EpicChain ecosystem, is offered to the global development
//  community to explore, extend, and improve. Whether you're fixing bugs, optimizing performance,
//  or building new features, your contributions are welcome and appreciated.
//
//  By contributing to this project, you become part of a community dedicated to shaping the future
//  of blockchain technology. Join us in our mission to create more secure, scalable, and accessible
//  digital infrastructure for all.
//
// ---------------------------------------------------------------------------------------------
//  Terms of Use:
//  Redistribution and usage of this file in both source and compiled (binary) forms—with or without
//  modification—are fully permitted under the MIT License. Users of this software are expected to
//  adhere to the simple and clear guidelines established in the LICENSE file.
//
//  By using this file and other components of the EpicChain Labs project, you acknowledge and agree
//  to the terms of the MIT License. This ensures that the ethos of free and open software development
//  continues to flourish and remain protected.
//
// ---------------------------------------------------------------------------------------------
//  Final Note:
//  EpicChain Labs remains committed to pushing the boundaries of blockchain innovation. Whether
//  you're an experienced developer, a researcher, a student, or simply a curious enthusiast, we
//  invite you to explore the possibilities of EpicChain—and contribute toward a decentralized future.
//
//  Learn more about the project, get involved, or access full documentation at:
//  ➤ https://epic-chain.org
//
// =============================================================================================



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
