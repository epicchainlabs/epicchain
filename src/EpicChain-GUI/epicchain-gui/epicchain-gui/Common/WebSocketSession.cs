using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Neo.Models;

namespace Neo.Common
{
    public class WebSocketSession
    {
        private static readonly AsyncLocal<WebSocketConnection> asyncConnection = new AsyncLocal<WebSocketConnection>();

        public WebSocketConnection Connection
        {
            get => asyncConnection.Value;
            set => asyncConnection.Value = value;
        }

        private static readonly AsyncLocal<WsRequest> asyncRequest = new AsyncLocal<WsRequest>();

        public WsRequest Request
        {
            get => asyncRequest.Value;
            set => asyncRequest.Value = value;
        }


    }
}
