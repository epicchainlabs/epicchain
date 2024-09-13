// Copyright (C) 2021-2024 EpicChain Labs.

//
// TestUtils.Block.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.Util.Internal;
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
using System.Collections.Generic;
using System.Linq;

namespace EpicChain.UnitTests;

public partial class TestUtils
{
    const byte Prefix_Block = 5;
    const byte Prefix_BlockHash = 9;
    const byte Prefix_Transaction = 11;
    const byte Prefix_CurrentBlock = 12;

    /// <summary>
    /// Test Util function SetupHeaderWithValues
    /// </summary>
    /// <param name="snapshot">The snapshot of the current storage provider. Can be null.</param>
    /// <param name="header">The header to be assigned</param>
    /// <param name="val256">PrevHash</param>
    /// <param name="merkRootVal">MerkleRoot</param>
    /// <param name="val160">NextConsensus</param>
    /// <param name="timestampVal">Timestamp</param>
    /// <param name="indexVal">Index</param>
    /// <param name="nonceVal">Nonce</param>
    /// <param name="scriptVal">Witness</param>
    public static void SetupHeaderWithValues(DataCache snapshot, Header header, UInt256 val256, out UInt256 merkRootVal, out UInt160 val160, out ulong timestampVal, out ulong nonceVal, out uint indexVal, out Witness scriptVal)
    {
        header.PrevHash = val256;
        header.MerkleRoot = merkRootVal = UInt256.Parse("0x6226416a0e5aca42b5566f5a19ab467692688ba9d47986f6981a7f747bba2772");
        header.Timestamp = timestampVal = new DateTime(2024, 06, 05, 0, 33, 1, 001, DateTimeKind.Utc).ToTimestampMS();
        if (snapshot != null)
            header.Index = indexVal = NativeContract.Ledger.CurrentIndex(snapshot) + 1;
        else
            header.Index = indexVal = 0;
        header.Nonce = nonceVal = 0;
        header.NextConsensus = val160 = UInt160.Zero;
        header.Witness = scriptVal = new Witness
        {
            InvocationScript = Array.Empty<byte>(),
            VerificationScript = new[] { (byte)OpCode.PUSH1 }
        };
    }

    public static void SetupBlockWithValues(DataCache snapshot, Block block, UInt256 val256, out UInt256 merkRootVal, out UInt160 val160, out ulong timestampVal, out ulong nonceVal, out uint indexVal, out Witness scriptVal, out Transaction[] transactionsVal, int numberOfTransactions)
    {
        Header header = new Header();
        SetupHeaderWithValues(snapshot, header, val256, out merkRootVal, out val160, out timestampVal, out nonceVal, out indexVal, out scriptVal);

        transactionsVal = new Transaction[numberOfTransactions];
        if (numberOfTransactions > 0)
        {
            for (int i = 0; i < numberOfTransactions; i++)
            {
                transactionsVal[i] = GetTransaction(UInt160.Zero);
            }
        }

        block.Header = header;
        block.Transactions = transactionsVal;

        header.MerkleRoot = merkRootVal = MerkleTree.ComputeRoot(block.Transactions.Select(p => p.Hash).ToArray());
    }

    public static Block CreateBlockWithValidTransactions(DataCache snapshot, XEP6Wallet wallet, WalletAccount account, int numberOfTransactions)
    {
        var transactions = new List<Transaction>();
        for (var i = 0; i < numberOfTransactions; i++)
        {
            transactions.Add(CreateValidTx(snapshot, wallet, account));
        }

        return CreateBlockWithValidTransactions(snapshot, account, transactions.ToArray());
    }

