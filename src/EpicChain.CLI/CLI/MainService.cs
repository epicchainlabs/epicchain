// Copyright (C) 2021-2024 EpicChain Labs.

//
// MainService.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Akka.Actor;
using EpicChain.ConsoleService;
using EpicChain.Cryptography.ECC;
using EpicChain.Extensions;
using EpicChain.IO;
using EpicChain.Json;
using EpicChain.Ledger;
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Plugins;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Manifest;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using EpicChain.VM.Types;
using EpicChain.Wallets;
using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Net;
using System.Numerics;
using System.Reflection;
using System.Text.RegularExpressions;
using System.Threading;
using System.Threading.Tasks;
using Array = System.Array;

namespace EpicChain.CLI
{
    public partial class MainService : ConsoleServiceBase, IWalletProvider
    {
        public event EventHandler<Wallet?>? WalletChanged = null;

        public const long TestModeEpicPulse = 20_00000000;

        private Wallet? _currentWallet;

        public Wallet? CurrentWallet
        {
            get => _currentWallet;
            private set
            {
                _currentWallet = value;
                WalletChanged?.Invoke(this, value);
            }
        }

        private EpicChainSystem? _EpicChainSystem;
        public EpicChainSystem EpicChainSystem
        {
            get => _EpicChainSystem!;
            private set => _EpicChainSystem = value;
        }

        private LocalNode? _localNode;

        public LocalNode LocalNode
        {
            get => _localNode!;
            private set => _localNode = value;
        }

        protected override string Prompt => "epic";
        public override string ServiceName => "EpicChain-CLI";

        /// <summary>
        /// Constructor
        /// </summary>
        public MainService() : base()
        {
            RegisterCommandHandler<string, UInt160>(false, str => StringToAddress(str, EpicChainSystem.Settings.AddressVersion));
            RegisterCommandHandler<string, UInt256>(false, UInt256.Parse);
            RegisterCommandHandler<string[], UInt256[]>(str => str.Select(u => UInt256.Parse(u.Trim())).ToArray());
            RegisterCommandHandler<string[], UInt160[]>(arr => arr.Select(str => StringToAddress(str, EpicChainSystem.Settings.AddressVersion)).ToArray());
            RegisterCommandHandler<string, ECPoint>(str => ECPoint.Parse(str.Trim(), ECCurve.Secp256r1));
            RegisterCommandHandler<string[], ECPoint[]>(str => str.Select(u => ECPoint.Parse(u.Trim(), ECCurve.Secp256r1)).ToArray());
            RegisterCommandHandler<string, JToken>(str => JToken.Parse(str)!);
            RegisterCommandHandler<string, JObject>(str => (JObject)JToken.Parse(str)!);
            RegisterCommandHandler<string, decimal>(str => decimal.Parse(str, CultureInfo.InvariantCulture));
            RegisterCommandHandler<JToken, JArray>(obj => (JArray)obj);

            RegisterCommand(this);

            Initialize_Logger();
        }

        internal UInt160 StringToAddress(string input, byte version)
        {
            switch (input.ToLowerInvariant())
            {
                case "epicchain": return NativeContract.EpicChain.Hash;
                case "epicpulse": return NativeContract.EpicPulse.Hash;
            }

            if (input.IndexOf('.') > 0 && input.LastIndexOf('.') < input.Length)
            {
                return ResolveEpicChainNameServiceAddress(input);
            }

            // Try to parse as UInt160

            if (UInt160.TryParse(input, out var addr))
            {
                return addr;
            }

            // Accept wallet format

            return input.ToScriptHash(version);
        }

        Wallet? IWalletProvider.GetWallet()
        {
            return CurrentWallet;
        }

