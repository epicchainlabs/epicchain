using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Neo.SmartContract.Native;

namespace Neo.Common
{
    /// <summary>
    /// 
    /// </summary>
    public class AssetComparer: IComparer<UInt160>
    {
        public int Compare(UInt160 x, UInt160 y)
        {
            if (x == y)
            {
                return 0;
            }
            if (x == NativeContract.NEO.Hash)
            {
                return 1;
            }
            if (y == NativeContract.NEO.Hash)
            {
                return -1;
            }
            if (x == NativeContract.GAS.Hash)
            {
                return 1;
            }
            if (y == NativeContract.GAS.Hash)
            {
                return -1;
            }
            return x.CompareTo(y);
        }
    }
}
