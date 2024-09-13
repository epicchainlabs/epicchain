// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractMethodMetadata.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography.ECC;
using EpicChain.IO;
using EpicChain.Persistence;
using EpicChain.SmartContract.Manifest;
using EpicChain.VM.Types;
using System;
using System.Diagnostics;
using System.Linq;
using System.Numerics;
using System.Reflection;
using Array = EpicChain.VM.Types.Array;

namespace EpicChain.SmartContract.Native
{
    [DebuggerDisplay("{Name}")]
    internal class ContractMethodMetadata : IHardforkActivable
    {
        public string Name { get; }
        public MethodInfo Handler { get; }
        public InteropParameterDescriptor[] Parameters { get; }
        public bool NeedApplicationEngine { get; }
        public bool NeedSnapshot { get; }
        public long CpuFee { get; }
        public long StorageFee { get; }
        public CallFlags RequiredCallFlags { get; }
        public ContractMethodDescriptor Descriptor { get; }
        public Hardfork? ActiveIn { get; init; } = null;
        public Hardfork? DeprecatedIn { get; init; } = null;

        public ContractMethodMetadata(MemberInfo member, ContractMethodAttribute attribute)
        {
            Name = attribute.Name ?? member.Name.ToLower()[0] + member.Name[1..];
            Handler = member switch
            {
                MethodInfo m => m,
                PropertyInfo p => p.GetMethod,
                _ => throw new ArgumentException(null, nameof(member))
            };
            ParameterInfo[] parameterInfos = Handler.GetParameters();
            if (parameterInfos.Length > 0)
            {
                NeedApplicationEngine = parameterInfos[0].ParameterType.IsAssignableFrom(typeof(ApplicationEngine));
                NeedSnapshot = parameterInfos[0].ParameterType.IsAssignableFrom(typeof(DataCache));
            }
            if (NeedApplicationEngine || NeedSnapshot)
                Parameters = parameterInfos.Skip(1).Select(p => new InteropParameterDescriptor(p)).ToArray();
            else
                Parameters = parameterInfos.Select(p => new InteropParameterDescriptor(p)).ToArray();
            CpuFee = attribute.CpuFee;
            StorageFee = attribute.StorageFee;
            RequiredCallFlags = attribute.RequiredCallFlags;
            ActiveIn = attribute.ActiveIn;
            DeprecatedIn = attribute.DeprecatedIn;
            Descriptor = new ContractMethodDescriptor
            {
                Name = Name,
                ReturnType = ToParameterType(Handler.ReturnType),
                Parameters = Parameters.Select(p => new ContractParameterDefinition { Type = ToParameterType(p.Type), Name = p.Name }).ToArray(),
                Safe = (attribute.RequiredCallFlags & ~CallFlags.ReadOnly) == 0
            };
        }

        private static ContractParameterType ToParameterType(Type type)
        {
            if (type.BaseType == typeof(ContractTask)) return ToParameterType(type.GenericTypeArguments[0]);
            if (type == typeof(ContractTask)) return ContractParameterType.Void;
            if (type == typeof(void)) return ContractParameterType.Void;
            if (type == typeof(bool)) return ContractParameterType.Boolean;
            if (type == typeof(sbyte)) return ContractParameterType.Integer;
            if (type == typeof(byte)) return ContractParameterType.Integer;
            if (type == typeof(short)) return ContractParameterType.Integer;
            if (type == typeof(ushort)) return ContractParameterType.Integer;
            if (type == typeof(int)) return ContractParameterType.Integer;
            if (type == typeof(uint)) return ContractParameterType.Integer;
            if (type == typeof(long)) return ContractParameterType.Integer;
            if (type == typeof(ulong)) return ContractParameterType.Integer;
            if (type == typeof(BigInteger)) return ContractParameterType.Integer;
            if (type == typeof(byte[])) return ContractParameterType.ByteArray;
            if (type == typeof(string)) return ContractParameterType.String;
            if (type == typeof(UInt160)) return ContractParameterType.Hash160;
            if (type == typeof(UInt256)) return ContractParameterType.Hash256;
            if (type == typeof(ECPoint)) return ContractParameterType.PublicKey;
            if (type == typeof(VM.Types.Boolean)) return ContractParameterType.Boolean;
            if (type == typeof(Integer)) return ContractParameterType.Integer;
            if (type == typeof(ByteString)) return ContractParameterType.ByteArray;
            if (type == typeof(VM.Types.Buffer)) return ContractParameterType.ByteArray;
            if (type == typeof(Array)) return ContractParameterType.Array;
            if (type == typeof(Struct)) return ContractParameterType.Array;
            if (type == typeof(Map)) return ContractParameterType.Map;
            if (type == typeof(StackItem)) return ContractParameterType.Any;
            if (type == typeof(object)) return ContractParameterType.Any;
            if (typeof(IInteroperable).IsAssignableFrom(type)) return ContractParameterType.Array;
            if (typeof(ISerializable).IsAssignableFrom(type)) return ContractParameterType.ByteArray;
            if (type.IsArray) return ContractParameterType.Array;
            if (type.IsEnum) return ContractParameterType.Integer;
            if (type.IsValueType) return ContractParameterType.Array;
            return ContractParameterType.InteropInterface;
        }
    }
}
