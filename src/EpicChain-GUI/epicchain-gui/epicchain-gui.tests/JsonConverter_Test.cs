using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using Neo;
using Neo.SmartContract;
using Neo.VM;
using Neo.VM.Types;

namespace neo3_gui.tests
{
    [TestClass]
    public class JsonConverter_Test
    {
        [TestMethod]
        public void Test()
        {
            var data = "1234".HexToBytes();
            StackItem item = new ByteString(data);
            var json = item.SerializeJson();
            Console.WriteLine(json);
            StackItem item2 = json.DeserializeJson<StackItem>();
            Assert.AreEqual(json,item2.SerializeJson());

            item = new Neo.VM.Types.Buffer(data);
            json = item.SerializeJson();
            Console.WriteLine(json);
            item2 = json.DeserializeJson<StackItem>();
            Assert.AreEqual(json, item2.SerializeJson());


            item = new Integer(20);
            json = item.SerializeJson();
            Console.WriteLine(json);
            item2 = json.DeserializeJson<StackItem>();
            Assert.AreEqual(json, item2.SerializeJson());


            item = true;
            json = item.SerializeJson();
            Console.WriteLine(json);
            item2 = json.DeserializeJson<StackItem>();
            Assert.AreEqual(json, item2.SerializeJson());


            item = new Pointer(data, 1);
            json = item.SerializeJson();
            Console.WriteLine(json);
            item2 = json.DeserializeJson<StackItem>();
            Assert.AreEqual(json, item2.SerializeJson());


            item = StackItem.Null;
            json = item.SerializeJson();
            Console.WriteLine(json);
            item2 = json.DeserializeJson<StackItem>();
            Assert.AreEqual(json, item2.SerializeJson());


            var list = new List<StackItem>()
            {
                1, 2, true, false, data,StackItem.Null
            };
            item = new Neo.VM.Types.Array(list);
            json = item.SerializeJson();
            Console.WriteLine(json);
            item2 = json.DeserializeJson<StackItem>();
            Assert.AreEqual(json, item2.SerializeJson());


            item = new Neo.VM.Types.Struct(list);
            json = item.SerializeJson();
            Console.WriteLine(json);
            item2 = json.DeserializeJson<StackItem>();
            Assert.AreEqual(json, item2.SerializeJson());


            var map = new Neo.VM.Types.Map();
            map[1] = true;
            map[2] = false;
            map["a"] = "A";
            map["b"] = StackItem.Null;

            item=map;
            json = item.SerializeJson();
            Console.WriteLine(json);
            item2 = json.DeserializeJson<StackItem>();
            Assert.AreEqual(json, item2.SerializeJson());

        }
    }
}