    public static Block CreateBlockWithValidTransactions(DataCache snapshot, WalletAccount account, Transaction[] transactions)
    {
        var block = new Block();
        var header = new Header();
        var state = snapshot.TryGet(NativeContract.Ledger.CreateStorageKey(Prefix_CurrentBlock)).GetInteroperable<HashIndexState>();
        SetupHeaderWithValues(snapshot, header, state.Hash, out _, out _, out _, out _, out _, out _);

        block.Header = header;
        block.Transactions = transactions;

        header.MerkleRoot = MerkleTree.ComputeRoot(block.Transactions.Select(p => p.Hash).ToArray());
        var contract = Contract.CreateMultiSigContract(1, TestProtocolSettings.SoleNode.StandbyCommittee);
        var sc = new ContractParametersContext(snapshot, header, TestProtocolSettings.SoleNode.Network);
        var signature = header.Sign(account.GetKey(), TestProtocolSettings.SoleNode.Network);
        sc.AddSignature(contract, TestProtocolSettings.SoleNode.StandbyCommittee[0], signature.ToArray());
        block.Header.Witness = sc.GetWitnesses()[0];

        return block;
    }

    public static void BlocksDelete(DataCache snapshot, UInt256 hash)
    {
        snapshot.Delete(NativeContract.Ledger.CreateStorageKey(Prefix_BlockHash, hash));
        snapshot.Delete(NativeContract.Ledger.CreateStorageKey(Prefix_Block, hash));
    }

    public static void TransactionAdd(DataCache snapshot, params TransactionState[] txs)
    {
        foreach (var tx in txs)
        {
            snapshot.Add(NativeContract.Ledger.CreateStorageKey(Prefix_Transaction, tx.Transaction.Hash), new StorageItem(tx));
        }
    }

    public static void BlocksAdd(DataCache snapshot, UInt256 hash, TrimmedBlock block)
    {
        snapshot.Add(NativeContract.Ledger.CreateStorageKey(Prefix_BlockHash, block.Index), new StorageItem(hash.ToArray()));
        snapshot.Add(NativeContract.Ledger.CreateStorageKey(Prefix_Block, hash), new StorageItem(block.ToArray()));

        var state = snapshot.GetAndChange(NativeContract.Ledger.CreateStorageKey(Prefix_CurrentBlock), () => new StorageItem(new HashIndexState())).GetInteroperable<HashIndexState>();
        state.Hash = hash;
        state.Index = block.Index;
    }

    public static void BlocksAdd(DataCache snapshot, UInt256 hash, Block block)
    {

        block.Transactions.ForEach(tx =>
        {
            var state = new TransactionState
            {
                BlockIndex = block.Index,
                Transaction = tx
            };
            TransactionAdd(snapshot, state);
        });

        snapshot.Add(NativeContract.Ledger.CreateStorageKey(Prefix_BlockHash, block.Index), new StorageItem(hash.ToArray()));
        snapshot.Add(NativeContract.Ledger.CreateStorageKey(Prefix_Block, hash), new StorageItem(block.ToTrimmedBlock().ToArray()));
        var state = snapshot.GetAndChange(NativeContract.Ledger.CreateStorageKey(Prefix_CurrentBlock), () => new StorageItem(new HashIndexState())).GetInteroperable<HashIndexState>();
        state.Hash = hash;
        state.Index = block.Index;
    }

    public static string CreateInvalidBlockFormat()
    {
        // Create a valid block
        var validBlock = new Block
        {
            Header = new Header
            {
                Version = 0,
                PrevHash = UInt256.Zero,
                MerkleRoot = UInt256.Zero,
                Timestamp = 0,
                Index = 0,
                NextConsensus = UInt160.Zero,
                Witness = new Witness { InvocationScript = Array.Empty<byte>(), VerificationScript = Array.Empty<byte>() }
            },
            Transactions = []
        };

        // Serialize the valid block
        byte[] validBlockBytes = validBlock.ToArray();

        // Corrupt the serialized data
        // For example, we can truncate the data by removing the last few bytes
        byte[] invalidBlockBytes = new byte[validBlockBytes.Length - 5];
        Array.Copy(validBlockBytes, invalidBlockBytes, invalidBlockBytes.Length);

        // Convert the corrupted data to a Base64 string
        return Convert.ToBase64String(invalidBlockBytes);
    }

    public static TrimmedBlock ToTrimmedBlock(this Block block)
    {
        return new TrimmedBlock
        {
            Header = block.Header,
            Hashes = block.Transactions.Select(p => p.Hash).ToArray()
        };
    }
}
