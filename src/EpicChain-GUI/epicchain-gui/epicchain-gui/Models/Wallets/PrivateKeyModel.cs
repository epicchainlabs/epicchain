using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Neo.Wallets;

namespace Neo.Models.Wallets
{
    public class PrivateKeyModel
    {
        public UInt160 ScriptHash { get; set; }
        public string Address => ScriptHash?.ToAddress();
        public string PublicKey { get; set; }
        public string PrivateKey { get; set; }
        public string Wif { get; set; }
    }
}
