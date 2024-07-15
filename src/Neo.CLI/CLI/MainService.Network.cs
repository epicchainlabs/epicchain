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

using Akka.Actor;
using Neo.ConsoleService;
using Neo.IO;
using Neo.Json;
using Neo.Network.P2P;
using Neo.Network.P2P.Capabilities;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using System;
using System.Net;

namespace Neo.CLI
{
    partial class MainService
    {
        /// <summary>
        /// Process "broadcast addr" command
        /// </summary>
        /// <param name="payload">Payload</param>
        /// <param name="port">Port</param>
        [ConsoleCommand("broadcast addr", Category = "Network Commands")]
        private void OnBroadcastAddressCommand(IPAddress payload, ushort port)
        {
            if (payload == null)
            {
                ConsoleHelper.Warning("You must input the payload to relay.");
                return;
            }

            OnBroadcastCommand(MessageCommand.Addr,
                AddrPayload.Create(
                    NetworkAddressWithTime.Create(
                        payload, DateTime.UtcNow.ToTimestamp(),
                        new FullNodeCapability(),
                        new ServerCapability(NodeCapabilityType.TcpServer, port))
                    ));
        }

        /// <summary>
        /// Process "broadcast block" command
        /// </summary>
        /// <param name="hash">Hash</param>
        [ConsoleCommand("broadcast block", Category = "Network Commands")]
        private void OnBroadcastGetBlocksByHashCommand(UInt256 hash)
        {
            OnBroadcastCommand(MessageCommand.Block, NativeContract.Ledger.GetBlock(NeoSystem.StoreView, hash));
        }

        /// <summary>
        /// Process "broadcast block" command
        /// </summary>
        /// <param name="height">Block index</param>
        [ConsoleCommand("broadcast block", Category = "Network Commands")]
        private void OnBroadcastGetBlocksByHeightCommand(uint height)
        {
            OnBroadcastCommand(MessageCommand.Block, NativeContract.Ledger.GetBlock(NeoSystem.StoreView, height));
        }

        /// <summary>
        /// Process "broadcast getblocks" command
        /// </summary>
        /// <param name="hash">Hash</param>
        [ConsoleCommand("broadcast getblocks", Category = "Network Commands")]
        private void OnBroadcastGetBlocksCommand(UInt256 hash)
        {
            OnBroadcastCommand(MessageCommand.GetBlocks, GetBlocksPayload.Create(hash));
        }

        /// <summary>
        /// Process "broadcast getheaders" command
        /// </summary>
        /// <param name="index">Index</param>
        [ConsoleCommand("broadcast getheaders", Category = "Network Commands")]
        private void OnBroadcastGetHeadersCommand(uint index)
        {
            OnBroadcastCommand(MessageCommand.GetHeaders, GetBlockByIndexPayload.Create(index));
        }

        /// <summary>
        /// Process "broadcast getdata" command
        /// </summary>
        /// <param name="type">Type</param>
        /// <param name="payload">Payload</param>
        [ConsoleCommand("broadcast getdata", Category = "Network Commands")]
        private void OnBroadcastGetDataCommand(InventoryType type, UInt256[] payload)
        {
            OnBroadcastCommand(MessageCommand.GetData, InvPayload.Create(type, payload));
        }

        /// <summary>
        /// Process "broadcast inv" command
        /// </summary>
        /// <param name="type">Type</param>
        /// <param name="payload">Payload</param>
        [ConsoleCommand("broadcast inv", Category = "Network Commands")]
        private void OnBroadcastInvCommand(InventoryType type, UInt256[] payload)
        {
            OnBroadcastCommand(MessageCommand.Inv, InvPayload.Create(type, payload));
        }

        /// <summary>
        /// Process "broadcast transaction" command
        /// </summary>
        /// <param name="hash">Hash</param>
        [ConsoleCommand("broadcast transaction", Category = "Network Commands")]
        private void OnBroadcastTransactionCommand(UInt256 hash)
        {
            if (NeoSystem.MemPool.TryGetValue(hash, out Transaction tx))
                OnBroadcastCommand(MessageCommand.Transaction, tx);
        }

        private void OnBroadcastCommand(MessageCommand command, ISerializable ret)
        {
            NeoSystem.LocalNode.Tell(Message.Create(command, ret));
        }

        /// <summary>
        /// Process "relay" command
        /// </summary>
        /// <param name="jsonObjectToRelay">Json object</param>
        [ConsoleCommand("relay", Category = "Network Commands")]
        private void OnRelayCommand(JObject jsonObjectToRelay)
        {
            if (jsonObjectToRelay == null)
            {
                ConsoleHelper.Warning("You must input JSON object to relay.");
                return;
            }

            try
            {
                ContractParametersContext context = ContractParametersContext.Parse(jsonObjectToRelay.ToString(), NeoSystem.StoreView);
                if (!context.Completed)
                {
                    ConsoleHelper.Error("The signature is incomplete.");
                    return;
                }
                if (!(context.Verifiable is Transaction tx))
                {
                    ConsoleHelper.Warning("Only support to relay transaction.");
                    return;
                }
                tx.Witnesses = context.GetWitnesses();
                NeoSystem.Blockchain.Tell(tx);
                Console.WriteLine($"Data relay success, the hash is shown as follows: {Environment.NewLine}{tx.Hash}");
            }
            catch (Exception e)
            {
                ConsoleHelper.Error(GetExceptionMessage(e));
            }
        }
    }
}
