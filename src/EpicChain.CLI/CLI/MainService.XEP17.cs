// =============================================================================================
//  © Copyright (C) 2021-2025 EpicChain Labs. All rights reserved.
// =============================================================================================
//
//  File: MainService.XEP17.cs
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
using EpicChain.Json;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM.Types;
using EpicChain.Wallets;
using System;
using System.Linq;
using Array = System.Array;

namespace EpicChain.CLI
{
    partial class MainService
    {
        /// <summary>
        /// Process "transfer" command
        /// </summary>
        /// <param name="tokenHash">Script hash</param>
        /// <param name="to">To</param>
        /// <param name="amount">Amount</param>
        /// <param name="from">From</param>
        /// <param name="data">Data</param>
        /// <param name="signersAccounts">Signer's accounts</param>
        [ConsoleCommand("transfer", Category = "XEP17 Commands")]
        private void OnTransferCommand(UInt160 tokenHash, UInt160 to, decimal amount, UInt160? from = null, string? data = null, UInt160[]? signersAccounts = null)
        {
            var snapshot = EpicChainSystem.StoreView;
            var asset = new AssetDescriptor(snapshot, EpicChainSystem.Settings, tokenHash);
            var value = new BigDecimal(amount, asset.Decimals);

            if (NoWallet()) return;

            Transaction tx;
            try
            {
                tx = CurrentWallet!.MakeTransaction(snapshot, new[]
                {
                    new TransferOutput
                    {
                        AssetId = tokenHash,
                        Value = value,
                        ScriptHash = to,
                        Data = data
                    }
                }, from: from, cosigners: signersAccounts?.Select(p => new Signer
                {
                    // default access for transfers should be valid only for first invocation
                    Scopes = WitnessScope.CalledByEntry,
                    Account = p
                })
                .ToArray() ?? Array.Empty<Signer>());
            }
            catch (InvalidOperationException e)
            {
                ConsoleHelper.Error(GetExceptionMessage(e));
                return;
            }
            if (!ConsoleHelper.ReadUserInput("Relay tx(no|yes)").IsYes())
            {
                return;
            }
            SignAndSendTx(snapshot, tx);
        }

        /// <summary>
        /// Process "balanceOf" command
        /// </summary>
        /// <param name="tokenHash">Script hash</param>
        /// <param name="address">Address</param>
        [ConsoleCommand("balanceOf", Category = "XEP17 Commands")]
        private void OnBalanceOfCommand(UInt160 tokenHash, UInt160 address)
        {
            var arg = new JObject
            {
                ["type"] = "Hash160",
                ["value"] = address.ToString()
            };

            var asset = new AssetDescriptor(EpicChainSystem.StoreView, EpicChainSystem.Settings, tokenHash);

            if (!OnInvokeWithResult(tokenHash, "balanceOf", out StackItem balanceResult, null, new JArray(arg))) return;

            var balance = new BigDecimal(((PrimitiveType)balanceResult).GetInteger(), asset.Decimals);

            Console.WriteLine();
            ConsoleHelper.Info($"{asset.AssetName} balance: ", $"{balance}");
        }

        /// <summary>
        /// Process "name" command
        /// </summary>
        /// <param name="tokenHash">Script hash</param>
        [ConsoleCommand("name", Category = "XEP17 Commands")]
        private void OnNameCommand(UInt160 tokenHash)
        {
            ContractState contract = NativeContract.ContractManagement.GetContract(EpicChainSystem.StoreView, tokenHash);
            if (contract == null) Console.WriteLine($"Contract hash not exist: {tokenHash}");
            else ConsoleHelper.Info("Result: ", contract.Manifest.Name);
        }

        /// <summary>
        /// Process "decimals" command
        /// </summary>
        /// <param name="tokenHash">Script hash</param>
        [ConsoleCommand("decimals", Category = "XEP17 Commands")]
        private void OnDecimalsCommand(UInt160 tokenHash)
        {
            if (!OnInvokeWithResult(tokenHash, "decimals", out StackItem result)) return;

            ConsoleHelper.Info("Result: ", $"{((PrimitiveType)result).GetInteger()}");
        }

        /// <summary>
        /// Process "totalSupply" command
        /// </summary>
        /// <param name="tokenHash">Script hash</param>
        [ConsoleCommand("totalSupply", Category = "XEP17 Commands")]
        private void OnTotalSupplyCommand(UInt160 tokenHash)
        {
            if (!OnInvokeWithResult(tokenHash, "totalSupply", out StackItem result)) return;

            var asset = new AssetDescriptor(EpicChainSystem.StoreView, EpicChainSystem.Settings, tokenHash);
            var totalSupply = new BigDecimal(((PrimitiveType)result).GetInteger(), asset.Decimals);

            ConsoleHelper.Info("Result: ", $"{totalSupply}");
        }
    }
}