        public override void RunConsole()
        {
            Console.ForegroundColor = ConsoleColor.DarkGreen;

            var cliV = Assembly.GetAssembly(typeof(Program))!.GetName().Version;
            var epicchainV = Assembly.GetAssembly(typeof(EpicChainSystem))!.GetName().Version;
            var vmV = Assembly.GetAssembly(typeof(ExecutionEngine))!.GetName().Version;
            Console.WriteLine($"EpicChain-BlockSphere V1.0.2 - Symbolizing a robust, all-encompassing");
            Console.WriteLine();

            base.RunConsole();
        }

        public void CreateWallet(string path, string password, bool createDefaultAccount = true)
        {
            Wallet wallet = Wallet.Create(null, path, password, EpicChainSystem.Settings);
            if (wallet == null)
            {
                ConsoleHelper.Warning("Wallet files in that format are not supported, please use a .json or .db3 file extension.");
                return;
            }
            if (createDefaultAccount)
            {
                WalletAccount account = wallet.CreateAccount();
                ConsoleHelper.Info("   Address: ", account.Address);
                ConsoleHelper.Info("    Pubkey: ", account.GetKey().PublicKey.EncodePoint(true).ToHexString());
                ConsoleHelper.Info("ScriptHash: ", $"{account.ScriptHash}");
            }
            wallet.Save();
            CurrentWallet = wallet;
        }

        private IEnumerable<Block> GetBlocks(Stream stream, bool read_start = false)
        {
            using BinaryReader r = new BinaryReader(stream);
            uint start = read_start ? r.ReadUInt32() : 0;
            uint count = r.ReadUInt32();
            uint end = start + count - 1;
            uint currentHeight = NativeContract.Ledger.CurrentIndex(EpicChainSystem.StoreView);
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
                using (FileStream fs = new(pathAcc, FileMode.Open, FileAccess.Read, FileShare.Read))
                    foreach (var block in GetBlocks(fs))
                        yield return block;

            const string pathAccZip = pathAcc + ".zip";
            if (File.Exists(pathAccZip))
                using (FileStream fs = new(pathAccZip, FileMode.Open, FileAccess.Read, FileShare.Read))
                using (ZipArchive zip = new(fs, ZipArchiveMode.Read))
                using (Stream? zs = zip.GetEntry(pathAcc)?.Open())
                {
                    if (zs is not null)
                    {
                        foreach (var block in GetBlocks(zs))
                            yield return block;
                    }
                }

            var paths = Directory.EnumerateFiles(".", "chain.*.acc", SearchOption.TopDirectoryOnly).Concat(Directory.EnumerateFiles(".", "chain.*.acc.zip", SearchOption.TopDirectoryOnly)).Select(p => new
            {
                FileName = Path.GetFileName(p),
                Start = uint.Parse(Regex.Match(p, @"\d+").Value),
                IsCompressed = p.EndsWith(".zip")
            }).OrderBy(p => p.Start);

            uint height = NativeContract.Ledger.CurrentIndex(EpicChainSystem.StoreView);
            foreach (var path in paths)
            {
                if (path.Start > height + 1) break;
                if (path.IsCompressed)
                    using (FileStream fs = new(path.FileName, FileMode.Open, FileAccess.Read, FileShare.Read))
                    using (ZipArchive zip = new(fs, ZipArchiveMode.Read))
                    using (Stream? zs = zip.GetEntry(Path.GetFileNameWithoutExtension(path.FileName))?.Open())
                    {
                        if (zs is not null)
                        {
                            foreach (var block in GetBlocks(zs, true))
                                yield return block;
                        }
                    }
                else
                    using (FileStream fs = new(path.FileName, FileMode.Open, FileAccess.Read, FileShare.Read))
                        foreach (var block in GetBlocks(fs, true))
                            yield return block;
            }
        }

        private bool NoWallet()
        {
            if (CurrentWallet != null) return false;
            ConsoleHelper.Error("You have to open the wallet first.");
            return true;
        }

