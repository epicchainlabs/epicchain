using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;

namespace Neo.Models
{
    public class WsRequest
    {
        public string Id { get; set; }
        public string Method { get; set; }
        public JsonElement Params { get; set; }
    }
}
