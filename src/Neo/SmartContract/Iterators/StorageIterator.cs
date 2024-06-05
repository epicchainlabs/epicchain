// Copyright (C) 2021-2024 The EpicChain Labs.
//
// StorageIterator.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Neo.VM;
using Neo.VM.Types;
using System;
using System.Collections.Generic;

namespace Neo.SmartContract.Iterators
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
