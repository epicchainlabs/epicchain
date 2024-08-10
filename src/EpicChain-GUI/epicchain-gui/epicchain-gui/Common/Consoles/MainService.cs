using Akka.Actor;
using Neo.IO;
using Neo.Ledger;
using Neo.Network.P2P;
using Neo.Network.P2P.Payloads;
using Neo.Plugins;
using Neo.SmartContract;
using Neo.SmartContract.Native;
using Neo.Wallets;
using Neo.Wallets.NEP6;
using System;
using System.Collections.Generic;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Net;
using System.Numerics;
using System.Security.Cryptography;
using System.Text.RegularExpressions;
using System.Threading;
using System.Threading.Tasks;


namespace Neo.Common.Consoles
{
    public class MainService : ConsoleServiceBase
    {
        public event EventHandler WalletChanged;

        public LocalNode LocalNode;

        private Wallet currentWallet;
        public Wallet CurrentWallet
        {
            get
            {
                return currentWallet;
            }
            internal set
            {
                currentWallet = value;
                WalletChanged?.Invoke(this, EventArgs.Empty);
            }
        }

        private NeoSystem neoSystem;
        public NeoSystem NeoSystem
        {
            get
            {
                return neoSystem;
            }
            private set
            {
                neoSystem = value;
            }
        }

        protected override string Prompt => "neo";
        public override string ServiceName => "NEO-CLI";

        public virtual async Task Start(string[] args)
        {
            if (NeoSystem != null) return;
            try
            {
                NeoSystem = new NeoSystem(CliSettings.Default.Protocol, CliSettings.Default.Storage.Engine, CliSettings.Default.Storage.Path);
                NeoSystem.AddService(this);

                LocalNode = await NeoSystem.LocalNode.Ask<LocalNode>(new LocalNode.GetInstance());
            }
            catch (Exception e)
            {
                Console.WriteLine(e);
                throw;
            }

            Task.Run(ImportAndStartNode);

            //NeoSystem.StartNode(new ChannelsConfig
            //{
            //    Tcp = new IPEndPoint(IPAddress.Any, CliSettings.Default.P2P.Port),
            //    WebSocket = new IPEndPoint(IPAddress.Any, CliSettings.Default.P2P.WsPort),
            //    MinDesiredConnections = CliSettings.Default.P2P.MinDesiredConnections,
            //    MaxConnections = CliSettings.Default.P2P.MaxConnections,
            //    MaxConnectionsPerAddress = CliSettings.Default.P2P.MaxConnectionsPerAddress
            //});
            //if (CliSettings.Default.UnlockWallet.IsActive)
            //{
            //    try
            //    {
            //        OpenWallet(CliSettings.Default.UnlockWallet.Path, CliSettings.Default.UnlockWallet.Password);
            //    }
            //    catch (FileNotFoundException)
            //    {
            //        Console.WriteLine($"Warning: wallet file \"{CliSettings.Default.UnlockWallet.Path}\" not found.");
            //    }
            //    catch (CryptographicException)
            //    {
            //        Console.WriteLine($"failed to open file \"{CliSettings.Default.UnlockWallet.Path}\"");
            //    }
            //}
        }

        private async Task ImportAndStartNode()
        {
            await ImportBlocks();
            await StartNode();
        }


        public async Task ImportBlocks()
        {
            using IEnumerator<Block> blocksBeingImported = GetBlocksFromFile().GetEnumerator();
            while (true)
            {
                List<Block> blocksToImport = new List<Block>();
                for (int i = 0; i < 10; i++)
                {
                    if (!blocksBeingImported.MoveNext()) break;
                    blocksToImport.Add(blocksBeingImported.Current);
                }
                if (blocksToImport.Count == 0) break;
                await NeoSystem.Blockchain.Ask<Blockchain.ImportCompleted>(new Blockchain.Import { Blocks = blocksToImport });
                if (NeoSystem is null) return;
            }
        }

