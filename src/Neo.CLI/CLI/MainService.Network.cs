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
// MainService.Network.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

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
