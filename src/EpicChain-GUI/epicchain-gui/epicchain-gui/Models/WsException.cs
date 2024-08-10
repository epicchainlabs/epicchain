using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models
{
    public class WsException : Exception
    {

        public WsException(ErrorCode code) : this(code, code.ToError().Message)
        {
        }
        public WsException(ErrorCode code, string message) : this((int)code, message)
        {
        }
        private WsException(int code, string message) : base(message)
        {
            Code = code;
        }
        public int Code { get; set; }
    }
}
