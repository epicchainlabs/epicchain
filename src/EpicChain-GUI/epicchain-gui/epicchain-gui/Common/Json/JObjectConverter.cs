using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Threading.Tasks;
using Neo.Json;

namespace Neo.Common.Json
{
    public class JObjectConverter : JsonConverter<JToken>
    {
        public const int MaxJsonLength = 10 * 1024 * 1024;

        public override bool CanConvert(Type typeToConvert)
        {
            return typeof(JToken).IsAssignableFrom(typeToConvert);
        }

        public override JToken Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
        {
            using JsonDocument document = JsonDocument.ParseValue(ref reader);
            var text = document.RootElement.Clone().ToString();
            if (!text.StartsWith("{"))
            {
                return (JString)text;
            }
            return JToken.Parse(text);
        }

        public override void Write(Utf8JsonWriter writer, JToken value, JsonSerializerOptions options)
        {
            switch (value)
            {
                case JArray array:
                    writer.WriteStartArray();
                    foreach (JObject item in array)
                    {
                        Write(writer, item, options);
                    }
                    writer.WriteEndArray();
                    break;
                case JNumber num:
                    writer.WriteNumberValue(num.Value);
                    break;
                case JString str:
                    writer.WriteStringValue(str.Value);
                    break;
                case JBoolean boolean:
                    writer.WriteBooleanValue(boolean.Value);
                    break;
                case JObject obj:
                    writer.WriteStartObject();
                    foreach (KeyValuePair<string, JToken> pair in obj.Properties)
                    {
                        writer.WritePropertyName(pair.Key);
                        if (pair.Value is null)
                            writer.WriteNullValue();
                        else
                            Write(writer, pair.Value, options);
                    }
                    writer.WriteEndObject();
                    break;
            }

            if (writer.BytesCommitted > MaxJsonLength)
            {
                throw new InvalidCastException("json is too long to write!");
            }
        }
    }
}
