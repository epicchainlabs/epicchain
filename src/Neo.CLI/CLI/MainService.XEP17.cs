// 
// Copyright (C) 2021-2024 EpicChain Lab's
// All rights reserved.
// 
// This file is part of the EpicChain project, developed by xmoohad.
// 
// This file is subject to the terms and conditions defined in the LICENSE file found in the top-level 
// directory of this distribution. Unauthorized copying, modification, or distribution of this file,
// via any medium, is strictly prohibited. Any use of this file without explicit permission from EpicChain Lab's
// is a violation of copyright law and will be prosecuted to the fullest extent possible.
// 
// This file is licensed under the MIT License; you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     https://opensource.org/licenses/MIT
// 
// Unless required by applicable law or agreed to in writing, software distributed under the License is distributed 
// on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for 
// the specific language governing permissions and limitations under the License.
// 
// For more information about EpicChain Lab's projects and innovations, visit our website at https://epic-chain.org
// or contact us at xmoohad@epic-chain.org.
// 
//

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
        [ConsoleCommand("transfer", Category = "XEP17 Commands")]
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
        [ConsoleCommand("balanceOf", Category = "XEP17 Commands")]
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
        [ConsoleCommand("name", Category = "XEP17 Commands")]
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

            var asset = new AssetDescriptor(NeoSystem.StoreView, NeoSystem.Settings, tokenHash);
            var totalSupply = new BigDecimal(((PrimitiveType)result).GetInteger(), asset.Decimals);

            ConsoleHelper.Info("Result: ", $"{totalSupply}");
        }
    }
}
