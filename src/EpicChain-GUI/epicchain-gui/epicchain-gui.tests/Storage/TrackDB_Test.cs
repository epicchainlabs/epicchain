using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo;
using Neo.Common.Storage;
using Neo.Models;
using Neo.SmartContract.Native;

namespace neo3_gui.tests.Storage
{
    [TestClass]
    public class TrackDB_Test
    {
        private TrackDB _db = new TrackDB();



        [TestMethod]
        public async Task AddTransfer_Test()
        {
            var transfer = new TransferInfo()
            {
                BlockHeight = 0,
                From = UInt160.Parse("0xf9df308b7bb380469354062f6b73f9cb0124317b"),
                To = UInt160.Parse("0x18c52425debcc3c76c06c3368044b8c60609a904"),
                    
                Asset = NativeContract.NEO.Hash,
                Amount = 1,
                TxId = UInt256.Parse(""),
                TimeStamp = DateTime.Now.ToTimestamp(),
                //AssetInfo = new AssetInfo()
                //{
                //    Asset = NativeContract.NEO.Hash,
                //    Name = "NEO",
                //    Symbol = "neo",
                //    Decimals = NativeContract.NEO.Decimals,
                //},
            };

            _db.AddTransfer(transfer);
            //_db.Commit();
        }
    }
}
