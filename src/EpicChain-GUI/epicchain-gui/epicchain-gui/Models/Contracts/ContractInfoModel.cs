using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models.Contracts
{
    public class ContractInfoModel
    {
        public UInt160 Hash { get; set; }
        public string Name { get; set; }
    }
}
