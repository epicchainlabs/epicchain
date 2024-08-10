using System.Text.Json.Serialization;
using Neo.Network.P2P.Payloads;

namespace Neo.Models.Contracts
{
    public class CosignerModel
    {
        public UInt160 Account { get; set; }


        [JsonConverter(typeof(JsonStringEnumConverter))]
        public WitnessScope Scopes { get; set; } = WitnessScope.Global;
    }
}