using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models.Wallets
{
    public class TransferRequestModel
    {
        public UInt160 Receiver { get; set; }
        public BigDecimal Amount { get; set; }
        public UInt160 Asset { get; set; }
        public UInt160 Sender { get; set; }
    }
}
