// Copyright (C) 2021-2024 EpicChain Labs.

//
// WalletDataContext.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
// distributed as free software under the MIT License, allowing for wide usage and modification
// with minimal restrictions. For comprehensive details regarding the license, please refer to
// the LICENSE file located in the root directory of the repository or visit
// http://www.opensource.org/licenses/mit-license.php.
//
// EpicChain Labs is dedicated to fostering innovation and development in the blockchain space,
// and we believe in the open-source philosophy as a way to drive progress and collaboration.
// This file, along with all associated code and documentation, is provided with the intention of
// supporting and enhancing the development community.
//
// Redistribution and use of this file in both source and binary forms, with or without
// modifications, are permitted. We encourage users to contribute to the project and respect the
// guidelines outlined in the LICENSE file. By using this software, you agree to the terms and
// conditions specified in the MIT License, ensuring the continuation of free and open software
// practices.


using Microsoft.Data.Sqlite;
using Microsoft.EntityFrameworkCore;

namespace EpicChain.Wallets.SQLite;

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
        modelBuilder.Entity<Account>().Property(p => p.Xep2key).HasColumnType("VarChar").HasMaxLength(byte.MaxValue).IsRequired();
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
