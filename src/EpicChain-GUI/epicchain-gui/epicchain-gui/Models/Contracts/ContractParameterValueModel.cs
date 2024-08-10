using System.Text.Json.Serialization;
using Neo.SmartContract;

namespace Neo.Models.Contracts
{
    public class ContractParameterValueModel
    {
        /// <summary>
        /// Type indicates the type of the parameter. It can be one of the following values: 
        ///     Signature, Boolean, Integer, Hash160, Hash256, ByteArray, PublicKey, String, Array, InteropInterface.
        /// </summary>
        [JsonConverter(typeof(JsonStringEnumConverter))]
        public ContractParameterType Type { get; set; }

        public string Value { get; set; }
    }
}