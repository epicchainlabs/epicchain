// Copyright (C) 2021-2024 EpicChain Labs.

//
// Buffer.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System;
using System.Buffers;
using System.Collections.Generic;
using System.Diagnostics;
using System.Numerics;

namespace EpicChain.VM.Types
{
    /// <summary>
    /// Represents a memory block that can be used for reading and writing in the VM.
    /// </summary>
    [DebuggerDisplay("Type={GetType().Name}, Value={System.Convert.ToHexString(GetSpan())}")]
    public class Buffer : StackItem
    {
        /// <summary>
        /// The internal byte array used to store the actual data.
        /// </summary>
        public readonly Memory<byte> InnerBuffer;

        /// <summary>
        /// The size of the buffer.
        /// </summary>
        public int Size => InnerBuffer.Length;
        public override StackItemType Type => StackItemType.Buffer;

        private readonly byte[] _buffer;
        private bool _keep_alive = false;

        /// <summary>
        /// Create a buffer of the specified size.
        /// </summary>
        /// <param name="size">The size of this buffer.</param>
        /// <param name="zeroInitialize">Indicates whether the created buffer is zero-initialized.</param>
        public Buffer(int size, bool zeroInitialize = true)
        {
            _buffer = ArrayPool<byte>.Shared.Rent(size);
            InnerBuffer = new Memory<byte>(_buffer, 0, size);
            if (zeroInitialize) InnerBuffer.Span.Clear();
        }

        /// <summary>
        /// Create a buffer with the specified data.
        /// </summary>
        /// <param name="data">The data to be contained in this buffer.</param>
        public Buffer(ReadOnlySpan<byte> data) : this(data.Length, false)
        {
            data.CopyTo(InnerBuffer.Span);
        }

        internal override void Cleanup()
        {
            if (!_keep_alive)
                ArrayPool<byte>.Shared.Return(_buffer, clearArray: false);
        }

        public void KeepAlive()
        {
            _keep_alive = true;
        }

        public override StackItem ConvertTo(StackItemType type)
        {
            switch (type)
            {
                case StackItemType.Integer:
                    if (InnerBuffer.Length > Integer.MaxSize)
                        throw new InvalidCastException();
                    return new BigInteger(InnerBuffer.Span);
                case StackItemType.ByteString:
#if NET5_0_OR_GREATER
                    byte[] clone = GC.AllocateUninitializedArray<byte>(InnerBuffer.Length);
#else
                    byte[] clone = new byte[InnerBuffer.Length];
#endif
                    InnerBuffer.CopyTo(clone);
                    return clone;
                default:
                    return base.ConvertTo(type);
            }
        }

        internal override StackItem DeepCopy(Dictionary<StackItem, StackItem> refMap, bool asImmutable)
        {
            if (refMap.TryGetValue(this, out StackItem? mappedItem)) return mappedItem;
            StackItem result = asImmutable ? new ByteString(InnerBuffer.ToArray()) : new Buffer(InnerBuffer.Span);
            refMap.Add(this, result);
            return result;
        }

        public override bool GetBoolean()
        {
            return true;
        }

        public override ReadOnlySpan<byte> GetSpan()
        {
            return InnerBuffer.Span;
        }

        public override string ToString()
        {
            return GetSpan().TryGetString(out var str) ? $"(\"{str}\")" : $"(\"Base64: {Convert.ToBase64String(GetSpan())}\")";
        }
    }
}
