using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Common.Storage.SQLiteModules
{
    [Table("ContractUpdateRecord")]

    public class ContractUpdateRecordEntity
    {
        [Key]
        public long Id { get; set; }

        /// <summary>
        /// script hash string, big-endian without "Ox"
        /// </summary>
        public string Hash { get; set; }

        /// <summary>
        /// Delete or migrate Contract transaction hash string, big-endian without "0x"
        /// </summary>
        public string MigrateTxId { get; set; }

        public DateTime MigrateTime { get; set; }

        //public AssetType OldType { get; set; }
        //public AssetType NewType { get; set; }
    }
}
