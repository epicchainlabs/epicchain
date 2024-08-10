using System;
using System.Collections.Generic;

namespace Neo.Common.Storage
{
    public class TransferFilter
    {
        public DateTime? StartTime { get; set; }
        public DateTime? EndTime { get; set; }
        public List<UInt160> From { get; set; }
        public List<UInt160> To { get; set; }
        public UInt160 Asset { get; set; }
        public uint? BlockHeight { get; set; }
        public List<UInt256> TxIds { get; set; }

        public List<UInt160> FromOrTo { get; set; }

        /// <summary>
        /// start from 1,paged result only if this is not null
        /// </summary>
        public int PageIndex { get; set; }
        public int PageSize { get; set; }
    }

    public class TransactionFilter
    {
        public DateTime? StartTime { get; set; }
        public DateTime? EndTime { get; set; }

        public uint? BlockHeight { get; set; }
        public List<UInt256> TxIds { get; set; }

        public List<UInt160> From { get; set; }
        public List<UInt160> To { get; set; }

        public List<UInt160> FromOrTo { get; set; }
        
        /// <summary>
        /// Relate  asset contracts hash
        /// </summary>
        public List<UInt160> Assets { get; set; }

        /// <summary>
        /// start from 1,paged result only if this is not null
        /// </summary>
        public int PageIndex { get; set; }
        public int PageSize { get; set; }
    }


}
