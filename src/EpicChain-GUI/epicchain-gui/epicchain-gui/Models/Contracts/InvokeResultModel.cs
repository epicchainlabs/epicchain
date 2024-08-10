using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json.Serialization;
using System.Threading.Tasks;
using Neo.VM;

namespace Neo.Models.Contracts
{
    public class InvokeResultModel
    {
        public UInt256 TxId { get; set; }
        public UInt160 ContractHash { get; set; }
        public BigDecimal GasConsumed { get; set; }


        [JsonConverter(typeof(JsonStringEnumConverter))]
        public VMState VmState { get; set; }

        public List<JStackItem> ResultStack { get; set; }

        public List<InvokeEventValueModel> Notifications { get; set; }

    }
}
