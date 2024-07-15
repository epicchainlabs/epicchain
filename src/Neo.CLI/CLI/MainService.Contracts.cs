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
using System;
using System.Linq;
using System.Numerics;

namespace Neo.CLI
{
    partial class MainService
    {
        /// <summary>
        /// Process "deploy" command
        /// </summary>
        /// <param name="filePath">File path</param>
        /// <param name="manifestPath">Manifest path</param>
        /// <param name="data">Extra data for deploy</param>
        [ConsoleCommand("deploy", Category = "Contract Commands")]
        private void OnDeployCommand(string filePath, string? manifestPath = null, JObject? data = null)
        {
            if (NoWallet()) return;
            byte[] script = LoadDeploymentScript(filePath, manifestPath, data, out var nef, out var manifest);
            Transaction tx;
            try
            {
                tx = CurrentWallet!.MakeTransaction(NeoSystem.StoreView, script);
            }
            catch (InvalidOperationException e)
            {
                ConsoleHelper.Error(GetExceptionMessage(e));
                return;
            }
            UInt160 hash = SmartContract.Helper.GetContractHash(tx.Sender, nef.CheckSum, manifest.Name);

            ConsoleHelper.Info("Contract hash: ", $"{hash}");
            ConsoleHelper.Info("EpicPulse consumed: ", $"{new BigDecimal((BigInteger)tx.SystemFee, NativeContract.GAS.Decimals)} EpicPulse");
            ConsoleHelper.Info("Network fee: ", $"{new BigDecimal((BigInteger)tx.NetworkFee, NativeContract.GAS.Decimals)} EpicPulse");
            ConsoleHelper.Info("Total fee: ", $"{new BigDecimal((BigInteger)(tx.SystemFee + tx.NetworkFee), NativeContract.GAS.Decimals)} EpicPulse");
            if (!ConsoleHelper.ReadUserInput("Relay Transaction? (no|yes)").IsYes()) // Add this in case just want to get hash but not relay
            {
                return;
            }
            SignAndSendTx(NeoSystem.StoreView, tx);
        }

        /// <summary>
        /// Process "update" command
        /// </summary>
        /// <param name="scriptHash">Script hash</param>
        /// <param name="filePath">File path</param>
        /// <param name="manifestPath">Manifest path</param>
        /// <param name="sender">Sender</param>
        /// <param name="signerAccounts">Signer Accounts</param>
        /// <param name="data">Extra data for update</param>
        [ConsoleCommand("update", Category = "Contract Commands")]
        private void OnUpdateCommand(UInt160 scriptHash, string filePath, string manifestPath, UInt160 sender, UInt160[]? signerAccounts = null, JObject? data = null)
        {
            Signer[] signers = Array.Empty<Signer>();

            if (NoWallet()) return;
            if (sender != null)
            {
                if (signerAccounts == null)
                    signerAccounts = new[] { sender };
                else if (signerAccounts.Contains(sender) && signerAccounts[0] != sender)
                {
                    var signersList = signerAccounts.ToList();
                    signersList.Remove(sender);
                    signerAccounts = signersList.Prepend(sender).ToArray();
                }
                else if (!signerAccounts.Contains(sender))
                {
                    signerAccounts = signerAccounts.Prepend(sender).ToArray();
                }
                signers = signerAccounts.Select(p => new Signer() { Account = p, Scopes = WitnessScope.CalledByEntry }).ToArray();
            }

            Transaction tx;
            try
            {
                byte[] script = LoadUpdateScript(scriptHash, filePath, manifestPath, data, out var nef, out var manifest);
                tx = CurrentWallet!.MakeTransaction(NeoSystem.StoreView, script, sender, signers);
            }
            catch (InvalidOperationException e)
            {
                ConsoleHelper.Error(GetExceptionMessage(e));
                return;
            }
            ContractState contract = NativeContract.ContractManagement.GetContract(NeoSystem.StoreView, scriptHash);
            if (contract == null)
            {
                ConsoleHelper.Warning($"Can't upgrade, contract hash not exist: {scriptHash}");
            }
            else
            {
                ConsoleHelper.Info("Contract hash: ", $"{scriptHash}");
                ConsoleHelper.Info("Updated times: ", $"{contract.UpdateCounter}");
                ConsoleHelper.Info("EpicPulse consumed: ", $"{new BigDecimal((BigInteger)tx.SystemFee, NativeContract.GAS.Decimals)} EpicPulse");
                ConsoleHelper.Info("Network fee: ", $"{new BigDecimal((BigInteger)tx.NetworkFee, NativeContract.GAS.Decimals)} EpicPulse");
                ConsoleHelper.Info("Total fee: ", $"{new BigDecimal((BigInteger)(tx.SystemFee + tx.NetworkFee), NativeContract.GAS.Decimals)} EpicPulse");
                if (!ConsoleHelper.ReadUserInput("Relay Transaction? (no|yes)").IsYes()) // Add this in case just want to get hash but not relay
                {
                    return;
                }
                SignAndSendTx(NeoSystem.StoreView, tx);
            }
        }

