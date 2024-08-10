using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Neo.SmartContract;

namespace Neo.Models.Contracts
{
    public class ContractModel
    {
        public ContractModel(ContractState contract)
        {
            ContractId = contract.Id;
            Script = contract.Script.ToArray();
            Manifest = new ManifestModel(contract.Manifest);
        }



        public int ContractId { get; set; }
        public UInt160 ContractHash { get; set; }
        //public bool HasStorage { get; set; }
        //public bool Payable { get; set; }
        public byte[] Script { get; set; }
        public ManifestModel Manifest { get; set; }

    }

}
