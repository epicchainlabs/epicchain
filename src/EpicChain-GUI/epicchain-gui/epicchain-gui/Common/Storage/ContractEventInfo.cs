using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Neo.Common.Storage.SQLiteModules;

namespace Neo.Common.Storage
{
    public class ContractEventInfo
    {
        public UInt160 Contract { get; set; }

        /// <summary>
        /// Contract name
        /// </summary>
        public string Name { get; set; }

        public ContractEventType Event { get; set; }

    }
}
