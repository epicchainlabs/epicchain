// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// Buffer.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System;
using System.Buffers;
using System.Collections.Generic;
using System.Diagnostics;
using System.Numerics;

namespace Neo.VM.Types
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
