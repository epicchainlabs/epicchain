using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace Neo.Common.Json
{
    public class UInt256Converter : JsonConverter<UInt256>
    {
        public override UInt256 Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            var data = reader.GetString();
            if (UInt256.TryParse(data, out var hash))
            {
                return hash;
            }
            throw new ArgumentException($"invalid uint256 string:{data}");
        }

        public override void Write(Utf8JsonWriter writer, UInt256 value, JsonSerializerOptions options)
        {
            writer.WriteStringValue(value.ToString());
        }
    }
}
