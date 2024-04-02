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
// MemoryReader.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using System;
using System.Buffers.Binary;
using System.Runtime.CompilerServices;

namespace Neo.IO
{
    public ref struct MemoryReader
    {
        private readonly ReadOnlyMemory<byte> _memory;
        private readonly ReadOnlySpan<byte> _span;
        private int _pos = 0;

        public readonly int Position => _pos;

        public MemoryReader(ReadOnlyMemory<byte> memory)
        {
            _memory = memory;
            _span = memory.Span;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        private readonly void EnsurePosition(int move)
        {
            if (_pos + move > _span.Length) throw new FormatException();
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public readonly byte Peek()
        {
            EnsurePosition(1);
            return _span[_pos];
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public bool ReadBoolean()
        {
            return ReadByte() switch
            {
                0 => false,
                1 => true,
                _ => throw new FormatException()
            };
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public sbyte ReadSByte()
        {
            EnsurePosition(1);
            var b = _span[_pos++];
            return unchecked((sbyte)b);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public byte ReadByte()
        {
            EnsurePosition(1);
            return _span[_pos++];
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public short ReadInt16()
        {
            EnsurePosition(sizeof(short));
            var result = BinaryPrimitives.ReadInt16LittleEndian(_span[_pos..]);
            _pos += sizeof(short);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public short ReadInt16BigEndian()
        {
            EnsurePosition(sizeof(short));
            var result = BinaryPrimitives.ReadInt16BigEndian(_span[_pos..]);
            _pos += sizeof(short);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public ushort ReadUInt16()
        {
            EnsurePosition(sizeof(ushort));
            var result = BinaryPrimitives.ReadUInt16LittleEndian(_span[_pos..]);
            _pos += sizeof(ushort);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public ushort ReadUInt16BigEndian()
        {
            EnsurePosition(sizeof(ushort));
            var result = BinaryPrimitives.ReadUInt16BigEndian(_span[_pos..]);
            _pos += sizeof(ushort);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int ReadInt32()
        {
            EnsurePosition(sizeof(int));
            var result = BinaryPrimitives.ReadInt32LittleEndian(_span[_pos..]);
            _pos += sizeof(int);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public int ReadInt32BigEndian()
        {
            EnsurePosition(sizeof(int));
            var result = BinaryPrimitives.ReadInt32BigEndian(_span[_pos..]);
            _pos += sizeof(int);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public uint ReadUInt32()
        {
            EnsurePosition(sizeof(uint));
            var result = BinaryPrimitives.ReadUInt32LittleEndian(_span[_pos..]);
            _pos += sizeof(uint);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public uint ReadUInt32BigEndian()
        {
            EnsurePosition(sizeof(uint));
            var result = BinaryPrimitives.ReadUInt32BigEndian(_span[_pos..]);
            _pos += sizeof(uint);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public long ReadInt64()
        {
            EnsurePosition(sizeof(long));
            var result = BinaryPrimitives.ReadInt64LittleEndian(_span[_pos..]);
            _pos += sizeof(long);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public long ReadInt64BigEndian()
        {
            EnsurePosition(sizeof(long));
            var result = BinaryPrimitives.ReadInt64BigEndian(_span[_pos..]);
            _pos += sizeof(long);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public ulong ReadUInt64()
        {
            EnsurePosition(sizeof(ulong));
            var result = BinaryPrimitives.ReadUInt64LittleEndian(_span[_pos..]);
            _pos += sizeof(ulong);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public ulong ReadUInt64BigEndian()
        {
            EnsurePosition(sizeof(ulong));
            var result = BinaryPrimitives.ReadUInt64BigEndian(_span[_pos..]);
            _pos += sizeof(ulong);
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public ulong ReadVarInt(ulong max = ulong.MaxValue)
        {
            var b = ReadByte();
            var value = b switch
            {
                0xfd => ReadUInt16(),
                0xfe => ReadUInt32(),
                0xff => ReadUInt64(),
                _ => b
            };
            if (value > max) throw new FormatException();
            return value;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public string ReadFixedString(int length)
        {
            EnsurePosition(length);
            var end = _pos + length;
            var i = _pos;
            while (i < end && _span[i] != 0) i++;
            var data = _span[_pos..i];
            for (; i < end; i++)
                if (_span[i] != 0)
                    throw new FormatException();
            _pos = end;
            return Utility.StrictUTF8.GetString(data);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public string ReadVarString(int max = 0x1000000)
        {
            var length = (int)ReadVarInt((ulong)max);
            EnsurePosition(length);
            var data = _span.Slice(_pos, length);
            _pos += length;
            return Utility.StrictUTF8.GetString(data);
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public ReadOnlyMemory<byte> ReadMemory(int count)
        {
            EnsurePosition(count);
            var result = _memory.Slice(_pos, count);
            _pos += count;
            return result;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public ReadOnlyMemory<byte> ReadVarMemory(int max = 0x1000000) =>
            ReadMemory((int)ReadVarInt((ulong)max));

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public ReadOnlyMemory<byte> ReadToEnd()
        {
            var result = _memory[_pos..];
            _pos = _memory.Length;
            return result;
        }
    }
}
