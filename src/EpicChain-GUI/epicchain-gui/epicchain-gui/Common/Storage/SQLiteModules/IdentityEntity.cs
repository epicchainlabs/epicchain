using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Neo.Common.Storage.SQLiteModules
{
    [Table("Identity")]
    public class IdentityEntity
    {
        [Key]
        public long Id { get; set; }

        public byte[] Data { get; set; }
    }
}
