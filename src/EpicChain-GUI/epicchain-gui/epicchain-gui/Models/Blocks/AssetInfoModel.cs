using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models.Blocks
{
    public class AssetInfoModel : AssetInfo
    {
        public DateTime? CreateTime { get; set; }
        public BigDecimal? TotalSupply { get; set; }

        public int TransactionCount { get; set; }

    }
}
