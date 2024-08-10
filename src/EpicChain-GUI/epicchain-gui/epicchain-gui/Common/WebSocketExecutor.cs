using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Neo.Models;
using Microsoft.Extensions.DependencyInjection;
using Neo.Services;


namespace Neo.Common
{


    public class WebSocketExecutor
    {

        private readonly Dictionary<string, MethodMetadata> _methods = new Dictionary<string, MethodMetadata>(StringComparer.OrdinalIgnoreCase);

        private readonly IServiceProvider _provider;


        public WebSocketExecutor(IServiceProvider provider)
        {
            _provider = provider;
            var invokerType = typeof(IApiService);
            foreach (var type in invokerType.Assembly.GetExportedTypes().Where(t => !t.IsAbstract && t != invokerType && invokerType.IsAssignableFrom(t)))
            {
                foreach (var methodInfo in type.GetMethods(BindingFlags.Instance | BindingFlags.Public | BindingFlags.DeclaredOnly))
                {
                    try
                    {
                        var methodMetadata = new MethodMetadata(methodInfo);
                        if (methodMetadata.IsValid)
                        {
                            _methods[methodInfo.Name] = methodMetadata;
                        }
                    }
                    catch (Exception e)
                    {
                        Console.WriteLine(e);
                        throw;
                    }
                }
            }
        }

        public async Task<object> Execute(WsRequest request)
        {
            if (request.Method.IsNull())
            {
                return ErrorCode.MethodNotFound.ToError();
            }
            if (_methods.TryGetValue(request.Method, out var method))
            {
                var instance = _provider.GetService(method.DeclaringType);
                if (instance is ApiService invoker)
                {
                    invoker.Client = _provider.GetService<WebSocketSession>().Connection;
                }
                return await method.Invoke(instance, request);

            }
            return new WsError() { Code = (int)ErrorCode.MethodNotFound, Message = $"method [{request.Method}] not found!" };
        }




    }
}
