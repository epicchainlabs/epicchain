using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace Neo.Common.Storage.SQLiteModules
{
    [Table("Address")]
    public class AddressEntity
    {
        [Key]
        public long Id { get; set; }

        /// <summary>
        /// address script hash string, big-endian without "Ox"
        /// </summary>
        public string Hash { get; set; }

    }
}