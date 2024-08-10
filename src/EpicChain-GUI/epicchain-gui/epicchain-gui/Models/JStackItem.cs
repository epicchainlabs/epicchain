using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Text;
using Neo.VM.Types;

namespace Neo.Models
{
    public class JStackItem
    {
        //public ContractParameterType TypeCode { get; set; }
        public StackItemType TypeCode { get; set; }
        public string Type => TypeCode.ToString();
        public object Value { get; set; }

        public string ValueString => Value is byte[] v ? GetUTF8String(v) : null;
        public string ValueAddress => Value is byte[] v ? v.TryToAddress() : null;
        public UInt160 ValueHash160 => Value is byte[] { Length: 20 } v ? new UInt160(v) : null;
        public UInt256 ValueHash256 => Value is byte[] { Length: 32 } v ? new UInt256(v) : null;

        private string GetUTF8String(byte[] bytes)
        {
            return bytes.IsValidUTF8ByteArray() ? Encoding.UTF8.GetString(bytes) : null;
        }
    }
}
