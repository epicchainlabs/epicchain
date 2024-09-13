// Copyright (C) 2021-2024 EpicChain Labs.

//
// UT_Message.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.IO;
using FluentAssertions;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.IO;
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Payloads;
using EpicChain.VM;
using System;
using System.Linq;

namespace EpicChain.UnitTests.Network.P2P
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
