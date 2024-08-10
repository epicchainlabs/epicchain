using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Neo.Common.Storage;
using Neo.Wallets;

namespace Neo.Models.Wallets
{
    public class AddressBalanceModel
    {
        public UInt160 AddressHash { get; set; }
        public string Address => AddressHash?.ToAddress();

        public List<AssetBalanceModel> Balances { get; set; } = new List<AssetBalanceModel>();
    }
}
