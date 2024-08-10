using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Threading.Tasks;
using Neo.SmartContract;

namespace Neo.Common.Storage.LevelDBModules
{
    public class TransferStorageItem
    {
        public UInt160 From { get; set; }
        public UInt160 To { get; set; }
        public BigInteger Amount { get; set; }
        public UInt160 Asset { get; set; }
        public UInt256 TxId { get; set; }
        public TriggerType Trigger { get; set; }
        public string TokenId { get; set; }
    }
}
