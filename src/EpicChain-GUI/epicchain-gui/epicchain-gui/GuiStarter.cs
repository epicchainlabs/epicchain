using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Neo.Common.Analyzers;
using Neo.Common.Consoles;
using Neo.Common.Scanners;
using Neo.Common.Utility;


namespace Neo
{
    public class GuiStarter : MainService
    {
        public readonly ExecuteResultScanner ExecuteResultScanner;
        public readonly ExecuteResultLogTracker ExecuteResultLogTracker;
        private Task _scanTask;

        public GuiStarter()
        {
            ExecuteResultLogTracker = new ExecuteResultLogTracker();
            ExecuteResultScanner = new ExecuteResultScanner();
        }

        public override async Task Start(string[] args)
        {
            await base.Start(args);
            _scanTask = Task.Factory.StartNew(() => ExecuteResultScanner.Start(), TaskCreationOptions.LongRunning);
            UnconfirmedTransactionCache.RegisterBlockPersistEvent(this.NeoSystem);
        }

    }
}
