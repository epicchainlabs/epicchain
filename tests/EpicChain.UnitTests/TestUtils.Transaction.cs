// Copyright (C) 2021-2024 EpicChain Labs.

//
// TestUtils.Transaction.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Microsoft.VisualStudio.TestTools.UnitTesting;
using EpicChain.Cryptography;
using EpicChain.Extensions;
using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.Wallets;
using EpicChain.Wallets.XEP6;
using System;
using System.IO;
using System.Linq;
using System.Numerics;

namespace EpicChain.UnitTests;

public partial class TestUtils
{
    public static Transaction CreateValidTx(DataCache snapshot, XEP6Wallet wallet, WalletAccount account)
    {
        return CreateValidTx(snapshot, wallet, account.ScriptHash, (uint)new Random().Next());
    }

    public static Transaction CreateValidTx(DataCache snapshot, XEP6Wallet wallet, UInt160 account, uint nonce)
    {
        var tx = wallet.MakeTransaction(snapshot, [
                new TransferOutput
                {
                    AssetId = NativeContract.EpicPulse.Hash,
                    ScriptHash = account,
                    Value = new BigDecimal(BigInteger.One, 8)
                }
            ],
            account);

        tx.Nonce = nonce;
        tx.Signers = [new Signer { Account = account, Scopes = WitnessScope.CalledByEntry }];
        var data = new ContractParametersContext(snapshot, tx, TestProtocolSettings.Default.Network);
        Assert.IsNull(data.GetSignatures(tx.Sender));
        Assert.IsTrue(wallet.Sign(data));
        Assert.IsTrue(data.Completed);
        Assert.AreEqual(1, data.GetSignatures(tx.Sender).Count);

        tx.Witnesses = data.GetWitnesses();
        return tx;
    }

    public static Transaction CreateValidTx(DataCache snapshot, XEP6Wallet wallet, UInt160 account, uint nonce, UInt256[] conflicts)
    {
        var tx = wallet.MakeTransaction(snapshot, [
                new TransferOutput
                {
                    AssetId = NativeContract.EpicPulse.Hash,
                    ScriptHash = account,
                    Value = new BigDecimal(BigInteger.One, 8)
                }
            ],
            account);
        tx.Attributes = conflicts.Select(conflict => new Conflicts { Hash = conflict }).ToArray();
        tx.Nonce = nonce;
        tx.Signers = [new Signer { Account = account, Scopes = WitnessScope.CalledByEntry }];
        var data = new ContractParametersContext(snapshot, tx, TestProtocolSettings.Default.Network);
        Assert.IsNull(data.GetSignatures(tx.Sender));
        Assert.IsTrue(wallet.Sign(data));
        Assert.IsTrue(data.Completed);
        Assert.AreEqual(1, data.GetSignatures(tx.Sender).Count);
        tx.Witnesses = data.GetWitnesses();
        return tx;
    }

    public static Transaction CreateRandomHashTransaction()
    {
        var randomBytes = new byte[16];
        TestRandom.NextBytes(randomBytes);
        return new Transaction
        {
            Script = randomBytes,
            Attributes = [],
            Signers = [new Signer { Account = UInt160.Zero }],
            Witnesses =
            [
                new Witness
                {
                    InvocationScript = Array.Empty<byte>(),
                    VerificationScript = Array.Empty<byte>()
                }
            ]
        };
    }

    public static Transaction GetTransaction(UInt160 sender)
    {
        return new Transaction
        {
            Script = new[] { (byte)OpCode.PUSH2 },
            Attributes = [],
            Signers =
            [
                new Signer
                {
                    Account = sender,
                    Scopes = WitnessScope.CalledByEntry,
                    AllowedContracts = [],
                    AllowedGroups = [],
                    Rules = [],
                }
            ],
            Witnesses =
            [
                new Witness
                {
                    InvocationScript = Array.Empty<byte>(),
                    VerificationScript = Array.Empty<byte>()
                }
            ]
        };
    }

