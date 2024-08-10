using System.Linq;
using Neo.SmartContract.Manifest;

namespace Neo.Models.Contracts
{
    public class ContractEventModel
    {
        public ContractEventModel(ContractEventDescriptor e)
        {
            if (e != null)
            {
                Name = e.Name;
                Parameters = e.Parameters.Select(p => new ContractParameterModel(p)).ToArray();
            }
        }

        /// <summary>
        /// Name is the name of the method, which can be any valid identifier.
        /// </summary>
        public string Name { get; set; }

        /// <summary>
        /// Parameters is an array of Parameter objects which describe the details of each parameter in the method.
        /// </summary>
        public ContractParameterModel[] Parameters { get; set; }
    }
}