        public async Task StartNode()
        {
            NeoSystem.StartNode(new ChannelsConfig
            {
                Tcp = new IPEndPoint(IPAddress.Any, CliSettings.Default.P2P.Port),
                MinDesiredConnections = CliSettings.Default.P2P.MinDesiredConnections,
                MaxConnections = CliSettings.Default.P2P.MaxConnections,
                MaxConnectionsPerAddress = CliSettings.Default.P2P.MaxConnectionsPerAddress
            });
        }

        public void Stop()
        {
            Interlocked.Exchange(ref neoSystem, null)?.Dispose();
        }



        private IEnumerable<Block> GetBlocks(Stream stream, bool read_start = false)
        {
            using BinaryReader r = new BinaryReader(stream);
            uint start = read_start ? r.ReadUInt32() : 0;
            uint count = r.ReadUInt32();
            uint end = start + count - 1;
            uint currentHeight = NativeContract.Ledger.CurrentIndex(NeoSystem.StoreView);
            if (end <= currentHeight) yield break;
            for (uint height = start; height <= end; height++)
            {
                var size = r.ReadInt32();
                if (size > Message.PayloadMaxSize)
                    throw new ArgumentException($"Block {height} exceeds the maximum allowed size");

                byte[] array = r.ReadBytes(size);
                if (height > currentHeight)
                {
                    Block block = array.AsSerializable<Block>();
                    yield return block;
                }
            }
        }

        private IEnumerable<Block> GetBlocksFromFile()
        {
            const string pathAcc = "chain.acc";
            if (File.Exists(pathAcc))
                using (FileStream fs = new FileStream(pathAcc, FileMode.Open, FileAccess.Read, FileShare.Read))
                    foreach (var block in GetBlocks(fs))
                        yield return block;

            const string pathAccZip = pathAcc + ".zip";
            if (File.Exists(pathAccZip))
                using (FileStream fs = new FileStream(pathAccZip, FileMode.Open, FileAccess.Read, FileShare.Read))
                using (ZipArchive zip = new ZipArchive(fs, ZipArchiveMode.Read))
                using (Stream zs = zip.GetEntry(pathAcc).Open())
                    foreach (var block in GetBlocks(zs))
                        yield return block;

            var paths = Directory.EnumerateFiles(".", "chain.*.acc", SearchOption.TopDirectoryOnly).Concat(Directory.EnumerateFiles(".", "chain.*.acc.zip", SearchOption.TopDirectoryOnly)).Select(p => new
            {
                FileName = Path.GetFileName(p),
                Start = uint.Parse(Regex.Match(p, @"\d+").Value),
                IsCompressed = p.EndsWith(".zip")
            }).OrderBy(p => p.Start);

            uint height = NativeContract.Ledger.CurrentIndex(NeoSystem.StoreView);
            foreach (var path in paths)
            {
                if (path.Start > height + 1) break;
                if (path.IsCompressed)
                    using (FileStream fs = new FileStream(path.FileName, FileMode.Open, FileAccess.Read, FileShare.Read))
                    using (ZipArchive zip = new ZipArchive(fs, ZipArchiveMode.Read))
                    using (Stream zs = zip.GetEntry(Path.GetFileNameWithoutExtension(path.FileName)).Open())
                        foreach (var block in GetBlocks(zs, true))
                            yield return block;
                else
                    using (FileStream fs = new FileStream(path.FileName, FileMode.Open, FileAccess.Read, FileShare.Read))
                        foreach (var block in GetBlocks(fs, true))
                            yield return block;
            }
        }

        private bool NoWallet()
        {
            if (CurrentWallet != null) return false;
            Console.WriteLine("You have to open the wallet first.");
            return true;
        }

        protected override bool OnCommand(string[] args)
        {
            if (Plugin.SendMessage(args)) return true;
            switch (args[0].ToLower())
            {
                case "export":
                    return OnExportCommand(args);
                case "plugins":
                    return OnPluginsCommand(args);
                case "list":
                    return OnListCommand(args);
                case "open":
                    return OnOpenCommand(args);
                case "close":
                    return OnCloseCommand(args);
                case "send":
                    return OnSendCommand(args);
                case "show":
                    return OnShowCommand(args);
                default:
                    return base.OnCommand(args);
            }
        }