        /// <summary>
        /// Process "invoke" command
        /// </summary>
        /// <param name="scriptHash">Script hash</param>
        /// <param name="operation">Operation</param>
        /// <param name="contractParameters">Contract parameters</param>
        /// <param name="sender">Transaction's sender</param>
        /// <param name="signerAccounts">Signer's accounts</param>
        /// <param name="maxGas">Max fee for running the script, in the unit of GAS</param>
        [ConsoleCommand("invoke", Category = "Contract Commands")]
        private void OnInvokeCommand(UInt160 scriptHash, string operation, JArray? contractParameters = null, UInt160? sender = null, UInt160[]? signerAccounts = null, decimal maxGas = 20)
        {
            // In the unit of datoshi, 1 datoshi = 1e-8 GAS
            var datoshi = new BigDecimal(maxGas, NativeContract.GAS.Decimals);
            Signer[] signers = Array.Empty<Signer>();
            if (!NoWallet())
            {
                if (sender == null)
                    sender = CurrentWallet!.GetDefaultAccount()?.ScriptHash;

                if (sender != null)
                {
                    if (signerAccounts == null)
                        signerAccounts = new UInt160[1] { sender };
                    else if (signerAccounts.Contains(sender) && signerAccounts[0] != sender)
                    {
                        var signersList = signerAccounts.ToList();
                        signersList.Remove(sender);
                        signerAccounts = signersList.Prepend(sender).ToArray();
                    }
                    else if (!signerAccounts.Contains(sender))
                    {
                        signerAccounts = signerAccounts.Prepend(sender).ToArray();
                    }
                    signers = signerAccounts.Select(p => new Signer() { Account = p, Scopes = WitnessScope.CalledByEntry }).ToArray();
                }
            }

            Transaction tx = new Transaction
            {
                Signers = signers,
                Attributes = Array.Empty<TransactionAttribute>(),
                Witnesses = Array.Empty<Witness>(),
            };

            if (!OnInvokeWithResult(scriptHash, operation, out _, tx, contractParameters, datoshi: (long)datoshi.Value)) return;

            if (NoWallet()) return;
            try
            {
                tx = CurrentWallet!.MakeTransaction(NeoSystem.StoreView, tx.Script, sender, signers, maxGas: (long)datoshi.Value);
            }
            catch (InvalidOperationException e)
            {
                ConsoleHelper.Error(GetExceptionMessage(e));
                return;
            }
            ConsoleHelper.Info("Network fee: ",
                $"{new BigDecimal((BigInteger)tx.NetworkFee, NativeContract.GAS.Decimals)} EpicPulse\t",
                "Total fee: ",
                $"{new BigDecimal((BigInteger)(tx.SystemFee + tx.NetworkFee), NativeContract.GAS.Decimals)} EpicPulse");
            if (!ConsoleHelper.ReadUserInput("Relay Transaction? (no|yes)").IsYes())
            {
                return;
            }
            SignAndSendTx(NeoSystem.StoreView, tx);
        }
    }
}
