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
using Neo.Cryptography.ECC;
using Neo.Json;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using Neo.VM;
using Neo.VM.Types;
using Neo.Wallets;
using System;
using System.Numerics;

namespace Neo.CLI
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
            var testGas = NativeContract.NEO.GetRegisterPrice(NeoSystem.StoreView) + (BigInteger)Math.Pow(10, NativeContract.GAS.Decimals) * 10;
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
                scriptBuilder.EmitDynamicCall(NativeContract.NEO.Hash, "registerCandidate", publicKey);
                script = scriptBuilder.ToArray();
            }

            SendTransaction(script, account, (long)testGas);
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
                scriptBuilder.EmitDynamicCall(NativeContract.NEO.Hash, "unregisterCandidate", publicKey);
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
                scriptBuilder.EmitDynamicCall(NativeContract.NEO.Hash, "vote", senderAccount, publicKey);
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
                scriptBuilder.EmitDynamicCall(NativeContract.NEO.Hash, "vote", senderAccount, null);
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
            if (!OnInvokeWithResult(NativeContract.NEO.Hash, "getCandidates", out StackItem result, null, null, false)) return;

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
            if (!OnInvokeWithResult(NativeContract.NEO.Hash, "getCommittee", out StackItem result, null, null, false)) return;

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
            if (!OnInvokeWithResult(NativeContract.NEO.Hash, "getNextBlockValidators", out StackItem result, null, null, false)) return;

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

            if (!OnInvokeWithResult(NativeContract.NEO.Hash, "getAccountState", out StackItem result, null, new JArray(arg))) return;
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
            ConsoleHelper.Info("Voted: ", Contract.CreateSignatureRedeemScript(publickey).ToScriptHash().ToAddress(NeoSystem.Settings.AddressVersion));
            ConsoleHelper.Info("Amount: ", new BigDecimal(((Integer)resJArray[0]).GetInteger(), NativeContract.NEO.Decimals).ToString());
            ConsoleHelper.Info("Block: ", ((Integer)resJArray[1]).GetInteger().ToString());
        }
    }
}
