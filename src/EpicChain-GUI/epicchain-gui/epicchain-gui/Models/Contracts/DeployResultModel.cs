using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models.Contracts
{
    public class DeployResultModel
    {
        public UInt256 TxId { get; set; }
        public UInt160 ContractHash { get; set; }
        public BigDecimal GasConsumed { get; set; }
    }
}
