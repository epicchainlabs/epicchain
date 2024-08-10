using System;
using System.Threading.Tasks;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo;
using Neo.Services.ApiServices;

namespace neo3_gui.tests.Invokers
{
    [TestClass]
    public class BlockInvoker_Test
    {

        [TestInitialize]
        public void Initialize()
        {
            var system = TestBlockchain.TheNeoSystem;
  
        }

        [TestMethod]
        public async Task GetBlock_Test()
        {
            var invoker = new BlockApiService();
            var block= await invoker.GetBlock(0);
            Console.WriteLine(block.SerializeJson());
        }
    }
}
