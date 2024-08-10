using System;
using System.Collections.Generic;
using System.Linq;
using System.Linq.Expressions;
using System.Reflection;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;
using Neo.Models;

namespace Neo.Common
{
    public class MethodMetadata
    {
        private readonly MethodInfo _methodInfo;
        private readonly ParameterInfo[] _parameters;
        private readonly dynamic _delegate;


        public Type DeclaringType { get; }


        public bool IsValid = false;
        public MethodMetadata(MethodInfo methodInfo)
        {
            _methodInfo = methodInfo;
            DeclaringType = methodInfo.DeclaringType;
            _parameters = methodInfo.GetParameters();
            var paras = new List<Type>();
            paras.Add(methodInfo.DeclaringType);
            paras.AddRange(_parameters.Select(p => p.ParameterType));
            paras.Add(methodInfo.ReturnType);

            if (methodInfo.ReturnType.IsGenericType && methodInfo.ReturnType.GetGenericTypeDefinition() == typeof(Task<>))
            {
                IsValid = true;
                var func = Expression.GetFuncType(paras.ToArray());
                _delegate = Delegate.CreateDelegate(func, methodInfo);
            }
        }


        public dynamic Invoke(object instance, WsRequest request)
        {
            var paras = PrepareParameters(request.Params);
            return InvokeInternal(instance, paras.ToArray());
        }

        private dynamic InvokeInternal(dynamic instance, params dynamic[] paras)
        {
            switch (_parameters.Length)
            {
                case 0:
                    return _delegate(instance);
                case 1:
                    return _delegate(instance, paras[0]);
                case 2:
                    return _delegate(instance, paras[0], paras[1]);
                case 3:
                    return _delegate(instance, paras[0], paras[1], paras[2]);
                case 4:
                    return _delegate(instance, paras[0], paras[1], paras[2], paras[3]);
                case 5:
                    return _delegate(instance, paras[0], paras[1], paras[2], paras[3], paras[4]);
                case 6:
                    return _delegate(instance, paras[0], paras[1], paras[2], paras[3], paras[4], paras[5]);
            }

            return Task.FromResult(string.Empty);
        }

        private List<object> PrepareParameters(JsonElement inputParas)
        {
            var paras = new List<object>();
            if (_parameters.Length == 0)
            {
                //no parameter method
                return paras;
            }
            if (inputParas.ValueKind == JsonValueKind.Undefined)
            {
                //no input paras
                paras.AddRange(_parameters.Select(p => p.DefaultValue));
                return paras;
            }
            //only accept one parameter
            if (_parameters.Length == 1)
            {
                var parameterType = _parameters[0].ParameterType;

                if (inputParas.ValueKind == JsonValueKind.Array && parameterType.IsArray)
                {
                    // input paras is Array format, method only accept one array parameter
                    paras.Add(inputParas.GetRawText().DeserializeJson(parameterType));
                    return paras;
                }

                if (!parameterType.IsPrimitive && !parameterType.IsArray && !parameterType.IsEnum && parameterType != typeof(string) && parameterType != typeof(UInt256) && parameterType != typeof(UInt160))
                {
                    //method only accept one Object parameter
                    paras.Add(inputParas.GetRawText().DeserializeJson(parameterType));
                    return paras;
                }
            }

            //input para is array, method accept many parameters
            if (inputParas.ValueKind == JsonValueKind.Array)
            {
                paras.AddRange(_parameters.Select((p, index) => inputParas[index].GetRawText().DeserializeJson(p.ParameterType)));
                return paras;
            }

            // input para is Object, method accept many parameters
            foreach (var parameterInfo in _parameters)
            {
                if (inputParas.TryGetProperty(parameterInfo.Name, out var paraVal))
                {
                    paras.Add(paraVal.GetRawText().DeserializeJson(parameterInfo.ParameterType));
                }
                else
                {
                    //try find paraVal case-insensitive
                    var paraToken = inputParas.EnumerateObject().FirstOrDefault(p => parameterInfo.Name.Equals(p.Name, StringComparison.OrdinalIgnoreCase));
                    if (paraToken.Value.ValueKind != JsonValueKind.Undefined)
                    {
                        //found
                        paras.Add(paraToken.Value.GetRawText().DeserializeJson(parameterInfo.ParameterType));
                    }
                    else
                    {
                        //not found, set default value
                        //paras.Add(parameterInfo.ParameterType.GetDefaultValue());
                        paras.Add(parameterInfo.DefaultValue != DBNull.Value ? parameterInfo.DefaultValue : parameterInfo.ParameterType.GetDefaultValue());
                    }
                }
            }
            return paras;
        }
    }
}
