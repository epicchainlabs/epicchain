using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models
{
    public class ConfirmStateModel
    {
        /// <summary>
        /// confirm transactions relate current node
        /// </summary>
        public List<UInt256> Confirms { get; set; }

        /// <summary>
        /// confirm transactions relate my current open wallet
        /// </summary>
        public List<UInt256> MyConfirms { get; set; }
    }
}
