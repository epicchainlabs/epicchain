using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json.Serialization;
using System.Threading.Tasks;
using Neo.SmartContract;
using Neo.SmartContract.Manifest;

namespace Neo.Models.Contracts
{
    public class ContractParameterModel
    {
        public ContractParameterModel(ContractParameterDefinition parameter)
        {
            if (parameter != null)
            {
                Name = parameter.Name;
                Type = parameter.Type;
            }
        }

        /// <summary>
        /// Name is the name of the parameter, which can be any valid identifier.
        /// </summary>
        public string Name { get; set; }

        /// <summary>
        /// Type indicates the type of the parameter. It can be one of the following values: 
        ///     Signature, Boolean, Integer, Hash160, Hash256, ByteArray, PublicKey, String, Array, InteropInterface.
        /// </summary>
        [JsonConverter(typeof(JsonStringEnumConverter))]
        public ContractParameterType Type { get; set; }
    }
}
