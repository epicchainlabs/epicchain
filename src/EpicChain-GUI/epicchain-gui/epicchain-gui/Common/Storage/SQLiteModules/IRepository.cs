using System;
using System.Collections.Generic;
using System.Numerics;
using Neo;
using Neo.Common.Storage;
using Neo.Common.Storage.SQLiteModules;
using Neo.Models;

interface IRepository
{
    byte[] Identity { get; }
    uint? GetMaxSyncIndex();
    void AddSyncIndex(uint index);
    //void SetMaxSyncIndex(uint height);

    void AddTransfer(TransferInfo transfer);
    void AddTransaction(TransactionInfo transaction);


    void UpdateBalance(UInt160 addressHash, UInt160 assetHash, BigInteger balance, uint height);

    PageList<TransactionInfo> QueryTransactions(TransactionFilter filter, bool includeTransfers = false);
    IEnumerable<BalanceInfo> FindAssetBalance(BalanceFilter filter);

    void CreateContract(ContractEntity newContract);
    void DeleteContract(UInt160 contractHash, UInt256 txId, DateTime time);
    void MigrateContract(ContractEntity migrateContract);
    IEnumerable<ContractEntity> GetAllContracts();

    ContractEntity GetActiveContract(UInt160 contract);
    ContractEntity GetActiveContract(string contractHash);
    ContractEntity GetContract(long id);
    AddressEntity GetAddress(long id);
    void Commit();
}

public class SQLiteRepository : IRepository
{
    private SQLiteContext _db;

    public SQLiteRepository(string path)
    {
        _db = new SQLiteContext(path);
    }
    public byte[] Identity => _db.Identity;
    public uint? GetMaxSyncIndex()
    {
        return _db.GetMaxSyncIndex();
    }

    public void AddSyncIndex(uint index)
    {
        _db.Add(new SyncIndex() { BlockHeight = index });
    }

    //public void SetMaxSyncIndex(uint height)
    //{
    //    throw new NotImplementedException();
    //}

    public void AddTransfer(TransferInfo transfer)
    {
        //var from = GetOrCreateAddress(transfer.From);
        //var to = GetOrCreateAddress(transfer.To);
        //var asset = GetActiveContract(transfer.Asset);

        //var tran = new TransferEntity
        //{
        //    BlockHeight = transfer.BlockHeight,
        //    TxId = transfer.TxId?.ToBigEndianHex(),
        //    FromId = from?.Id,
        //    ToId = to?.Id,
        //    Amount = transfer.Amount.ToByteArray(),
        //    AssetId = asset.Id,
        //    Time = transfer.TimeStamp.FromTimestampMS(),
        //    Trigger = transfer.Trigger,
        //    TokenId = transfer.TokenId,
        //};
        //_db.Transfers.Add(tran);
    }

    public void AddTransaction(TransactionInfo transaction)
    {
        throw new NotImplementedException();
    }

    public void UpdateBalance(UInt160 addressHash, UInt160 assetHash, BigInteger balance, uint height)
    {
        throw new NotImplementedException();
    }

    public PageList<TransactionInfo> QueryTransactions(TransactionFilter filter, bool includeTransfers = false)
    {
        throw new NotImplementedException();
    }

    public IEnumerable<BalanceInfo> FindAssetBalance(BalanceFilter filter)
    {
        throw new NotImplementedException();
    }

    public void CreateContract(ContractEntity newContract)
    {
        throw new NotImplementedException();
    }

    public void DeleteContract(UInt160 contractHash, UInt256 txId, DateTime time)
    {
        throw new NotImplementedException();
    }

    public void MigrateContract(ContractEntity migrateContract)
    {
        throw new NotImplementedException();
    }

    public IEnumerable<ContractEntity> GetAllContracts()
    {
        throw new NotImplementedException();
    }

    public ContractEntity GetActiveContract(UInt160 contract)
    {
        throw new NotImplementedException();
    }

    public ContractEntity GetActiveContract(string contractHash)
    {
        throw new NotImplementedException();
    }

    public ContractEntity GetContract(long id)
    {
        throw new NotImplementedException();
    }

    public AddressEntity GetAddress(long id)
    {
        throw new NotImplementedException();
    }

    public void Commit()
    {
        throw new NotImplementedException();
    }
}