        private byte[] LoadDeploymentScript(string nefFilePath, string? manifestFilePath, JObject? data, out NefFile nef, out ContractManifest manifest)
        {
            if (string.IsNullOrEmpty(manifestFilePath))
            {
                manifestFilePath = Path.ChangeExtension(nefFilePath, ".manifest.json");
            }

            // Read manifest

            var info = new FileInfo(manifestFilePath);
            if (!info.Exists || info.Length >= Transaction.MaxTransactionSize)
            {
                throw new ArgumentException(nameof(manifestFilePath));
            }

            manifest = ContractManifest.Parse(File.ReadAllBytes(manifestFilePath));

            // Read nef

            info = new FileInfo(nefFilePath);
            if (!info.Exists || info.Length >= Transaction.MaxTransactionSize)
            {
                throw new ArgumentException(nameof(nefFilePath));
            }

            nef = File.ReadAllBytes(nefFilePath).AsSerializable<NefFile>();

            ContractParameter? dataParameter = null;
            if (data is not null)
                try
                {
                    dataParameter = ContractParameter.FromJson(data);
                }
                catch
                {
                    throw new FormatException("invalid data");
                }

            // Basic script checks
            nef.Script.IsScriptValid(manifest.Abi);

            // Build script

            using (ScriptBuilder sb = new ScriptBuilder())
            {
                if (dataParameter is not null)
                    sb.EmitDynamicCall(NativeContract.ContractManagement.Hash, "deploy", nef.ToArray(), manifest.ToJson().ToString(), dataParameter);
                else
                    sb.EmitDynamicCall(NativeContract.ContractManagement.Hash, "deploy", nef.ToArray(), manifest.ToJson().ToString());
                return sb.ToArray();
            }
        }

        private byte[] LoadUpdateScript(UInt160 scriptHash, string nefFilePath, string manifestFilePath, JObject? data, out NefFile nef, out ContractManifest manifest)
        {
            if (string.IsNullOrEmpty(manifestFilePath))
            {
                manifestFilePath = Path.ChangeExtension(nefFilePath, ".manifest.json");
            }

            // Read manifest

            var info = new FileInfo(manifestFilePath);
            if (!info.Exists || info.Length >= Transaction.MaxTransactionSize)
            {
                throw new ArgumentException(nameof(manifestFilePath));
            }

            manifest = ContractManifest.Parse(File.ReadAllBytes(manifestFilePath));

            // Read nef

            info = new FileInfo(nefFilePath);
            if (!info.Exists || info.Length >= Transaction.MaxTransactionSize)
            {
                throw new ArgumentException(nameof(nefFilePath));
            }

            nef = File.ReadAllBytes(nefFilePath).AsSerializable<NefFile>();

            ContractParameter? dataParameter = null;
            if (data is not null)
                try
                {
                    dataParameter = ContractParameter.FromJson(data);
                }
                catch
                {
                    throw new FormatException("invalid data");
                }

            // Basic script checks
            nef.Script.IsScriptValid(manifest.Abi);

            // Build script

            using (ScriptBuilder sb = new ScriptBuilder())
            {
                if (dataParameter is null)
                    sb.EmitDynamicCall(scriptHash, "update", nef.ToArray(), manifest.ToJson().ToString());
                else
                    sb.EmitDynamicCall(scriptHash, "update", nef.ToArray(), manifest.ToJson().ToString(), dataParameter);
                return sb.ToArray();
            }
        }

        public override bool OnStart(string[] args)
        {
            if (!base.OnStart(args)) return false;
            return OnStartWithCommandLine(args) != 1;
        }

        public override void OnStop()
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

            CurrentWallet = Wallet.Open(path, password, EpicChainSystem.Settings) ?? throw new NotSupportedException();
        }

