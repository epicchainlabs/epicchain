// Copyright (C) 2021-2024 EpicChain Labs.

//
// StorageIterator.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.Collections.Generic;

namespace EpicChain.SmartContract.Iterators
{
    internal class StorageIterator : IIterator
    {
        private readonly IEnumerator<(StorageKey Key, StorageItem Value)> enumerator;
        private readonly int prefixLength;
        private readonly FindOptions options;

        public StorageIterator(IEnumerator<(StorageKey, StorageItem)> enumerator, int prefixLength, FindOptions options)
        {
            this.enumerator = enumerator;
            this.prefixLength = prefixLength;
            this.options = options;
        }

        public void Dispose()
        {
            enumerator.Dispose();
        }

        public bool Next()
        {
            return enumerator.MoveNext();
        }

        public StackItem Value(ReferenceCounter referenceCounter)
        {
            ReadOnlyMemory<byte> key = enumerator.Current.Key.Key;
            ReadOnlyMemory<byte> value = enumerator.Current.Value.Value;

            if (options.HasFlag(FindOptions.RemovePrefix))
                key = key[prefixLength..];

            StackItem item = options.HasFlag(FindOptions.DeserializeValues)
                ? BinarySerializer.Deserialize(value, ExecutionEngineLimits.Default, referenceCounter)
                : value;

            if (options.HasFlag(FindOptions.PickField0))
                item = ((VM.Types.Array)item)[0];
            else if (options.HasFlag(FindOptions.PickField1))
                item = ((VM.Types.Array)item)[1];

            if (options.HasFlag(FindOptions.KeysOnly))
                return key;
            if (options.HasFlag(FindOptions.ValuesOnly))
                return item;
            return new Struct(referenceCounter) { key, item };
        }
    }
}
