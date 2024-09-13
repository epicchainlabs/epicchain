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
using System.Diagnostics;

namespace EpicChain.VM.Benchmark
{
    public class Benchmarks_PoCs
    {
        [Benchmark]
        public void EpicChainIssue2528()
        {
            // https://github.com/epicchainlabs/epicchain/issues/2528
            // L01: INITSLOT 1, 0
            // L02: NEWARRAY0
            // L03: DUP
            // L04: DUP
            // L05: PUSHINT16 2043
            // L06: STLOC 0
            // L07: PUSH1
            // L08: PACK
            // L09: LDLOC 0
            // L10: DEC
            // L11: STLOC 0
            // L12: LDLOC 0
            // L13: JMPIF_L L07
            // L14: PUSH1
            // L15: PACK
            // L16: APPEND
            // L17: PUSHINT32 38000
            // L18: STLOC 0
            // L19: PUSH0
            // L20: PICKITEM
            // L21: LDLOC 0
            // L22: DEC
            // L23: STLOC 0
            // L24: LDLOC 0
            // L25: JMPIF_L L19
            // L26: DROP
            Run(nameof(EpicChainIssue2528), "VwEAwkpKAfsHdwARwG8AnXcAbwAl9////xHAzwJwlAAAdwAQzm8AnXcAbwAl9////0U=");
        }

        [Benchmark]
        public void EpicChainVMIssue418()
        {
            // https://github.com/epicchainlabs/epiccha-vm/issues/418
            // L00: NEWARRAY0
            // L01: PUSH0
            // L02: PICK
            // L03: PUSH1
            // L04: PACK
            // L05: PUSH1
            // L06: PICK
            // L07: PUSH1
            // L08: PACK
            // L09: INITSSLOT 1
            // L10: PUSHINT16 510
            // L11: DEC
            // L12: STSFLD0
            // L13: PUSH1
            // L14: PICK
            // L15: PUSH1
            // L16: PICK
            // L17: PUSH2
            // L18: PACK
            // L19: REVERSE3
            // L20: PUSH2
            // L21: PACK
            // L22: LDSFLD0
            // L23: DUP
            // L24: JMPIF L11
            // L25: DROP
            // L26: ROT
            // L27: DROP
            Run(nameof(EpicChainVMIssue418), "whBNEcARTRHAVgEB/gGdYBFNEU0SwFMSwFhKJPNFUUU=");
        }

        [Benchmark]
        public void EpicChainIssue2723()
        {
            // L00: INITSSLOT 1
            // L01: PUSHINT32 130000
            // L02: STSFLD 0
            // L03: PUSHINT32 1048576
            // L04: NEWBUFFER
            // L05: DROP
            // L06: LDSFLD 0
            // L07: DEC
            // L08: DUP
            // L09: STSFLD 0
            // L10: JMPIF L03
            Run(nameof(EpicChainIssue2723), "VgEC0PsBAGcAAgAAEACIRV8AnUpnACTz");
        }

        private static void Run(string name, string poc)
        {
            byte[] script = Convert.FromBase64String(poc);
            using ExecutionEngine engine = new();
            engine.LoadScript(script);
            engine.Execute();
            Debug.Assert(engine.State == VMState.HALT);
        }
    }
}
