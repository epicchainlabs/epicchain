// Copyright (C) 2021-2024 EpicChain Labs.

//
// Benchmarks.POC.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using BenchmarkDotNet.Attributes;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.VM;
using System.Diagnostics;

namespace EpicChain.Benchmark;

public class Benchmarks_PoCs
{
    private static readonly ProtocolSettings protocol = ProtocolSettings.Load("config.json");
    private static readonly EpicChainSystem system = new(protocol, (string)null);

    [Benchmark]
    public void EpicChainIssue2725()
    {
        // https://github.com/epicchainlabs/epicchain/issues/2725
        // L00: INITSSLOT 1
        // L01: NEWARRAY0
        // L02: PUSHDATA1 6161616161 //"aaaaa"
        // L03: PUSHINT16 500
        // L04: STSFLD0
        // L05: OVER
        // L06: OVER
        // L07: SYSCALL 95016f61 //System.Runtime.Notify
        // L08: LDSFLD0
        // L09: DEC
        // L10: DUP
        // L11: STSFLD0
        // L12: JMPIF L05
        // L13: CLEAR
        // L14: SYSCALL dbfea874 //System.Runtime.GetExecutingScriptHash
        // L15: PUSHINT16 8000
        // L16: STSFLD0
        // L17: DUP
        // L18: SYSCALL 274335f1 //System.Runtime.GetNotifications
        // L19: DROP
        // L20: LDSFLD0
        // L21: DEC
        // L22: DUP
        // L23: STSFLD0
        // L24: JMPIF L17
        Run(nameof(EpicChainIssue2725), "VgHCDAVhYWFhYQH0AWBLS0GVAW9hWJ1KYCT1SUHb/qh0AUAfYEpBJ0M18UVYnUpgJPU=");
    }

    private static void Run(string name, string poc)
    {
        Random random = new();
        Transaction tx = new()
        {
            Version = 0,
            Nonce = (uint)random.Next(),
            SystemFee = 20_00000000,
            NetworkFee = 1_00000000,
            ValidUntilBlock = ProtocolSettings.Default.MaxTraceableBlocks,
            Signers = Array.Empty<Signer>(),
            Attributes = Array.Empty<TransactionAttribute>(),
            Script = Convert.FromBase64String(poc),
            Witnesses = Array.Empty<Witness>()
        };
        using var snapshot = system.GetSnapshotCache();
        using var engine = ApplicationEngine.Create(TriggerType.Application, tx, snapshot, system.GenesisBlock, protocol, tx.SystemFee);
        engine.LoadScript(tx.Script);
        engine.Execute();
        Debug.Assert(engine.State == VMState.FAULT);
    }
}
