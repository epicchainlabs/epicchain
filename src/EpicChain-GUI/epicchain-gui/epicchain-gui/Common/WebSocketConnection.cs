using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
using System.Net.WebSockets;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Components.Forms;
using Neo.Models;
using Microsoft.Extensions.DependencyInjection;

namespace Neo.Common
{
    public interface IWebSocketConnection
    {
        void PushMessage(WsMessage message);
    }
    public class WebSocketConnection:IWebSocketConnection
    {
        private readonly WebSocket _socket;

        private readonly BlockingCollection<object> _pushMessagesQueue = new BlockingCollection<object>();

        private readonly ArraySegment<byte> _buffer = WebSocket.CreateServerBuffer(4 * 1024);

        public string ConnectionId { get; set; }

        public WebSocketConnection(WebSocket socket)
        {
            _socket = socket;
            ConnectionId = Guid.NewGuid().ToString("N");
        }

        /// <summary>
        ///  send message (json format) to connection in queue
        /// </summary>
        /// <param name="message"></param>
        public void PushMessage(WsMessage message)
        {
            if (message != null)
            {
                _pushMessagesQueue.Add(message);
            }
        }

        /// <summary>
        /// send message queue loop
        /// </summary>
        /// <returns></returns>
        public async Task PushLoop()
        {
            foreach (var msg in _pushMessagesQueue.GetConsumingEnumerable())
            {
                await SendAsync(msg);
            }
        }

        /// <summary>
        /// send message (json format) to client directly
        /// </summary>
        /// <param name="data"></param>
        /// <returns></returns>
        private async Task SendAsync(object data)
        {
            await _socket.SendAsync(data);
        }

        /// <summary>
        /// close this connection
        /// </summary>
        /// <param name="closeStatus"></param>
        /// <param name="closeDescription"></param>
        /// <returns></returns>
        public async Task CloseAsync(WebSocketCloseStatus closeStatus, string closeDescription)
        {
            await _socket.CloseAsync(closeStatus, closeDescription, CancellationToken.None);
        }


 

        /// <summary>
        /// receive string from client
        /// </summary>
        /// <returns></returns>
        public async Task<WebSocketStringResult> ReceiveStringAsync()
        {
            var receiveResult= await _socket.ReceiveAsync(_buffer, CancellationToken.None);
            var result=new WebSocketStringResult(receiveResult.Count,receiveResult.MessageType,receiveResult.EndOfMessage,receiveResult.CloseStatus,receiveResult.CloseStatusDescription);
            if (result.EndOfMessage)
            {
                result.Message = Encoding.UTF8.GetString(_buffer.Array, 0, result.Count);
            }
            else
            {
                throw new Exception("too long message!");
            }
            return result;
        }






    }
}
