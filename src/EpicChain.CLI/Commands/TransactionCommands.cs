using System;
using System.Threading.Tasks;
using EpicChain;
using EpicChain.Network.P2P.Payloads;
using McMaster.Extensions.CommandLineUtils;
using Microsoft.Extensions.DependencyInjection;

namespace EpicChain.CLI.Commands
{
    [Command("tx", Description = "Transaction management commands")]
    [Subcommand(typeof(SendCommand), typeof(GetCommand), typeof(ListCommand), typeof(SignCommand))]
    public class TransactionCommands
    {
        private readonly IServiceProvider _serviceProvider;

        public TransactionCommands(IServiceProvider serviceProvider)
        {
            _serviceProvider = serviceProvider;
        }

        [Command("send", Description = "Send a transaction")]
        public class SendCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public SendCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-f|--from", Description = "Sender address")]
            public string FromAddress { get; }

            [Option("-t|--to", Description = "Recipient address")]
            public string ToAddress { get; }

            [Option("-a|--amount", Description = "Amount to send")]
            public decimal Amount { get; }

            [Option("-p|--password", Description = "Wallet password")]
            public string Password { get; }

            [Option("-d|--data", Description = "Transaction data")]
            public string Data { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var walletManager = _serviceProvider.GetRequiredService<IWalletManager>();
                    var wallet = await walletManager.GetWalletAsync(FromAddress);
                    var tx = await wallet.CreateTransactionAsync(ToAddress, Amount, Data);
                    await wallet.SignTransactionAsync(tx, Password);
                    
                    var node = _serviceProvider.GetRequiredService<EpicChainSystem>();
                    await node.BroadcastTransactionAsync(tx);

                    _console.WriteLine("Transaction sent successfully:");
                    _console.WriteLine($"Hash: {tx.Hash}");
                    _console.WriteLine($"From: {FromAddress}");
                    _console.WriteLine($"To: {ToAddress}");
                    _console.WriteLine($"Amount: {Amount}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error sending transaction: {ex.Message}");
                }
            }
        }

        [Command("get", Description = "Get transaction details")]
        public class GetCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public GetCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-h|--hash", Description = "Transaction hash")]
            public string Hash { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var node = _serviceProvider.GetRequiredService<EpicChainSystem>();
                    var tx = await node.GetTransactionAsync(Hash);

                    _console.WriteLine("Transaction Details:");
                    _console.WriteLine($"Hash: {tx.Hash}");
                    _console.WriteLine($"From: {tx.Sender}");
                    _console.WriteLine($"To: {tx.Recipient}");
                    _console.WriteLine($"Amount: {tx.Amount}");
                    _console.WriteLine($"Status: {tx.Status}");
                    _console.WriteLine($"Timestamp: {tx.Timestamp}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error getting transaction: {ex.Message}");
                }
            }
        }

        [Command("list", Description = "List transactions")]
        public class ListCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public ListCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-a|--address", Description = "Address to filter by")]
            public string Address { get; }

            [Option("-s|--start", Description = "Start block height")]
            public uint? StartHeight { get; }

            [Option("-e|--end", Description = "End block height")]
            public uint? EndHeight { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var node = _serviceProvider.GetRequiredService<EpicChainSystem>();
                    var txs = await node.GetTransactionsAsync(Address, StartHeight, EndHeight);

                    _console.WriteLine("Transactions:");
                    foreach (var tx in txs)
                    {
                        _console.WriteLine($"Hash: {tx.Hash}");
                        _console.WriteLine($"From: {tx.Sender}");
                        _console.WriteLine($"To: {tx.Recipient}");
                        _console.WriteLine($"Amount: {tx.Amount}");
                        _console.WriteLine($"Status: {tx.Status}");
                        _console.WriteLine();
                    }
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error listing transactions: {ex.Message}");
                }
            }
        }

        [Command("sign", Description = "Sign a transaction")]
        public class SignCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public SignCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-h|--hash", Description = "Transaction hash")]
            public string Hash { get; }

            [Option("-a|--address", Description = "Signer address")]
            public string Address { get; }

            [Option("-p|--password", Description = "Wallet password")]
            public string Password { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var node = _serviceProvider.GetRequiredService<EpicChainSystem>();
                    var walletManager = _serviceProvider.GetRequiredService<IWalletManager>();
                    
                    var tx = await node.GetTransactionAsync(Hash);
                    var wallet = await walletManager.GetWalletAsync(Address);
                    await wallet.SignTransactionAsync(tx, Password);

                    _console.WriteLine("Transaction signed successfully");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error signing transaction: {ex.Message}");
                }
            }
        }
    }
} 