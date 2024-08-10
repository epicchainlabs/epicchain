using System.Numerics;

namespace Neo.Common.Storage
{
    public class BalanceInfo
    {
        public UInt160 Address { get; set; }
        public UInt160 Asset { get; set; }
        public string AssetName { get; set; }
        public string AssetSymbol { get; set; }
        /// <summary>
        /// asset decimals
        /// </summary>
        public byte AssetDecimals { get; set; }
        public BigInteger Balance { get; set; }
        public uint BlockHeight { get; set; }
    }
}
