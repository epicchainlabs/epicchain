using System.Collections.Generic;
using Neo.Common.Utility;
using Neo.Models.Contracts;
using Neo.Network.P2P.Payloads;
using Neo.Wallets;

namespace Neo.Models.Transactions
{
    public class WitnessModel
    {
        public WitnessModel(Witness witness)
        {
            if (witness != null)
            {
                InvocationScript = witness.InvocationScript.ToArray();
                VerificationScript = witness.VerificationScript.ToArray();
                ScriptHash = witness.ScriptHash;
                InvocationOpCode = OpCodeConverter.Parse(witness.InvocationScript);
                VerificationOpCode = OpCodeConverter.Parse(witness.VerificationScript);
            }
        }
        public byte[] InvocationScript { get; set; }
        public byte[] VerificationScript { get; set; }


        public List<InstructionInfo> InvocationOpCode { get; set; }
        public List<InstructionInfo> VerificationOpCode { get; set; }

        public UInt160 ScriptHash { get; set; }

        public string Address => ScriptHash?.ToAddress();
    }
}