        public async void Start(CommandLineOptions options)
        {
            if (EpicChainSystem != null) return;
            bool verifyImport = !(options.NoVerify ?? false);

            ProtocolSettings protocol = ProtocolSettings.Load("config.json");
            CustomProtocolSettings(options, protocol);
            CustomApplicationSettings(options, Settings.Default);
            try
            {
                EpicChainSystem = new EpicChainSystem(protocol, Settings.Default.Storage.Engine,
                    string.Format(Settings.Default.Storage.Path, protocol.Network.ToString("X8")));
            }
            catch (DllNotFoundException ex) when (ex.Message.Contains("libleveldb"))
            {
                if (OperatingSystem.IsWindows())
                {
                    if (File.Exists("libleveldb.dll"))
                    {
                        DisplayError("Dependency DLL not found, please install Microsoft Visual C++ Redistributable.",
                            "See https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist");
                    }
                    else
                    {
                        DisplayError("DLL not found, please get libleveldb.dll.",
                            "Download from https://github.com/xmoohad/leveldb/releases");
                    }
                }
                else if (OperatingSystem.IsLinux())
                {
                    DisplayError("Shared library libleveldb.so not found, please get libleveldb.so.",
                        "Use command \"sudo apt-get install libleveldb-dev\" in terminal or download from https://github.com/xmoohad/leveldb/releases");
                }
                else if (OperatingSystem.IsMacOS() || OperatingSystem.IsMacCatalyst())
                {
                    DisplayError("Shared library libleveldb.dylib not found, please get libleveldb.dylib.",
                        "Use command \"brew install leveldb\" in terminal or download from https://github.com/xmoohad/leveldb/releases");
                }
                else
                {
                    DisplayError("EpicChain CLI is broken, please reinstall it.",
                        "Download from https://github.com/epicchainlabs/epicchain/releases");
                }
                return;
            }
            catch (DllNotFoundException)
            {
                DisplayError("EpicChain CLI is broken, please reinstall it.",
                    "Download from https://github.com/epicchainlabs/epicchain/releases");
                return;
            }

            EpicChainSystem.AddService(this);

            LocalNode = EpicChainSystem.LocalNode.Ask<LocalNode>(new LocalNode.GetInstance()).Result;

            // installing plugins
            var installTasks = options.Plugins?.Select(p => p).Where(p => !string.IsNullOrEmpty(p)).ToList().Select(p => InstallPluginAsync(p));
            if (installTasks is not null)
            {
                await Task.WhenAll(installTasks);
            }
            foreach (var plugin in Plugin.Plugins)
            {
                // Register plugins commands

                RegisterCommand(plugin, plugin.Name);
            }

            using (IEnumerator<Block> blocksBeingImported = GetBlocksFromFile().GetEnumerator())
            {
                while (true)
                {
                    List<Block> blocksToImport = new List<Block>();
                    for (int i = 0; i < 10; i++)
                    {
                        if (!blocksBeingImported.MoveNext()) break;
                        blocksToImport.Add(blocksBeingImported.Current);
                    }
                    if (blocksToImport.Count == 0) break;
                    await EpicChainSystem.Blockchain.Ask<Blockchain.ImportCompleted>(new Blockchain.Import
                    {
                        Blocks = blocksToImport,
                        Verify = verifyImport
                    });
                    if (EpicChainSystem is null) return;
                }
            }
            EpicChainSystem.StartNode(new ChannelsConfig
            {
                Tcp = new IPEndPoint(IPAddress.Any, Settings.Default.P2P.Port),
                MinDesiredConnections = Settings.Default.P2P.MinDesiredConnections,
                MaxConnections = Settings.Default.P2P.MaxConnections,
                MaxConnectionsPerAddress = Settings.Default.P2P.MaxConnectionsPerAddress
            });

            if (Settings.Default.UnlockWallet.IsActive)
            {
                try
                {
                    if (Settings.Default.UnlockWallet.Path is null)
                    {
                        throw new InvalidOperationException("UnlockWallet.Path must be defined");
                    }
                    else if (Settings.Default.UnlockWallet.Password is null)
                    {
                        throw new InvalidOperationException("UnlockWallet.Password must be defined");
                    }

                    OpenWallet(Settings.Default.UnlockWallet.Path, Settings.Default.UnlockWallet.Password);
                }
                catch (FileNotFoundException)
                {
                    ConsoleHelper.Warning($"wallet file \"{Settings.Default.UnlockWallet.Path}\" not found.");
                }
                catch (System.Security.Cryptography.CryptographicException)
                {
                    ConsoleHelper.Error($"Failed to open file \"{Settings.Default.UnlockWallet.Path}\"");
                }
                catch (Exception ex)
                {
                    ConsoleHelper.Error(ex.GetBaseException().Message);
                }
            }

            return;

            void DisplayError(string primaryMessage, string? secondaryMessage = null)
            {
                ConsoleHelper.Error(primaryMessage + Environment.NewLine +
                                    (secondaryMessage != null ? secondaryMessage + Environment.NewLine : "") +
                                    "Press any key to exit.");
                Console.ReadKey();
                Environment.Exit(-1);
            }
        }

