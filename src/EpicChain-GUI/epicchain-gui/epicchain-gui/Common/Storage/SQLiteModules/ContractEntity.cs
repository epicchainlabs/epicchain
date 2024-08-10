using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Linq;
using System.Threading.Tasks;

namespace Neo.Common.Storage.SQLiteModules
{
    [Table("Contract")]
    public class ContractEntity
    {
        [Key]
        public long Id { get; set; }

        /// <summary>
        /// script hash string, big-endian without "Ox"
        /// </summary>
        public string Hash { get; set; }

        #region NepInfo

        public string Name { get; set; }
        public string Symbol { get; set; }
        public byte Decimals { get; set; }

        public AssetType AssetType { get; set; }

        #endregion


        public DateTime? CreateTime { get; set; }
        public DateTime? DeleteTime { get; set; }
        public DateTime? MigrateTime { get; set; }

        /// <summary>
        /// Create Contract transaction hash string, big-endian without "0x"
        /// </summary>
        public string CreateTxId { get; set; }


        /// <summary>
        /// Delete or migrate Contract transaction hash string, big-endian without "0x"
        /// </summary>
        public string MigrateTxId { get; set; }

        /// <summary>
        /// Delete or migrate Contract transaction hash string, big-endian without "0x"
        /// </summary>
        public string DeleteTxId { get; set; }

    }


    public enum AssetType
    {
        None,
        Nep11,
        Nep17,
    }
}
