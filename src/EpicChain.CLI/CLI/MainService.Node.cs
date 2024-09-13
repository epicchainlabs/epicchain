// Copyright (C) 2021-2024 EpicChain Labs.

//
// MainService.Node.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract.Native;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;

namespace EpicChain.CLI
{
    partial class MainService
    {
        /// <summary>
        /// Process "show pool" command
        /// </summary>
        [ConsoleCommand("show pool", Category = "Node Commands", Description = "Show the current state of the mempool")]
        private void OnShowPoolCommand(bool verbose = false)
        {
            int verifiedCount, unverifiedCount;
            if (verbose)
            {
                EpicChainSystem.MemPool.GetVerifiedAndUnverifiedTransactions(
                    out IEnumerable<Transaction> verifiedTransactions,
                    out IEnumerable<Transaction> unverifiedTransactions);
                ConsoleHelper.Info("Verified Transactions:");
                foreach (Transaction tx in verifiedTransactions)
                    Console.WriteLine($" {tx.Hash} {tx.GetType().Name} {tx.NetworkFee} EpicPulse_NetFee");
                ConsoleHelper.Info("Unverified Transactions:");
                foreach (Transaction tx in unverifiedTransactions)
                    Console.WriteLine($" {tx.Hash} {tx.GetType().Name} {tx.NetworkFee} EpicPulse_NetFee");

                verifiedCount = verifiedTransactions.Count();
                unverifiedCount = unverifiedTransactions.Count();
            }
            else
            {
                verifiedCount = EpicChainSystem.MemPool.VerifiedCount;
                unverifiedCount = EpicChainSystem.MemPool.UnVerifiedCount;
            }
            Console.WriteLine($"total: {EpicChainSystem.MemPool.Count}, verified: {verifiedCount}, unverified: {unverifiedCount}");
        }

        /// <summary>
        /// Process "show state" command
        /// </summary>
        [ConsoleCommand("show state", Category = "Node Commands", Description = "Show the current state of the node")]
        private void OnShowStateCommand()
        {
            var cancel = new CancellationTokenSource();

            Console.CursorVisible = false;
            Console.Clear();

            Task broadcast = Task.Run(async () =>
            {
                while (!cancel.Token.IsCancellationRequested)
                {
                    EpicChainSystem.LocalNode.Tell(Message.Create(MessageCommand.Ping, PingPayload.Create(NativeContract.Ledger.CurrentIndex(EpicChainSystem.StoreView))));
                    await Task.Delay(EpicChainSystem.Settings.TimePerBlock, cancel.Token);
                }
            });
            Task task = Task.Run(async () =>
            {
                int maxLines = 0;
                while (!cancel.Token.IsCancellationRequested)
                {
                    uint height = NativeContract.Ledger.CurrentIndex(EpicChainSystem.StoreView);
                    uint headerHeight = EpicChainSystem.HeaderCache.Last?.Index ?? height;

                    Console.SetCursorPosition(0, 0);
                    WriteLineWithoutFlicker($"block: {height}/{headerHeight}  connected: {LocalNode.ConnectedCount}  unconnected: {LocalNode.UnconnectedCount}", Console.WindowWidth - 1);

                    int linesWritten = 1;
                    foreach (RemoteNode node in LocalNode.GetRemoteNodes().OrderByDescending(u => u.LastBlockIndex).Take(Console.WindowHeight - 2).ToArray())
                    {
                        ConsoleHelper.Info("  ip: ",
                            $"{node.Remote.Address,-15}\t",
                            "port: ",
                            $"{node.Remote.Port,-5}\t",
                            "listen: ",
                            $"{node.ListenerTcpPort,-5}\t",
                            "height: ",
                            $"{node.LastBlockIndex,-7}");
                        linesWritten++;
                    }

                    maxLines = Math.Max(maxLines, linesWritten);

                    while (linesWritten < maxLines)
                    {
                        WriteLineWithoutFlicker("", Console.WindowWidth - 1);
                        maxLines--;
                    }

                    await Task.Delay(500, cancel.Token);
                }
            });
            ReadLine();
            cancel.Cancel();
            try { Task.WaitAll(task, broadcast); } catch { }
            Console.WriteLine();
            Console.CursorVisible = true;
        }
    }
}
