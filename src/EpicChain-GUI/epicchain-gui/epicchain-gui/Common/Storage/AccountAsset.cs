using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Common.Storage
{
    public record AccountAsset(UInt160 Account, UInt160 Asset);

    //{
    //    UInt160 Account { get; set; }
    //    UInt160 Asset { get; set; }
    //}
}
