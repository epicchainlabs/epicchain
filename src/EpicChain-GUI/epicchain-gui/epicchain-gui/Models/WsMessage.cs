using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Neo.Network.P2P.Payloads;

namespace Neo.Models
{
    public class WsMessage
    {
        public string Id { get; set; }
        public WsMessageType MsgType { get; set; }
        public string Method { get; set; }
        public object Result { get; set; }
        public WsError Error { get; set; }

    }
}
