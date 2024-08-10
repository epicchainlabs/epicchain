using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.WebSockets;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Neo.Models;

namespace Neo.Common
{
    public class WebSocketHubMiddleware : IMiddleware
    {

        private readonly IServiceProvider _provider;

        public WebSocketHubMiddleware(IServiceProvider provider)
        {
            _provider = provider;
        }



        public async Task InvokeAsync(HttpContext context, RequestDelegate next)
        {
            if (context.WebSockets.IsWebSocketRequest)
            {
                var hub = context.RequestServices.GetService<WebSocketHub>();
                var webSocket = await context.WebSockets.AcceptWebSocketAsync();
                var client = new WebSocketConnection(webSocket);
                if (!hub.Accept(client))
                {
                    // unaccepted connection
                    await client.CloseAsync(WebSocketCloseStatus.PolicyViolation, "Too many connections!");
                    return;
                }


                //push loop
                var _ = Task.Run(async () =>
                  {
                      try
                      {
                          await client.PushLoop();
                      }
                      catch (WebSocketException e)
                      {
                          Console.WriteLine(e);
                          hub.Remove(client);
                      }
                  });

                await ReceiveLoop(client);

                return;
            }
            await next(context);
        }



        ///// <summary>
        ///// push message to client, must be single thead
        ///// </summary>
        ///// <param name="connection"></param>
        ///// <param name="hub"></param>
        ///// <returns></returns>
        //private async Task PushLoop(WebSocketConnection connection, WebSocketHub hub)
        //{
        //    try
        //    {
        //        await connection.PushLoop();
        //    }
        //    catch (WebSocketException e)
        //    {
        //        Console.WriteLine(e);
        //        hub.Remove(connection);
        //    }
        //}


        /// <summary>
        /// receive message from client, must be single thread
        /// </summary>
        /// <param name="connection"></param>
        /// <returns></returns>
        public async Task ReceiveLoop(WebSocketConnection connection)
        {
            var result = await connection.ReceiveStringAsync();
            while (!result.CloseStatus.HasValue)
            {
                var task = Task.Run(() => Excute(connection, result.Message));
                result = await connection.ReceiveStringAsync();
            }
            await connection.CloseAsync(result.CloseStatus.Value, result.CloseStatusDescription);
        }


        public async Task Excute(WebSocketConnection connection, string requestString)
        {
            var message = new WsMessage();
            try
            {
                var request = requestString.DeserializeJson<WsRequest>();
                message.MsgType = WsMessageType.Result;
                message.Id = request.Id;
                message.Method = request.Method;

                var session = _provider.GetService<WebSocketSession>();
                session.Connection = connection;
                session.Request = request;

                var executor = _provider.GetService<WebSocketExecutor>();
                var result = await executor.Execute(request);
                if (result is WsError error)
                {
                    message.MsgType = WsMessageType.Error;
                    message.Error = error;
                }
                else
                {
                    message.Result = result;
                }
            }
            catch (ArgumentException ex)
            {
                message.MsgType = WsMessageType.Error;
                message.Error = new WsError()
                {
                    Code = (int)ErrorCode.InvalidPara,
                    Message = ex.Message,
                };
            }
            catch (WsException wsEx)
            {
                message.MsgType = WsMessageType.Error;
                message.Error = new WsError()
                {
                    Code = wsEx.Code,
                    Message = wsEx.Message,
                };
            }
            catch (Exception e)
            {
                message.MsgType = WsMessageType.Error;
                message.Error = new WsError()
                {
                    Code = -1,
                    Message = e.ToString(),
                };
            }
            finally
            {
                connection.PushMessage(message);
            }
        }
    }
}
