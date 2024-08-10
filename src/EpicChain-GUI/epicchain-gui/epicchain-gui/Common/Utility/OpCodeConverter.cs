using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Numerics;
using System.Reflection;
using System.Text;
using System.Threading.Tasks;
using Neo.Models.Contracts;
using Neo.SmartContract;
using Neo.VM;

namespace Neo.Common.Utility
{
    public class OpCodeConverter
    {
        private static readonly Dictionary<uint, string> _interopServiceMap;
        private static readonly int[] _operandSizePrefixTable = new int[256];
        private static readonly int[] _operandSizeTable = new int[256];


        static OpCodeConverter()
        {
            //初始化所有 InteropService Method
            _interopServiceMap = ApplicationEngine.Services.ToDictionary(s => s.Key, s => s.Value.Name);
            //初始化所有 OpCode OperandSize
            foreach (FieldInfo field in typeof(OpCode).GetFields(BindingFlags.Public | BindingFlags.Static))
            {
                var attribute = field.GetCustomAttribute<OperandSizeAttribute>();
                if (attribute == null) continue;
                int index = (int)(OpCode)field.GetValue(null);
                _operandSizePrefixTable[index] = attribute.SizePrefix;
                _operandSizeTable[index] = attribute.Size;
            }
        }
        public static string ToAsciiString(byte[] byteArray)
        {
            var output = Encoding.UTF8.GetString(byteArray);
            if (output.Any(p => p < '0' || p > 'z')) return byteArray.ToHexString();
            return output;
        }


        public static List<InstructionInfo> Parse(ReadOnlyMemory<byte> scripts)
        {
            return Parse(scripts.ToArray());
        }

        public static List<InstructionInfo> Parse(byte[] scripts)
        {
            var result = new List<InstructionInfo>();
            try
            {
                var s = new Script(scripts);
                for (int ip = 0; ip < scripts.Length;)
                {
                    var instruction = s.GetInstruction(ip);

                    var instructionInfo = new InstructionInfo()
                    { OpCode = instruction.OpCode, Position = ip, OpData = instruction.Operand.ToArray() };

                    if (instruction.OpCode == OpCode.SYSCALL)
                    {
                        instructionInfo.SystemCallMethod = _interopServiceMap[BitConverter.ToUInt32(instructionInfo.OpData)];
                    }

                    result.Add(instructionInfo);


                    ip += instruction.Size;
                }

            }
            catch (Exception e)
            {
                Console.WriteLine($"{scripts.ToHexString()}:{e}");
            }

            return result;
        }

    }
}
