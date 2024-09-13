// Copyright (C) 2021-2024 EpicChain Labs.

//
// Xep17NativeContractExtensions.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using FluentAssertions;
using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using System.IO;
using System.Numerics;

namespace EpicChain.UnitTests.Extensions
{
    public static class Xep17NativeContractExtensions
    {
        internal class ManualWitness : IVerifiable
        {
            private readonly UInt160[] _hashForVerify;

            public int Size => 0;

            public Witness[] Witnesses { get; set; }

            public ManualWitness(params UInt160[] hashForVerify)
            {
                _hashForVerify = hashForVerify ?? System.Array.Empty<UInt160>();
            }

            public void Deserialize(ref MemoryReader reader) { }

            public void DeserializeUnsigned(ref MemoryReader reader) { }

            public UInt160[] GetScriptHashesForVerifying(DataCache snapshot) => _hashForVerify;

            public void Serialize(BinaryWriter writer) { }

            public void SerializeUnsigned(BinaryWriter writer) { }
        }

        public static bool Transfer(this NativeContract contract, DataCache snapshot, byte[] from, byte[] to, BigInteger amount, bool signFrom, Block persistingBlock)
        {
            using var engine = ApplicationEngine.Create(TriggerType.Application,
                new ManualWitness(signFrom ? new UInt160(from) : null), snapshot, persistingBlock, settings: TestBlockchain.TheEpicChainSystem.Settings);

            using var script = new ScriptBuilder();
            script.EmitDynamicCall(contract.Hash, "transfer", from, to, amount, null);
            engine.LoadScript(script.ToArray());

            if (engine.Execute() == VMState.FAULT)
            {
                throw engine.FaultException;
            }

            var result = engine.ResultStack.Pop();
            result.Should().BeOfType(typeof(VM.Types.Boolean));

            return result.GetBoolean();
        }

        public static BigInteger TotalSupply(this NativeContract contract, DataCache snapshot)
        {
            using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshot, settings: TestBlockchain.TheEpicChainSystem.Settings);

            using var script = new ScriptBuilder();
            script.EmitDynamicCall(contract.Hash, "totalSupply");
            engine.LoadScript(script.ToArray());

            engine.Execute().Should().Be(VMState.HALT);

            var result = engine.ResultStack.Pop();
            result.Should().BeOfType(typeof(VM.Types.Integer));

            return result.GetInteger();
        }

        public static BigInteger BalanceOf(this NativeContract contract, DataCache snapshot, byte[] account)
        {
            using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshot, settings: TestBlockchain.TheEpicChainSystem.Settings);

            using var script = new ScriptBuilder();
            script.EmitDynamicCall(contract.Hash, "balanceOf", account);
            engine.LoadScript(script.ToArray());

            engine.Execute().Should().Be(VMState.HALT);

            var result = engine.ResultStack.Pop();
            result.Should().BeOfType(typeof(VM.Types.Integer));

            return result.GetInteger();
        }

        public static BigInteger Decimals(this NativeContract contract, DataCache snapshot)
        {
            using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshot, settings: TestBlockchain.TheEpicChainSystem.Settings);

            using var script = new ScriptBuilder();
            script.EmitDynamicCall(contract.Hash, "decimals");
            engine.LoadScript(script.ToArray());

            engine.Execute().Should().Be(VMState.HALT);

            var result = engine.ResultStack.Pop();
            result.Should().BeOfType(typeof(VM.Types.Integer));

            return result.GetInteger();
        }

        public static string Symbol(this NativeContract contract, DataCache snapshot)
        {
            using var engine = ApplicationEngine.Create(TriggerType.Application, null, snapshot, settings: TestBlockchain.TheEpicChainSystem.Settings);

            using var script = new ScriptBuilder();
            script.EmitDynamicCall(contract.Hash, "symbol");
            engine.LoadScript(script.ToArray());

            engine.Execute().Should().Be(VMState.HALT);

            var result = engine.ResultStack.Pop();
            result.Should().BeOfType(typeof(VM.Types.ByteString));

            return result.GetString();
        }
    }
}
