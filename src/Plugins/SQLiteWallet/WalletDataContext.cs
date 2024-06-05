// Copyright (C) 2021-2024 The EpicChain Labs.
//
// WalletDataContext.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using Microsoft.Data.Sqlite;
using Microsoft.EntityFrameworkCore;

namespace Neo.Wallets.SQLite;

class WalletDataContext : DbContext
{
    public DbSet<Account> Accounts { get; set; }
    public DbSet<Address> Addresses { get; set; }
    public DbSet<Contract> Contracts { get; set; }
    public DbSet<Key> Keys { get; set; }

    private readonly string filename;

    public WalletDataContext(string filename)
    {
        this.filename = filename;
    }

    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        base.OnConfiguring(optionsBuilder);
        SqliteConnectionStringBuilder sb = new()
        {
            DataSource = filename
        };
        optionsBuilder.UseSqlite(sb.ToString());
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);
        modelBuilder.Entity<Account>().ToTable(nameof(Account));
        modelBuilder.Entity<Account>().HasKey(p => p.PublicKeyHash);
        modelBuilder.Entity<Account>().Property(p => p.Nep2key).HasColumnType("VarChar").HasMaxLength(byte.MaxValue).IsRequired();
        modelBuilder.Entity<Account>().Property(p => p.PublicKeyHash).HasColumnType("Binary").HasMaxLength(20).IsRequired();
        modelBuilder.Entity<Address>().ToTable(nameof(Address));
        modelBuilder.Entity<Address>().HasKey(p => p.ScriptHash);
        modelBuilder.Entity<Address>().Property(p => p.ScriptHash).HasColumnType("Binary").HasMaxLength(20).IsRequired();
        modelBuilder.Entity<Contract>().ToTable(nameof(Contract));
        modelBuilder.Entity<Contract>().HasKey(p => p.ScriptHash);
        modelBuilder.Entity<Contract>().HasIndex(p => p.PublicKeyHash);
        modelBuilder.Entity<Contract>().HasOne(p => p.Account).WithMany().HasForeignKey(p => p.PublicKeyHash).OnDelete(DeleteBehavior.Cascade);
        modelBuilder.Entity<Contract>().HasOne(p => p.Address).WithMany().HasForeignKey(p => p.ScriptHash).OnDelete(DeleteBehavior.Cascade);
        modelBuilder.Entity<Contract>().Property(p => p.RawData).HasColumnType("VarBinary").IsRequired();
        modelBuilder.Entity<Contract>().Property(p => p.ScriptHash).HasColumnType("Binary").HasMaxLength(20).IsRequired();
        modelBuilder.Entity<Contract>().Property(p => p.PublicKeyHash).HasColumnType("Binary").HasMaxLength(20).IsRequired();
        modelBuilder.Entity<Key>().ToTable(nameof(Key));
        modelBuilder.Entity<Key>().HasKey(p => p.Name);
        modelBuilder.Entity<Key>().Property(p => p.Name).HasColumnType("VarChar").HasMaxLength(20).IsRequired();
        modelBuilder.Entity<Key>().Property(p => p.Value).HasColumnType("VarBinary").IsRequired();
    }
}
