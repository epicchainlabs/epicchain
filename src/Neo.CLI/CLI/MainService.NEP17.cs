// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// MainService.NEP17.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.ConsoleService;
using Neo.Json;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using Neo.VM.Types;
using Neo.Wallets;
using System;
using System.Linq;
using Array = System.Array;

namespace Neo.CLI
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
        [ConsoleCommand("transfer", Category = "NEP17 Commands")]
        private void OnTransferCommand(UInt160 tokenHash, UInt160 to, decimal amount, UInt160? from = null, string? data = null, UInt160[]? signersAccounts = null)
        {
            var snapshot = NeoSystem.StoreView;
            var asset = new AssetDescriptor(snapshot, NeoSystem.Settings, tokenHash);
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
        [ConsoleCommand("balanceOf", Category = "NEP17 Commands")]
        private void OnBalanceOfCommand(UInt160 tokenHash, UInt160 address)
        {
            var arg = new JObject
            {
                ["type"] = "Hash160",
                ["value"] = address.ToString()
            };

            var asset = new AssetDescriptor(NeoSystem.StoreView, NeoSystem.Settings, tokenHash);

            if (!OnInvokeWithResult(tokenHash, "balanceOf", out StackItem balanceResult, null, new JArray(arg))) return;

            var balance = new BigDecimal(((PrimitiveType)balanceResult).GetInteger(), asset.Decimals);

            Console.WriteLine();
            ConsoleHelper.Info($"{asset.AssetName} balance: ", $"{balance}");
        }

        /// <summary>
        /// Process "name" command
        /// </summary>
        /// <param name="tokenHash">Script hash</param>
        [ConsoleCommand("name", Category = "NEP17 Commands")]
        private void OnNameCommand(UInt160 tokenHash)
        {
            ContractState contract = NativeContract.ContractManagement.GetContract(NeoSystem.StoreView, tokenHash);
            if (contract == null) Console.WriteLine($"Contract hash not exist: {tokenHash}");
            else ConsoleHelper.Info("Result: ", contract.Manifest.Name);
        }

        /// <summary>
        /// Process "decimals" command
        /// </summary>
        /// <param name="tokenHash">Script hash</param>
        [ConsoleCommand("decimals", Category = "NEP17 Commands")]
        private void OnDecimalsCommand(UInt160 tokenHash)
        {
            if (!OnInvokeWithResult(tokenHash, "decimals", out StackItem result)) return;

            ConsoleHelper.Info("Result: ", $"{((PrimitiveType)result).GetInteger()}");
        }

        /// <summary>
        /// Process "totalSupply" command
        /// </summary>
        /// <param name="tokenHash">Script hash</param>
        [ConsoleCommand("totalSupply", Category = "NEP17 Commands")]
        private void OnTotalSupplyCommand(UInt160 tokenHash)
        {
            if (!OnInvokeWithResult(tokenHash, "totalSupply", out StackItem result)) return;

            var asset = new AssetDescriptor(NeoSystem.StoreView, NeoSystem.Settings, tokenHash);
            var totalSupply = new BigDecimal(((PrimitiveType)result).GetInteger(), asset.Decimals);

            ConsoleHelper.Info("Result: ", $"{totalSupply}");
        }
    }
}
