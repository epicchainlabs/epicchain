using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace Neo.Common.Storage.SQLiteModules
{
    [Table("SyncIndex")]
    public class SyncIndex
    {
        /// <summary>
        /// save synced block height,only block which has "Transfer" event log will save here
        /// </summary>
        [Key]
        [DatabaseGenerated(DatabaseGeneratedOption.None)]
        public uint BlockHeight { get; set; }
    }
}
