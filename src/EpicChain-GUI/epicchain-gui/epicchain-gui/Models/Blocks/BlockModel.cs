using System;
using System.Linq;
using System.Numerics;
using Neo.Models.Transactions;
using Neo.Network.P2P.Payloads;
using Neo.SmartContract.Native;

namespace Neo.Models.Blocks
{
    public class BlockModel
    {
        public BlockModel(Block block)
        {
            BlockHash = block.Hash;
            BlockHeight = block.Index;
            Timestamp = block.Timestamp;
            Size = block.Size;
            Version = block.Version;
            PrevHash = block.PrevHash;
            MerkleRoot = block.MerkleRoot;
            NextConsensusHash = block.NextConsensus;
            Witness = new WitnessModel(block.Witness);
            PrimaryIndex = block.PrimaryIndex;
            if (block.Transactions.NotEmpty())
            {
                SystemFee = new BigDecimal((BigInteger) block.Transactions.Sum(t => t.SystemFee),
                    NativeContract.GAS.Decimals).ToString();
                NetworkFee = new BigDecimal((BigInteger) block.Transactions.Sum(t => t.NetworkFee),
                    NativeContract.GAS.Decimals).ToString();
            }
        }

        public UInt256 BlockHash { get; set; }
        public uint BlockHeight { get; set; }
        public DateTime BlockTime => Timestamp.FromTimestampMS().ToLocalTime();
        public uint Confirmations { get; set; }
        public UInt256 MerkleRoot { get; set; }
        public string NetworkFee { get; set; }
        public string NextConsensus => NextConsensusHash?.ToAddress();
        public UInt160 NextConsensusHash { get; set; }
        public UInt256 PrevHash { get; set; }
        public byte PrimaryIndex { get; set; }
        public int Size { get; set; }
        public string SystemFee { get; set; }
        public ulong Timestamp { get; set; }
        public uint Version { get; set; }
        public WitnessModel Witness { get; set; }
    }
}