using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;

namespace Neo.Models.Wallets
{
    public class TransferRequest
    {
        public UInt160 Receiver { get; set; }
        public string Amount { get; set; }
        public string Asset { get; set; }
        public UInt160 Sender { get; set; }
    }
}
