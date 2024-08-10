using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models.Wallets
{
    public class WalletModel
    {
        public List<AccountModel> Accounts { get; set; } = new List<AccountModel>();
    }
}
