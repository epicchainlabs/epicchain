// Copyright (C) 2021-2024 EpicChain Labs.

//
// InteropParameterDescriptor.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.VM.Types;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Reflection;

namespace EpicChain.SmartContract
{
    /// <summary>
    /// Represents a descriptor of an interoperable service parameter.
    /// </summary>
    public class InteropParameterDescriptor
    {
        private readonly ValidatorAttribute[] _validators;

        /// <summary>
        /// The name of the parameter.
        /// </summary>
        public string Name { get; }

        /// <summary>
        /// The type of the parameter.
        /// </summary>
        public Type Type { get; }

        /// <summary>
        /// The converter to convert the parameter from <see cref="StackItem"/> to <see cref="object"/>.
        /// </summary>
        public Func<StackItem, object> Converter { get; }

        /// <summary>
        /// Indicates whether the parameter is an enumeration.
        /// </summary>
        public bool IsEnum => Type.IsEnum;

        /// <summary>
        /// Indicates whether the parameter is an array.
        /// </summary>
        public bool IsArray => Type.IsArray && Type.GetElementType() != typeof(byte);

        /// <summary>
        /// Indicates whether the parameter is an <see cref="InteropInterface"/>.
        /// </summary>
        public bool IsInterface { get; }

        private static readonly Dictionary<Type, Func<StackItem, object>> converters = new()
        {
            [typeof(StackItem)] = p => p,
            [typeof(VM.Types.Pointer)] = p => p,
            [typeof(VM.Types.Array)] = p => p,
            [typeof(InteropInterface)] = p => p,
            [typeof(bool)] = p => p.GetBoolean(),
            [typeof(sbyte)] = p => (sbyte)p.GetInteger(),
            [typeof(byte)] = p => (byte)p.GetInteger(),
            [typeof(short)] = p => (short)p.GetInteger(),
            [typeof(ushort)] = p => (ushort)p.GetInteger(),
            [typeof(int)] = p => (int)p.GetInteger(),
            [typeof(uint)] = p => (uint)p.GetInteger(),
            [typeof(long)] = p => (long)p.GetInteger(),
            [typeof(ulong)] = p => (ulong)p.GetInteger(),
            [typeof(BigInteger)] = p => p.GetInteger(),
            [typeof(byte[])] = p => p.IsNull ? null : p.GetSpan().ToArray(),
            [typeof(string)] = p => p.IsNull ? null : p.GetString(),
            [typeof(UInt160)] = p => p.IsNull ? null : new UInt160(p.GetSpan()),
            [typeof(UInt256)] = p => p.IsNull ? null : new UInt256(p.GetSpan()),
            [typeof(ECPoint)] = p => p.IsNull ? null : ECPoint.DecodePoint(p.GetSpan(), ECCurve.Secp256r1),
        };

        internal InteropParameterDescriptor(ParameterInfo parameterInfo)
            : this(parameterInfo.ParameterType, parameterInfo.GetCustomAttributes<ValidatorAttribute>(true).ToArray())
        {
            Name = parameterInfo.Name;
        }

        internal InteropParameterDescriptor(Type type, params ValidatorAttribute[] validators)
        {
            Type = type;
            _validators = validators;
            if (IsEnum)
            {
                Converter = converters[type.GetEnumUnderlyingType()];
            }
            else if (IsArray)
            {
                Converter = converters[type.GetElementType()];
            }
            else
            {
                IsInterface = !converters.TryGetValue(type, out var converter);
                if (IsInterface)
                    Converter = converters[typeof(InteropInterface)];
                else
                    Converter = converter;
            }
        }

        public void Validate(StackItem item)
        {
            foreach (var validator in _validators)
                validator.Validate(item);
        }
    }
}
