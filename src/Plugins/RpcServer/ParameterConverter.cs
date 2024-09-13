// Copyright (C) 2021-2024 EpicChain Labs.

//
// ParameterConverter.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Json;
using EpicChain.Plugins.RpcServer.Model;
using EpicChain.Wallets;
using System;
using System.Collections.Generic;
using JToken = EpicChain.Json.JToken;

namespace EpicChain.Plugins.RpcServer;

public static class ParameterConverter
{
    private static readonly Dictionary<Type, Func<JToken, object>> s_conversionStrategies;

    static ParameterConverter()
    {
        s_conversionStrategies = new Dictionary<Type, Func<JToken, object>>
        {
            { typeof(string), token => Result.Ok_Or(token.AsString, CreateInvalidParamError<string>(token)) },
            { typeof(byte), ConvertNumeric<byte> },
            { typeof(sbyte), ConvertNumeric<sbyte> },
            { typeof(short), ConvertNumeric<short> },
            { typeof(ushort), ConvertNumeric<ushort> },
            { typeof(int), ConvertNumeric<int> },
            { typeof(uint), ConvertNumeric<uint> },
            { typeof(long), ConvertNumeric<long> },
            { typeof(ulong), ConvertNumeric<ulong> },
            { typeof(double), token => Result.Ok_Or(token.AsNumber, CreateInvalidParamError<double>(token)) },
            { typeof(bool), token => Result.Ok_Or(token.AsBoolean, CreateInvalidParamError<bool>(token)) },
            { typeof(UInt256), ConvertUInt256 },
            { typeof(ContractNameOrHashOrId), ConvertContractNameOrHashOrId },
            { typeof(BlockHashOrIndex), ConvertBlockHashOrIndex }
        };
    }

    internal static object ConvertParameter(JToken token, Type targetType)
    {
        if (s_conversionStrategies.TryGetValue(targetType, out var conversionStrategy))
            return conversionStrategy(token);
        throw new RpcException(RpcError.InvalidParams.WithData($"Unsupported parameter type: {targetType}"));
    }

    private static object ConvertNumeric<T>(JToken token) where T : struct
    {
        if (TryConvertDoubleToNumericType<T>(token, out var result))
        {
            return result;
        }

        throw new RpcException(CreateInvalidParamError<T>(token));
    }

    private static bool TryConvertDoubleToNumericType<T>(JToken token, out T result) where T : struct
    {
        result = default;
        try
        {
            var value = token.AsNumber();
            var minValue = Convert.ToDouble(typeof(T).GetField("MinValue").GetValue(null));
            var maxValue = Convert.ToDouble(typeof(T).GetField("MaxValue").GetValue(null));

            if (value < minValue || value > maxValue)
            {
                return false;
            }

            if (!typeof(T).IsFloatingPoint() && !IsValidInteger(value))
            {
                return false;
            }

            result = (T)Convert.ChangeType(value, typeof(T));
            return true;
        }
        catch
        {
            return false;
        }
    }

    private static bool IsValidInteger(double value)
    {
        // Integer values are safe if they are within the range of MIN_SAFE_INTEGER and MAX_SAFE_INTEGER
        if (value < JNumber.MIN_SAFE_INTEGER || value > JNumber.MAX_SAFE_INTEGER)
            return false;
        return Math.Abs(value % 1) <= double.Epsilon;
    }

    internal static object ConvertUInt160(JToken token, byte addressVersion)
    {
        var value = token.AsString();
        if (UInt160.TryParse(value, out var scriptHash))
        {
            return scriptHash;
        }
        return Result.Ok_Or(() => value.ToScriptHash(addressVersion),
            RpcError.InvalidParams.WithData($"Invalid UInt160 Format: {token}"));
    }

    private static object ConvertUInt256(JToken token)
    {
        if (UInt256.TryParse(token.AsString(), out var hash))
        {
            return hash;
        }
        throw new RpcException(RpcError.InvalidParams.WithData($"Invalid UInt256 Format: {token}"));
    }

    private static object ConvertContractNameOrHashOrId(JToken token)
    {
        if (ContractNameOrHashOrId.TryParse(token.AsString(), out var contractNameOrHashOrId))
        {
            return contractNameOrHashOrId;
        }
        throw new RpcException(RpcError.InvalidParams.WithData($"Invalid contract hash or id Format: {token}"));
    }

    private static object ConvertBlockHashOrIndex(JToken token)
    {
        if (BlockHashOrIndex.TryParse(token.AsString(), out var blockHashOrIndex))
        {
            return blockHashOrIndex;
        }
        throw new RpcException(RpcError.InvalidParams.WithData($"Invalid block hash or index Format: {token}"));
    }

    private static RpcError CreateInvalidParamError<T>(JToken token)
    {
        return RpcError.InvalidParams.WithData($"Invalid {typeof(T)} value: {token}");
    }
}

public static class TypeExtensions
{
    public static bool IsFloatingPoint(this Type type)
    {
        return type == typeof(float) || type == typeof(double) || type == typeof(decimal);
    }
}