        private bool OnExportCommand(string[] args)
        {
            switch (args[1].ToLower())
            {
                case "block":
                case "blocks":
                    return OnExportBlocksCommand(args);
                case "key":
                    return OnExportKeyCommand(args);
                default:
                    return base.OnCommand(args);
            }
        }

        private bool OnExportBlocksCommand(string[] args)
        {
            uint height = NativeContract.Ledger.CurrentIndex(NeoSystem.StoreView);
            var start = args.Length > 2 ? uint.Parse(args[2]) : 0;
            var count = args.Length > 3 ? uint.Parse(args[3]) : uint.MaxValue;
            var path = args.Length > 4 ? args[4] : "chain.acc";

            if (height < start)
            {
                Console.WriteLine("Error: invalid start height.");
                return true;
            }

            count = Math.Min(count, height - start + 1);

            if (start > 0)
            {
                path = $"chain.{start}.acc";
            }
            WriteBlocks(start, count, path, true);
            return true;
        }

        private bool OnExportKeyCommand(string[] args)
        {
            if (NoWallet()) return true;
            if (args.Length < 2 || args.Length > 4)
            {
                Console.WriteLine("error");
                return true;
            }
            UInt160 scriptHash = null;
            string path = null;
            if (args.Length == 3)
            {
                try
                {
                    scriptHash = args[2].ToScriptHash();
                }
                catch (FormatException)
                {
                    path = args[2];
                }
            }
            else if (args.Length == 4)
            {
                scriptHash = args[2].ToScriptHash();
                path = args[3];
            }
            if (File.Exists(path))
            {
                Console.WriteLine($"Error: File '{path}' already exists");
                return true;
            }
            string password = ReadUserInput("password", true);
            if (password.Length == 0)
            {
                Console.WriteLine("cancelled");
                return true;
            }
            if (!CurrentWallet.VerifyPassword(password))
            {
                Console.WriteLine("Incorrect password");
                return true;
            }
            IEnumerable<KeyPair> keys;
            if (scriptHash == null)
                keys = CurrentWallet.GetAccounts().Where(p => p.HasKey).Select(p => p.GetKey());
            else
                keys = new[] { CurrentWallet.GetAccount(scriptHash).GetKey() };
            if (path == null)
                foreach (KeyPair key in keys)
                    Console.WriteLine(key.Export());
            else
                File.WriteAllLines(path, keys.Select(p => p.Export()));
            return true;
        }

        private bool OnPluginsCommand(string[] args)
        {
            if (Plugin.Plugins.Count > 0)
            {
                Console.WriteLine("Loaded plugins:");
                Plugin.Plugins.ForEach(p => Console.WriteLine("\t" + p.Name));
            }
            else
            {
                Console.WriteLine("No loaded plugins");
            }
            return true;
        }

        private bool OnListCommand(string[] args)
        {
            switch (args[1].ToLower())
            {
                case "address":
                    return OnListAddressCommand(args);
                case "asset":
                    return OnListAssetCommand(args);
                case "key":
                    return OnListKeyCommand(args);
                default:
                    return base.OnCommand(args);
            }
        }

        private bool OnShowGasCommand(string[] args)
        {
            if (NoWallet()) return true;
            BigInteger gas = BigInteger.Zero;
            var snapshot = NeoSystem.StoreView;
            uint height = NativeContract.Ledger.CurrentIndex(snapshot) + 1;
            foreach (UInt160 account in CurrentWallet.GetAccounts().Select(p => p.ScriptHash))
                gas += NativeContract.NEO.UnclaimedGas(snapshot, account, height);
            Console.WriteLine($"Unclaimed gas: {new BigDecimal(gas, NativeContract.GAS.Decimals)}");
            return true;
        }

        private bool OnListKeyCommand(string[] args)
        {
            if (NoWallet()) return true;
            foreach (KeyPair key in CurrentWallet.GetAccounts().Where(p => p.HasKey).Select(p => p.GetKey()))
            {
                Console.WriteLine(key.PublicKey);
            }
            return true;
        }

