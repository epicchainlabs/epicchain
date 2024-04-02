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
// UT_Message.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Akka.IO;
using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo.IO;
using Neo.Network.P2P;
using Neo.Network.P2P.Payloads;
using Neo.VM;
using System;
using System.Linq;

namespace Neo.UnitTests.Network.P2P
{
    [TestClass]
    public class UT_Message
    {
        [TestMethod]
        public void Serialize_Deserialize()
        {
            var payload = PingPayload.Create(uint.MaxValue);
            var msg = Message.Create(MessageCommand.Ping, payload);
            var buffer = msg.ToArray();
            var copy = buffer.AsSerializable<Message>();
            var payloadCopy = (PingPayload)copy.Payload;

            copy.Command.Should().Be(msg.Command);
            copy.Flags.Should().Be(msg.Flags);
            msg.Size.Should().Be(payload.Size + 3);

            payloadCopy.LastBlockIndex.Should().Be(payload.LastBlockIndex);
            payloadCopy.Nonce.Should().Be(payload.Nonce);
            payloadCopy.Timestamp.Should().Be(payload.Timestamp);
        }

        [TestMethod]
        public void Serialize_Deserialize_WithoutPayload()
        {
            var msg = Message.Create(MessageCommand.GetAddr);
            var buffer = msg.ToArray();
            var copy = buffer.AsSerializable<Message>();

            copy.Command.Should().Be(msg.Command);
            copy.Flags.Should().Be(msg.Flags);
            copy.Payload.Should().Be(null);
        }

        [TestMethod]
        public void ToArray()
        {
            var payload = PingPayload.Create(uint.MaxValue);
            var msg = Message.Create(MessageCommand.Ping, payload);
            _ = msg.ToArray();

            msg.Size.Should().Be(payload.Size + 3);
        }

        [TestMethod]
        public void Serialize_Deserialize_ByteString()
        {
            var payload = PingPayload.Create(uint.MaxValue);
            var msg = Message.Create(MessageCommand.Ping, payload);
            var buffer = ByteString.CopyFrom(msg.ToArray());
            var length = Message.TryDeserialize(buffer, out var copy);

            var payloadCopy = (PingPayload)copy.Payload;

            copy.Command.Should().Be(msg.Command);
            copy.Flags.Should().Be(msg.Flags);

            payloadCopy.LastBlockIndex.Should().Be(payload.LastBlockIndex);
            payloadCopy.Nonce.Should().Be(payload.Nonce);
            payloadCopy.Timestamp.Should().Be(payload.Timestamp);

            buffer.Count.Should().Be(length);
        }

        [TestMethod]
        public void ToArray_WithoutPayload()
        {
            var msg = Message.Create(MessageCommand.GetAddr);
            _ = msg.ToArray();
        }

        [TestMethod]
        public void Serialize_Deserialize_WithoutPayload_ByteString()
        {
            var msg = Message.Create(MessageCommand.GetAddr);
            var buffer = ByteString.CopyFrom(msg.ToArray());
            var length = Message.TryDeserialize(buffer, out var copy);

            copy.Command.Should().Be(msg.Command);
            copy.Flags.Should().Be(msg.Flags);
            copy.Payload.Should().Be(null);

            buffer.Count.Should().Be(length);
        }

        [TestMethod]
        public void MultipleSizes()
        {
            var msg = Message.Create(MessageCommand.GetAddr);
            var buffer = msg.ToArray();

            var length = Message.TryDeserialize(ByteString.Empty, out var copy);
            Assert.AreEqual(0, length);
            Assert.IsNull(copy);

            length = Message.TryDeserialize(ByteString.CopyFrom(buffer), out copy);
            Assert.AreEqual(buffer.Length, length);
            Assert.IsNotNull(copy);

            length = Message.TryDeserialize(ByteString.CopyFrom(buffer.Take(2).Concat(new byte[] { 0xFD }).ToArray()), out copy);
            Assert.AreEqual(0, length);
            Assert.IsNull(copy);

            length = Message.TryDeserialize(ByteString.CopyFrom(buffer.Take(2).Concat(new byte[] { 0xFD, buffer[2], 0x00 }).Concat(buffer.Skip(3)).ToArray()), out copy);
            Assert.AreEqual(buffer.Length + 2, length);
            Assert.IsNotNull(copy);

            length = Message.TryDeserialize(ByteString.CopyFrom(buffer.Take(2).Concat(new byte[] { 0xFD, 0x01, 0x00 }).Concat(buffer.Skip(3)).ToArray()), out copy);
            Assert.AreEqual(0, length);
            Assert.IsNull(copy);

            length = Message.TryDeserialize(ByteString.CopyFrom(buffer.Take(2).Concat(new byte[] { 0xFE }).Concat(buffer.Skip(3)).ToArray()), out copy);
            Assert.AreEqual(0, length);
            Assert.IsNull(copy);

            length = Message.TryDeserialize(ByteString.CopyFrom(buffer.Take(2).Concat(new byte[] { 0xFE, buffer[2], 0x00, 0x00, 0x00 }).Concat(buffer.Skip(3)).ToArray()), out copy);
            Assert.AreEqual(buffer.Length + 4, length);
            Assert.IsNotNull(copy);

            length = Message.TryDeserialize(ByteString.CopyFrom(buffer.Take(2).Concat(new byte[] { 0xFF }).Concat(buffer.Skip(3)).ToArray()), out copy);
            Assert.AreEqual(0, length);
            Assert.IsNull(copy);

            length = Message.TryDeserialize(ByteString.CopyFrom(buffer.Take(2).Concat(new byte[] { 0xFF, buffer[2], 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 }).Concat(buffer.Skip(3)).ToArray()), out copy);
            Assert.AreEqual(buffer.Length + 8, length);
            Assert.IsNotNull(copy);

            // Big message

            Assert.ThrowsException<FormatException>(() => Message.TryDeserialize(ByteString.CopyFrom(buffer.Take(2).Concat(new byte[] { 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01 }).Concat(buffer.Skip(3)).ToArray()), out copy));
        }

        [TestMethod]
        public void Compression()
        {
            var payload = new Transaction()
            {
                Nonce = 1,
                Version = 0,
                Attributes = Array.Empty<TransactionAttribute>(),
                Script = new byte[] { (byte)OpCode.PUSH1 },
                Signers = new Signer[] { new Signer() { Account = UInt160.Zero } },
                Witnesses = new Witness[] { new Witness() { InvocationScript = Array.Empty<byte>(), VerificationScript = Array.Empty<byte>() } },
            };

            var msg = Message.Create(MessageCommand.Transaction, payload);
            var buffer = msg.ToArray();

            buffer.Length.Should().Be(56);

            byte[] script = new byte[100];
            Array.Fill(script, (byte)OpCode.PUSH2);
            payload.Script = script;
            msg = Message.Create(MessageCommand.Transaction, payload);
            buffer = msg.ToArray();

            buffer.Length.Should().Be(30);
            msg.Flags.HasFlag(MessageFlags.Compressed).Should().BeTrue();

            _ = Message.TryDeserialize(ByteString.CopyFrom(msg.ToArray()), out var copy);
            Assert.IsNotNull(copy);

            copy.Flags.HasFlag(MessageFlags.Compressed).Should().BeTrue();
        }
    }
}
