using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Common.Storage.LevelDBModules
{
    public class BalanceStorageItem
    {
        public BigInteger Balance { get; set; }
        public uint Height { get; set; }
    }
}