        private bool OnListAddressCommand(string[] args)
        {
            if (NoWallet()) return true;

            var snapshot = NeoSystem.StoreView;
            foreach (var account in CurrentWallet.GetAccounts())
            {
                var contract = account.Contract;
                var type = "Nonstandard";

                if (account.WatchOnly)
                {
                    type = "WatchOnly";
                }
                else if (contract.Script.IsMultiSigContract())
                {
                    type = "MultiSignature";
                }
                else if (contract.Script.IsSignatureContract())
                {
                    type = "Standard";
                }
                else if (NativeContract.ContractManagement.GetContract(snapshot, account.ScriptHash) != null)
                {
                    type = "Deployed-Nonstandard";
                }

                Console.WriteLine($"{"   Address: "}{account.Address}\t{type}");
                Console.WriteLine($"{"ScriptHash: "}{account.ScriptHash}\n");
            }

            return true;
        }

        private bool OnListAssetCommand(string[] args)
        {
            if (NoWallet()) return true;
            var snapshot = NeoSystem.StoreView;
            foreach (UInt160 account in CurrentWallet.GetAccounts().Select(p => p.ScriptHash))
            {
                Console.WriteLine(account.ToAddress(NeoSystem.Settings.AddressVersion));
                Console.WriteLine($"NEO: {CurrentWallet.GetBalance(snapshot, NativeContract.NEO.Hash, account)}");
                Console.WriteLine($"GAS: {CurrentWallet.GetBalance(snapshot, NativeContract.GAS.Hash, account)}");
                Console.WriteLine();
            }
            Console.WriteLine("----------------------------------------------------");
            Console.WriteLine($"Total:   NEO: {CurrentWallet.GetAvailable(snapshot, NativeContract.NEO.Hash),10}     GAS: {CurrentWallet.GetAvailable(snapshot, NativeContract.GAS.Hash),18}");
            Console.WriteLine();
            Console.WriteLine("NEO hash: " + NativeContract.NEO.Hash);
            Console.WriteLine("GAS hash: " + NativeContract.GAS.Hash);
            return true;
        }

        private bool OnOpenCommand(string[] args)
        {
            switch (args[1].ToLower())
            {
                case "wallet":
                    return OnOpenWalletCommand(args);
                default:
                    return base.OnCommand(args);
            }
        }

        //TODO: 目前没有想到其它安全的方法来保存密码
        //所以只能暂时手动输入，但如此一来就不能以服务的方式启动了
        //未来再想想其它办法，比如采用智能卡之类的
        private bool OnOpenWalletCommand(string[] args)
        {
            if (args.Length < 3)
            {
                Console.WriteLine("error");
                return true;
            }
            string path = args[2];
            if (!File.Exists(path))
            {
                Console.WriteLine($"File does not exist");
                return true;
            }
            string password = ReadUserInput("password", true);
            if (password.Length == 0)
            {
                Console.WriteLine("cancelled");
                return true;
            }
            try
            {
                OpenWallet(path, password);
            }
            catch (CryptographicException)
            {
                Console.WriteLine($"failed to open file \"{path}\"");
            }
            return true;
        }

        /// <summary>
        /// process "close" command
        /// </summary>
        /// <param name="args"></param>
        /// <returns></returns>
        private bool OnCloseCommand(string[] args)
        {
            switch (args[1].ToLower())
            {
                case "wallet":
                    return OnCloseWalletCommand(args);
                default:
                    return base.OnCommand(args);
            }
        }

        /// <summary>
        /// process "close wallet" command
        /// </summary>
        /// <param name="args"></param>
        /// <returns></returns>
        public bool OnCloseWalletCommand(string[] args)
        {
            if (CurrentWallet == null)
            {
                Console.WriteLine($"Wallet is not opened");
                return true;
            }
            CurrentWallet = null;
            Console.WriteLine($"Wallet is closed");
            return true;
        }

