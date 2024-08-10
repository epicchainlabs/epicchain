using System;
using System.IO;
using System.Linq;
using Microsoft.Data.Sqlite;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Logging;
using Neo.Common.Consoles;

namespace Neo.Common.Storage.SQLiteModules
{
    public class SQLiteContext : DbContext
    {

        private static readonly object _lockObject = new object();
        private static byte[] _dbId;
        private readonly string _filename;

        /// <summary>
        /// db file unique identification
        /// </summary>
        public byte[] Identity => _dbId;


        public DbSet<IdentityEntity> Identities { get; set; }
        public DbSet<SyncIndex> SyncIndexes { get; set; }
        public DbSet<TransferEntity> Transfers { get; set; }
        public DbSet<AssetBalanceEntity> AssetBalances { get; set; }
        public DbSet<AddressEntity> Addresses { get; set; }
        public DbSet<TransactionEntity> Transactions { get; set; }

        public DbSet<ContractEntity> Contracts { get; set; }
        public DbSet<ContractUpdateRecordEntity> ContractUpdateRecords { get; set; }

        static SQLiteContext()
        {
            if (!Directory.Exists("Data_Track"))
            {
                Directory.CreateDirectory("Data_Track");
            }
        }

        public SQLiteContext() : this(Path.Combine($"Data_Track", $"track.{CliSettings.Default.Protocol.Network}.db"))
        {

        }

        public SQLiteContext(string filename)
        {
            this._filename = filename;
            Database.EnsureCreated();
            InitDbIdentity();
        }


        private void InitDbIdentity()
        {
            if (_dbId == null)
            {
                lock (_lockObject)
                {
                    if (_dbId == null)
                    {
                        var identity = Identities.FirstOrDefault();
                        if (identity == null)
                        {
                            var guid = Guid.NewGuid().ToByteArray();
                            identity = new IdentityEntity() { Data = guid };
                            Identities.Add(identity);
                            SaveChanges();
                        }
                        _dbId = identity.Data;
                        Console.WriteLine($"SQLite ID:{_dbId.ToHexString()}");
                    }
                }
            }
        }

        public uint? GetMaxSyncIndex()
        {
            try
            {
                return SyncIndexes.OrderByDescending(s => s.BlockHeight).FirstOrDefault()?.BlockHeight;
            }
            catch (Exception e)
            {
                return null;
            }
        }

        protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
        {
            base.OnConfiguring(optionsBuilder);
            SqliteConnectionStringBuilder sb = new SqliteConnectionStringBuilder
            {
                DataSource = _filename
            };
            optionsBuilder.UseSqlite(sb.ToString());
            //optionsBuilder.UseLoggerFactory(LoggerFactory.Create(b => b.AddConsole()));
        }




        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            base.OnModelCreating(modelBuilder);
            modelBuilder.Entity<TransferEntity>().HasIndex(p => p.FromId);
            modelBuilder.Entity<TransferEntity>().HasIndex(p => p.ToId);
            modelBuilder.Entity<TransferEntity>().HasIndex(p => p.Time);
            modelBuilder.Entity<TransferEntity>().HasIndex(p => p.TxId);
            modelBuilder.Entity<TransferEntity>().HasIndex(p => new { p.AssetId, p.TxId, p.Time });

            modelBuilder.Entity<AddressEntity>().HasIndex(p => p.Hash);

            modelBuilder.Entity<AssetBalanceEntity>().HasIndex(p => new { p.AddressId, p.AssetId });

            modelBuilder.Entity<SyncIndex>().HasIndex(p => p.BlockHeight);

            modelBuilder.Entity<TransactionEntity>().HasIndex(p => p.BlockHeight);
            
        }
    }
}