        public void Stop()
        {
            Dispose_Logger();
            Interlocked.Exchange(ref _EpicChainSystem, null)?.Dispose();
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
                    Block block = NativeContract.Ledger.GetBlock(EpicChainSystem.StoreView, i);
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

        /// <summary>
        /// Make and send transaction with script, sender
        /// </summary>
        /// <param name="script">script</param>
        /// <param name="account">sender</param>
        /// <param name="datoshi">Max fee for running the script, in the unit of datoshi, 1 datoshi = 1e-8 EpicPulse</param>
        private void SendTransaction(byte[] script, UInt160? account = null, long datoshi = TestModeEpicPulse)
        {
            if (NoWallet()) return;

            Signer[] signers = Array.Empty<Signer>();
            var snapshot = EpicChainSystem.StoreView;

            if (account != null)
            {
                signers = CurrentWallet!.GetAccounts()
                .Where(p => !p.Lock && !p.WatchOnly && p.ScriptHash == account && NativeContract.EpicPulse.BalanceOf(snapshot, p.ScriptHash).Sign > 0)
                .Select(p => new Signer { Account = p.ScriptHash, Scopes = WitnessScope.CalledByEntry })
                .ToArray();
            }

            try
            {
                Transaction tx = CurrentWallet!.MakeTransaction(snapshot, script, account, signers, maxEpicPulse: datoshi);
                ConsoleHelper.Info("Invoking script with: ", $"'{Convert.ToBase64String(tx.Script.Span)}'");

                using (ApplicationEngine engine = ApplicationEngine.Run(tx.Script, snapshot, container: tx, settings: EpicChainSystem.Settings, epicpulse: datoshi))
                {
                    PrintExecutionOutput(engine, true);
                    if (engine.State == VMState.FAULT) return;
                }

                if (!ConsoleHelper.ReadUserInput("Relay tx(no|yes)").IsYes())
                {
                    return;
                }

                SignAndSendTx(EpicChainSystem.StoreView, tx);
            }
            catch (InvalidOperationException e)
            {
                ConsoleHelper.Error(GetExceptionMessage(e));
            }
        }

        /// <summary>
        /// Process "invoke" command
        /// </summary>
        /// <param name="scriptHash">Script hash</param>
        /// <param name="operation">Operation</param>
        /// <param name="result">Result</param>
        /// <param name="verifiable">Transaction</param>
        /// <param name="contractParameters">Contract parameters</param>
        /// <param name="showStack">Show result stack if it is true</param>
        /// <param name="datoshi">Max fee for running the script, in the unit of datoshi, 1 datoshi = 1e-8 EpicPulse</param>
        /// <returns>Return true if it was successful</returns>
        private bool OnInvokeWithResult(UInt160 scriptHash, string operation, out StackItem result, IVerifiable? verifiable = null, JArray? contractParameters = null, bool showStack = true, long datoshi = TestModeEpicPulse)
        {
            List<ContractParameter> parameters = new();

            if (contractParameters != null)
            {
                foreach (var contractParameter in contractParameters)
                {
                    if (contractParameter is not null)
                    {
                        parameters.Add(ContractParameter.FromJson((JObject)contractParameter));
                    }
                }
            }

            ContractState contract = NativeContract.ContractManagement.GetContract(EpicChainSystem.StoreView, scriptHash);
            if (contract == null)
            {
                ConsoleHelper.Error("Contract does not exist.");
                result = StackItem.Null;
                return false;
            }
            else
            {
                if (contract.Manifest.Abi.GetMethod(operation, parameters.Count) == null)
                {
                    ConsoleHelper.Error("This method does not not exist in this contract.");
                    result = StackItem.Null;
                    return false;
                }
            }

            byte[] script;

            using (ScriptBuilder scriptBuilder = new ScriptBuilder())
            {
                scriptBuilder.EmitDynamicCall(scriptHash, operation, parameters.ToArray());
                script = scriptBuilder.ToArray();
                ConsoleHelper.Info("Invoking script with: ", $"'{script.ToBase64String()}'");
            }

            if (verifiable is Transaction tx)
            {
                tx.Script = script;
            }

            using ApplicationEngine engine = ApplicationEngine.Run(script, EpicChainSystem.StoreView, container: verifiable, settings: EpicChainSystem.Settings, epicpulse: datoshi);
            PrintExecutionOutput(engine, showStack);
            result = engine.State == VMState.FAULT ? StackItem.Null : engine.ResultStack.Peek();
            return engine.State != VMState.FAULT;
        }

        private void PrintExecutionOutput(ApplicationEngine engine, bool showStack = true)
        {
            ConsoleHelper.Info("VM State: ", engine.State.ToString());
            ConsoleHelper.Info("EpicPulse Consumed: ", new BigDecimal((BigInteger)engine.FeeConsumed, NativeContract.EpicPulse.Decimals).ToString());

            if (showStack)
                ConsoleHelper.Info("Result Stack: ", new JArray(engine.ResultStack.Select(p => p.ToJson())).ToString());

            if (engine.State == VMState.FAULT)
                ConsoleHelper.Error(GetExceptionMessage(engine.FaultException));
        }

        static string GetExceptionMessage(Exception exception)
        {
            if (exception == null) return "Engine faulted.";

            if (exception.InnerException != null)
            {
                return GetExceptionMessage(exception.InnerException);
            }

            return exception.Message;
        }

        public UInt160 ResolveEpicChainNameServiceAddress(string domain)
        {
            if (Settings.Default.Contracts.EpicChainNameService == UInt160.Zero)
                throw new Exception("EpicChain Name Service (XNS): is disabled on this network.");

            using var sb = new ScriptBuilder();
            sb.EmitDynamicCall(Settings.Default.Contracts.EpicChainNameService, "resolve", CallFlags.ReadOnly, domain, 16);

            using var appEng = ApplicationEngine.Run(sb.ToArray(), EpicChainSystem.StoreView, settings: EpicChainSystem.Settings);
            if (appEng.State == VMState.HALT)
            {
                var data = appEng.ResultStack.Pop();
                if (data is ByteString)
                {
                    try
                    {
                        var addressData = data.GetString();
                        if (UInt160.TryParse(addressData, out var address))
                            return address;
                        else
                            return addressData.ToScriptHash(EpicChainSystem.Settings.AddressVersion);
                    }
                    catch { }
                }
                else if (data is Null)
                {
                    throw new Exception($"EpicChain Name Service (XNS): \"{domain}\" domain not found.");
                }
                throw new Exception("EpicChain Name Service (XNS): Record invalid address format.");
            }
            else
            {
                if (appEng.FaultException is not null)
                {
                    throw new Exception($"EpicChain Name Service (XNS): \"{appEng.FaultException.Message}\".");
                }
            }
            throw new Exception($"EpicChain Name Service (XNS): \"{domain}\" domain not found.");
        }
    }
}
