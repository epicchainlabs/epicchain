using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Neo.Network.P2P.Payloads;

namespace Neo.Models.Transactions
{
    public class TransactionPreviewModel
    {
        public UInt256 TxId { get; set; }
        public uint? BlockHeight { get; set; }
        public DateTime? BlockTime => Timestamp.HasValue ? Timestamp.Value.FromTimestampMS().ToLocalTime() : (DateTime?)null;
        public ulong? Timestamp { get; set; }
        public List<TransferModel> Transfers { get; set; }
    }
}
