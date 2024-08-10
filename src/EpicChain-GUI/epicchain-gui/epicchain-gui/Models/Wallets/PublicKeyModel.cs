using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models.Wallets
{
    public class PublicKeyModel
    {
        public byte[] PublicKey { get; set; }
        public string Address { get; set; }
    }
}
