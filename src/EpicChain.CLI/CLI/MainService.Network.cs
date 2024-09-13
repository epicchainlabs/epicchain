// Copyright (C) 2021-2024 EpicChain Labs.

//
// MainService.Network.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.ConsoleService;
using EpicChain.Extensions;
using EpicChain.IO;
using EpicChain.Json;
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Capabilities;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using System;
using System.Net;

namespace EpicChain.CLI
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
            OnBroadcastCommand(MessageCommand.Block, NativeContract.Ledger.GetBlock(EpicChainSystem.StoreView, hash));
        }

        /// <summary>
        /// Process "broadcast block" command
        /// </summary>
        /// <param name="height">Block index</param>
        [ConsoleCommand("broadcast block", Category = "Network Commands")]
        private void OnBroadcastGetBlocksByHeightCommand(uint height)
        {
            OnBroadcastCommand(MessageCommand.Block, NativeContract.Ledger.GetBlock(EpicChainSystem.StoreView, height));
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
            if (EpicChainSystem.MemPool.TryGetValue(hash, out var tx))
                OnBroadcastCommand(MessageCommand.Transaction, tx);
        }

        private void OnBroadcastCommand(MessageCommand command, ISerializable ret)
        {
            EpicChainSystem.LocalNode.Tell(Message.Create(command, ret));
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
                ContractParametersContext context = ContractParametersContext.Parse(jsonObjectToRelay.ToString(), EpicChainSystem.StoreView);
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
                EpicChainSystem.Blockchain.Tell(tx);
                Console.WriteLine($"Data relay success, the hash is shown as follows: {Environment.NewLine}{tx.Hash}");
            }
            catch (Exception e)
            {
                ConsoleHelper.Error(GetExceptionMessage(e));
            }
        }
    }
}