        private bool OnSendCommand(string[] args)
        {
            if (args.Length != 4)
            {
                Console.WriteLine("error");
                return true;
            }
            if (NoWallet()) return true;
            string password = ReadUserInput("password", true);
            if (password.Length == 0)
            {
                Console.WriteLine("cancelled");
                return true;
            }
            if (!CurrentWallet.VerifyPassword(password))
            {
                Console.WriteLine("Incorrect password");
                return true;
            }
            UInt160 assetId;
            switch (args[1].ToLower())
            {
                case "neo":
                    assetId = NativeContract.NEO.Hash;
                    break;
                case "gas":
                    assetId = NativeContract.GAS.Hash;
                    break;
                default:
                    assetId = UInt160.Parse(args[1]);
                    break;
            }
            UInt160 to = args[2].ToScriptHash();
            var snapshot = NeoSystem.StoreView;
            Transaction tx;
            AssetDescriptor descriptor = new AssetDescriptor(snapshot, CliSettings.Default.Protocol, assetId);
            if (!BigDecimal.TryParse(args[3], descriptor.Decimals, out BigDecimal amount) || amount.Sign <= 0)
            {
                Console.WriteLine("Incorrect Amount Format");
                return true;
            }
            tx = CurrentWallet.MakeTransaction(snapshot, new[]
            {
                new TransferOutput
                {
                    AssetId = assetId,
                    Value = amount,
                    ScriptHash = to
                }
            });

            if (tx == null)
            {
                Console.WriteLine("Insufficient funds");
                return true;
            }

            ContractParametersContext context = new ContractParametersContext(snapshot, tx, CliSettings.Default.Protocol.Network);
            CurrentWallet.Sign(context);
            if (context.Completed)
            {
                tx.Witnesses = context.GetWitnesses();
                NeoSystem.Blockchain.Tell(tx);
                //NeoSystem.LocalNode.Tell(new LocalNode.Relay { Inventory = tx });
                Console.WriteLine($"TXID: {tx.Hash}");
            }
            else
            {
                Console.WriteLine("SignatureContext:");
                Console.WriteLine(context.ToString());
            }

            return true;
        }

        private bool OnShowCommand(string[] args)
        {
            switch (args[1].ToLower())
            {
                case "gas":
                    return OnShowGasCommand(args);
                case "pool":
                    return OnShowPoolCommand(args);
                case "state":
                    return OnShowStateCommand(args);
                default:
                    return base.OnCommand(args);
            }
        }

        private bool OnShowPoolCommand(string[] args)
        {
            bool verbose = args.Length >= 3 && args[2] == "verbose";

            int verifiedCount, unverifiedCount;
            if (verbose)
            {
                NeoSystem.MemPool.GetVerifiedAndUnverifiedTransactions(
                    out IEnumerable<Transaction> verifiedTransactions,
                    out IEnumerable<Transaction> unverifiedTransactions);
                Console.WriteLine("Verified Transactions:");
                foreach (Transaction tx in verifiedTransactions)
                    Console.WriteLine($" {tx.Hash} {tx.GetType().Name} {tx.NetworkFee} GAS_NetFee");
                Console.WriteLine("Unverified Transactions:");
                foreach (Transaction tx in unverifiedTransactions)
                    Console.WriteLine($" {tx.Hash} {tx.GetType().Name} {tx.NetworkFee} GAS_NetFee");

                verifiedCount = verifiedTransactions.Count();
                unverifiedCount = unverifiedTransactions.Count();
            }
            else
            {
                verifiedCount = NeoSystem.MemPool.VerifiedCount;
                unverifiedCount = NeoSystem.MemPool.UnVerifiedCount;
            }
            Console.WriteLine($"total: {NeoSystem.MemPool.Count}, verified: {verifiedCount}, unverified: {unverifiedCount}");
            return true;
        }

