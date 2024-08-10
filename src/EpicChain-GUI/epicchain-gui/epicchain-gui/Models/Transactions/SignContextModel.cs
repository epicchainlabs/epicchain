using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Neo.Cryptography.ECC;
using Neo.SmartContract;

namespace Neo.Models.Transactions
{
    public class SignContextModel
    {
        public byte[] Script { get; set; }
        public ContractParameter[] Parameters;
        public Dictionary<ECPoint, byte[]> Signatures;
    }
}
