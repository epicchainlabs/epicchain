using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Models
{
    public class HeightStateModel
    {
        public uint SyncHeight { get; set; }
        public uint HeaderHeight { get; set; }

        public int ConnectedCount { get; set; }

        public DateTime Time { get; set; } = DateTime.Now;
    }
}
