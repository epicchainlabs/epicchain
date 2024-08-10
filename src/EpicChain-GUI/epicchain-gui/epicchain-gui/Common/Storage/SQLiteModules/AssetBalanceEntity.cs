using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace Neo.Common.Storage.SQLiteModules
{
    [Table("AssetBalance")]
    public class AssetBalanceEntity
    {
        [Key]
        public long Id { get; set; }

        public long AddressId { get; set; }
        public AddressEntity Address { get; set; }

        public long AssetId { get; set; }
        public ContractEntity Asset { get; set; }

        public byte[] Balance { get; set; }

        public uint BlockHeight { get; set; }
    }
}
