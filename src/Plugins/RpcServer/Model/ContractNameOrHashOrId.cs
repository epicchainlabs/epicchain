// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractNameOrHashOrId.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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

public class ContractNameOrHashOrId
{
    private readonly object _value;

    public ContractNameOrHashOrId(int id)
    {
        _value = id;
    }

    public ContractNameOrHashOrId(UInt160 hash)
    {
        _value = hash;
    }

    public ContractNameOrHashOrId(string name)
    {
        _value = name;
    }

    public bool IsId => _value is int;
    public bool IsHash => _value is UInt160;
    public bool IsName => _value is string;

    public static bool TryParse(string value, [NotNullWhen(true)] out ContractNameOrHashOrId contractNameOrHashOrId)
    {
        if (int.TryParse(value, out var id))
        {
            contractNameOrHashOrId = new ContractNameOrHashOrId(id);
            return true;
        }
        if (UInt160.TryParse(value, out var hash))
        {
            contractNameOrHashOrId = new ContractNameOrHashOrId(hash);
            return true;
        }

        if (value.Length > 0)
        {
            contractNameOrHashOrId = new ContractNameOrHashOrId(value);
            return true;
        }
        contractNameOrHashOrId = null;
        return false;
    }

    public int AsId()
    {
        if (_value is int intValue)
            return intValue;
        throw new RpcException(RpcError.InvalidParams.WithData($"Value {_value} is not a valid contract id"));
    }

    public UInt160 AsHash()
    {
        if (_value is UInt160 hash)
            return hash;
        throw new RpcException(RpcError.InvalidParams.WithData($"Value {_value} is not a valid contract hash"));
    }

    public string AsName()
    {
        if (_value is string name)
            return name;
        throw new RpcException(RpcError.InvalidParams.WithData($"Value {_value} is not a valid contract name"));
    }
}