    public static Transaction CreateInvalidTransaction(DataCache snapshot, XEP6Wallet wallet, WalletAccount account, InvalidTransactionType type, UInt256 conflict = null)
    {
        var rand = new Random();
        var sender = account.ScriptHash;

        var tx = new Transaction
        {
            Version = 0,
            Nonce = (uint)rand.Next(),
            ValidUntilBlock = NativeContract.Ledger.CurrentIndex(snapshot) + wallet.ProtocolSettings.MaxValidUntilBlockIncrement,
            Signers = [new Signer { Account = sender, Scopes = WitnessScope.CalledByEntry }],
            Attributes = [],
            Script = new[] { (byte)OpCode.RET }
        };

        switch (type)
        {
            case InvalidTransactionType.InsufficientBalance:
                // Set an unrealistically high system fee
                tx.SystemFee = long.MaxValue;
                break;
            case InvalidTransactionType.InvalidScript:
                // Use an invalid script
                tx.Script = new byte[] { 0xFF };
                break;
            case InvalidTransactionType.InvalidAttribute:
                // Add an invalid attribute
                tx.Attributes = [new InvalidAttribute()];
                break;
            case InvalidTransactionType.Oversized:
                // Make the transaction oversized
                tx.Script = new byte[Transaction.MaxTransactionSize];
                break;
            case InvalidTransactionType.Expired:
                // Set an expired ValidUntilBlock
                tx.ValidUntilBlock = NativeContract.Ledger.CurrentIndex(snapshot) - 1;
                break;
            case InvalidTransactionType.Conflicting:
                // To create a conflicting transaction, we'd need another valid transaction.
                // For simplicity, we'll just add a Conflicts attribute with a random hash.
                tx.Attributes = [new Conflicts { Hash = conflict }];
                break;
        }

        var data = new ContractParametersContext(snapshot, tx, TestProtocolSettings.Default.Network);
        Assert.IsNull(data.GetSignatures(tx.Sender));
        Assert.IsTrue(wallet.Sign(data));
        Assert.IsTrue(data.Completed);
        Assert.AreEqual(1, data.GetSignatures(tx.Sender).Count);
        tx.Witnesses = data.GetWitnesses();
        if (type == InvalidTransactionType.InvalidSignature)
        {
            tx.Witnesses[0] = new Witness
            {
                InvocationScript = new byte[] { (byte)OpCode.PUSHDATA1, 64 }.Concat(new byte[64]).ToArray(),
                VerificationScript = data.GetWitnesses()[0].VerificationScript
            };
        }

        return tx;
    }

    public enum InvalidTransactionType
    {
        InsufficientBalance,
        InvalidSignature,
        InvalidScript,
        InvalidAttribute,
        Oversized,
        Expired,
        Conflicting
    }

    class InvalidAttribute : TransactionAttribute
    {
        public override TransactionAttributeType Type => (TransactionAttributeType)0xFF;
        public override bool AllowMultiple { get; }
        protected override void DeserializeWithoutType(ref MemoryReader reader) { }
        protected override void SerializeWithoutType(BinaryWriter writer) { }
    }

    public static void AddTransactionToBlockchain(DataCache snapshot, Transaction tx)
    {
        var block = new Block
        {
            Header = new Header
            {
                Index = NativeContract.Ledger.CurrentIndex(snapshot) + 1,
                PrevHash = NativeContract.Ledger.CurrentHash(snapshot),
                MerkleRoot = new UInt256(Crypto.Hash256(tx.Hash.ToArray())),
                Timestamp = TimeProvider.Current.UtcNow.ToTimestampMS(),
                NextConsensus = UInt160.Zero,
                Witness = new Witness { InvocationScript = Array.Empty<byte>(), VerificationScript = Array.Empty<byte>() }
            },
            Transactions = [tx]
        };

        BlocksAdd(snapshot, block.Hash, block);
    }
}
