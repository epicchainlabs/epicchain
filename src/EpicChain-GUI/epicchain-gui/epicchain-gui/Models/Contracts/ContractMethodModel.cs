using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json.Serialization;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;
using Neo.SmartContract;
using Neo.SmartContract.Manifest;

namespace Neo.Models.Contracts
{
    public class ContractMethodModel : ContractEventModel
    {
        public ContractMethodModel(ContractMethodDescriptor method) : base(method)
        {
            if (method != null)
            {
                Offset = method.Offset;
                Safe = method.Safe;
                ReturnType = method.ReturnType;
            }
        }

        public int Offset { get; set; }

        public bool Safe { get; set; }

        /// <summary>
        /// ReturnType indicates the return type of the method. It can be one of the following values: 
        ///     Signature, Boolean, Integer, Hash160, Hash256, ByteArray, PublicKey, String, Array, InteropInterface, Void.
        /// </summary>
        [JsonConverter(typeof(JsonStringEnumConverter))]
        public ContractParameterType ReturnType { get; set; }
    }
}
