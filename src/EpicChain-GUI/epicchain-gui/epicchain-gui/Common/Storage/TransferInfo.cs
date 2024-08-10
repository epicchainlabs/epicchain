using System.Numerics;
using Neo.Common.Storage.LevelDBModules;
using Neo.Models;
using Neo.SmartContract;

namespace Neo.Common.Storage
{
    public class TransferInfo:TransferStorageItem
    {
        public uint BlockHeight { get; set; }
        public ulong TimeStamp { get; set; }
    }
}
