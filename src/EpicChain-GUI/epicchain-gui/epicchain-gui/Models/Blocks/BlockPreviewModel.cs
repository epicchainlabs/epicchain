using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Neo.Network.P2P.Payloads;

namespace Neo.Models.Blocks
{
    public class BlockPreviewModel
    {
        public BlockPreviewModel(Block block)
        {
            BlockHash = block.Hash;
            BlockHeight = block.Index;
            Timestamp = block.Timestamp;
            TransactionCount = block.Transactions?.Length ?? 0;
            Size = block.Size;
        }
        public UInt256 BlockHash { get; set; }
        public uint BlockHeight { get; set; }
        public DateTime BlockTime => Timestamp.FromTimestampMS().ToLocalTime();
        public ulong Timestamp { get; set; }

        public int Size { get; set; }
        public int TransactionCount { get; set; }
    }
}
