using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models.Contracts
{
    public class ValidatorModel
    {
        public string Publickey { get; set; }

        public string Votes { get; set; }

        /// <summary>
        /// In Use
        /// </summary>
        public bool Active { get; set; }
    }
}
