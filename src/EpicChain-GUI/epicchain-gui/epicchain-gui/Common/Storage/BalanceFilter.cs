using System.Collections.Generic;

namespace Neo.Common.Storage
{
    public class BalanceFilter
    {
        public IEnumerable<UInt160> Addresses { get; set; }
        public IEnumerable<UInt160> Assets { get; set; }
    }
}
