// =============================================================================================
//  © Copyright (C) 2021-2025 EpicChain Labs. All rights reserved.
// =============================================================================================
//
//  File: MainService.Network.cs
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
