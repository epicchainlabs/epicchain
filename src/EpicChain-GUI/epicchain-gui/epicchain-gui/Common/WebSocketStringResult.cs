using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.WebSockets;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Common
{
    public class WebSocketStringResult:WebSocketReceiveResult
    {
        public WebSocketStringResult(int count, WebSocketMessageType messageType, bool endOfMessage) : base(count, messageType, endOfMessage)
        {
        }

        public WebSocketStringResult(int count, WebSocketMessageType messageType, bool endOfMessage, WebSocketCloseStatus? closeStatus, string closeStatusDescription) : base(count, messageType, endOfMessage, closeStatus, closeStatusDescription)
        {
        }


        public string Message { get; set; }
    }
}
