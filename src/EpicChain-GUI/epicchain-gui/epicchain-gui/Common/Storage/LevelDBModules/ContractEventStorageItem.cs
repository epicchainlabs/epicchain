using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Common.Storage.LevelDBModules
{
    public class ContractEventStorageItem
    {
        public UInt256 TxId { get; set; }
        public List<ContractEventInfo> Events { get; set; }
    }
}
