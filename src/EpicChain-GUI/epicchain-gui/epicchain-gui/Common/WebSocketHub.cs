using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Akka.Util;
using Neo.Models;

namespace Neo.Common
{
    public class WebSocketHub
    {
        private int _limitCount = 10;
        private readonly ConcurrentDictionary<WebSocketConnection, byte> _clients = new();

        public bool Accept(WebSocketConnection connection)
        {
            lock (_clients)
            {
                if (_clients.Count >= _limitCount)
                {
                    return false;
                }
                _clients.TryAdd(connection, 0);
                return true;
            }
        }

        public bool Remove(WebSocketConnection connection)
        {
            lock (_clients)
            {
                var success = _clients.TryRemove(connection, out var removedClient);
                return success;
            }
        }

        private bool IsHeartBeating = false;

        private async Task HeartBeatLoop()
        {
            IsHeartBeating = true;
            while (IsHeartBeating)
            {
                if (_clients.Any())
                {
                    foreach (var client in _clients.Keys)
                    {
                        client.PushMessage(new WsMessage { MsgType = WsMessageType.HeartBeat, Result = "heart beat" });
                    }
                }
                await Task.Delay(TimeSpan.FromSeconds(3));
            }
        }


        /// <summary>
        /// Push Message to all Clients
        /// </summary>
        /// <param name="msg"></param>
        /// <returns></returns>
        public void PushAll(WsMessage msg)
        {
            foreach (var client in _clients.Keys)
            {
                client.PushMessage(msg);
            }
        }


        public void StopHeartBeat()
        {
            IsHeartBeating = false;
        }


    }
}