        private bool OnShowStateCommand(string[] args)
        {
            var cancel = new CancellationTokenSource();

            Console.CursorVisible = false;
            Console.Clear();

            Task broadcast = Task.Run(async () =>
            {
                while (!cancel.Token.IsCancellationRequested)
                {
                    NeoSystem.LocalNode.Tell(Message.Create(MessageCommand.Ping, PingPayload.Create(NativeContract.Ledger.CurrentIndex(NeoSystem.StoreView))));
                    await Task.Delay(NeoSystem.Settings.TimePerBlock, cancel.Token);
                }
            });
            Task task = Task.Run(async () =>
            {
                int maxLines = 0;
                while (!cancel.Token.IsCancellationRequested)
                {
                    uint height = NativeContract.Ledger.CurrentIndex(NeoSystem.StoreView);
                    uint headerHeight = NeoSystem.HeaderCache.Last?.Index ?? height;

                    Console.SetCursorPosition(0, 0);
                    WriteLineWithoutFlicker($"block: {height}/{headerHeight}  connected: {LocalNode.ConnectedCount}  unconnected: {LocalNode.UnconnectedCount}", Console.WindowWidth - 1);

                    int linesWritten = 1;
                    foreach (RemoteNode node in LocalNode.GetRemoteNodes().OrderByDescending(u => u.LastBlockIndex).Take(Console.WindowHeight - 2).ToArray())
                    {
                        Console.WriteLine(
                            $"  ip: {node.Remote.Address,-15}\tport: {node.Remote.Port,-5}\tlisten: {node.ListenerTcpPort,-5}\theight: {node.LastBlockIndex,-7}");
                        linesWritten++;
                    }

                    maxLines = Math.Max(maxLines, linesWritten);

                    while (linesWritten < maxLines)
                    {
                        WriteLineWithoutFlicker("", Console.WindowWidth - 1);
                        maxLines--;
                    }

                    await Task.Delay(500, cancel.Token);
                }
            });
            ReadLine();
            cancel.Cancel();
            try { Task.WaitAll(task, broadcast); } catch { }
            Console.WriteLine();
            Console.CursorVisible = true;
            return true;
        }


        protected internal override void OnStop()
        {
            base.OnStop();
            Stop();
        }


        public void OpenWallet(string path, string password)
        {
            if (!File.Exists(path))
            {
                throw new FileNotFoundException();
            }

            NEP6Wallet nep6wallet = new NEP6Wallet(path, password, CliSettings.Default.Protocol);
            CurrentWallet = nep6wallet;

        }


        private void WriteBlocks(uint start, uint count, string path, bool writeStart)
        {
            uint end = start + count - 1;
            using FileStream fs = new FileStream(path, FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.None, 4096, FileOptions.WriteThrough);
            if (fs.Length > 0)
            {
                byte[] buffer = new byte[sizeof(uint)];
                if (writeStart)
                {
                    fs.Seek(sizeof(uint), SeekOrigin.Begin);
                    fs.Read(buffer, 0, buffer.Length);
                    start += BitConverter.ToUInt32(buffer, 0);
                    fs.Seek(sizeof(uint), SeekOrigin.Begin);
                }
                else
                {
                    fs.Read(buffer, 0, buffer.Length);
                    start = BitConverter.ToUInt32(buffer, 0);
                    fs.Seek(0, SeekOrigin.Begin);
                }
            }
            else
            {
                if (writeStart)
                {
                    fs.Write(BitConverter.GetBytes(start), 0, sizeof(uint));
                }
            }
            if (start <= end)
                fs.Write(BitConverter.GetBytes(count), 0, sizeof(uint));
            fs.Seek(0, SeekOrigin.End);
            Console.WriteLine("Export block from " + start + " to " + end);

            using (var percent = new ConsolePercent(start, end))
            {
                for (uint i = start; i <= end; i++)
                {
                    Block block = NativeContract.Ledger.GetBlock(NeoSystem.StoreView, i);
                    byte[] array = block.ToArray();
                    fs.Write(BitConverter.GetBytes(array.Length), 0, sizeof(int));
                    fs.Write(array, 0, array.Length);
                    percent.Value = i;
                }
            }
        }

        private static void WriteLineWithoutFlicker(string message = "", int maxWidth = 80)
        {
            if (message.Length > 0) Console.Write(message);
            var spacesToErase = maxWidth - message.Length;
            if (spacesToErase < 0) spacesToErase = 0;
            Console.WriteLine(new string(' ', spacesToErase));
        }
    }
}
