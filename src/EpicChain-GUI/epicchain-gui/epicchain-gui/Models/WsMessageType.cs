using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models
{
    public enum WsMessageType
    {
        None = 0,
        HeartBeat = 1,
        Push = 2,
        Result = 3,
        Error = -1,
    }
}
