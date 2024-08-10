using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Common.Storage
{
    public enum ContractEventType
    {
        Call = 0,
        Create = 1,
        Destroy = 2,
        Migrate = 3,
    }
}
