// =============================================================================================
//  © Copyright (C) 2021-2025 EpicChain Labs. All rights reserved.
// =============================================================================================
//
//  File: MainService.Node.cs
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
