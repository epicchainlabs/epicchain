using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Neo.SmartContract.Manifest;

namespace Neo.Models.Contracts
{
    public class ContractAbiModel
    {
        public ContractAbiModel(ContractAbi abi)
        {
            if (abi != null)
            {
                //ContractHash = abi.Hash;
                //EntryPoint= new ContractMethodModel(abi.EntryPoint);
                Methods = abi.Methods.Select(m => new ContractMethodModel(m)).ToArray();
                Events = abi.Events.Select(m => new ContractEventModel(m)).ToArray();
            }
        }
        ///// <summary>
        ///// Hash is the script hash of the contract. It is encoded as a hexadecimal string in big-endian.
        ///// </summary>
        //public UInt160 ContractHash { get; set; }

        /// <summary>
        /// Entrypoint is a Method object which describe the details of the entrypoint of the contract.
        /// </summary>
        public ContractMethodModel EntryPoint { get; set; }

        /// <summary>
        /// Methods is an array of Method objects which describe the details of each method in the contract.
        /// </summary>
        public ContractMethodModel[] Methods { get; set; }

        /// <summary>
        /// Events is an array of Event objects which describe the details of each event in the contract.
        /// </summary>
        public ContractEventModel[] Events { get; set; }
    }
}
