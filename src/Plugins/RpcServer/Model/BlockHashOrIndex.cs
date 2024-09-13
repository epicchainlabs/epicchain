// Copyright (C) 2021-2024 EpicChain Labs.

//
// BlockHashOrIndex.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Diagnostics.CodeAnalysis;

namespace EpicChain.Plugins.RpcServer.Model;

public class BlockHashOrIndex
{
    private readonly object _value;

    public BlockHashOrIndex(uint index)
    {
        _value = index;
    }

    public BlockHashOrIndex(UInt256 hash)
    {
        _value = hash;
    }

    public bool IsIndex => _value is uint;

    public static bool TryParse(string value, [NotNullWhen(true)] out BlockHashOrIndex blockHashOrIndex)
    {
        if (uint.TryParse(value, out var index))
        {
            blockHashOrIndex = new BlockHashOrIndex(index);
            return true;
        }
        if (UInt256.TryParse(value, out var hash))
        {
            blockHashOrIndex = new BlockHashOrIndex(hash);
            return true;
        }
        blockHashOrIndex = null;
        return false;
    }

    public uint AsIndex()
    {
        if (_value is uint intValue)
            return intValue;
        throw new RpcException(RpcError.InvalidParams.WithData($"Value {_value} is not a valid block index"));
    }

    public UInt256 AsHash()
    {
        if (_value is UInt256 hash)
            return hash;
        throw new RpcException(RpcError.InvalidParams.WithData($"Value {_value} is not a valid block hash"));
    }
}
