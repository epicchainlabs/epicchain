using System;
using System.Threading.Tasks;
using EpicChain;
using EpicChain.Wallets;
using McMaster.Extensions.CommandLineUtils;
using Microsoft.Extensions.DependencyInjection;

namespace EpicChain.CLI.Commands
{
    [Command("wallet", Description = "Wallet management commands")]
    [Subcommand(typeof(CreateCommand), typeof(ImportCommand), typeof(ExportCommand), typeof(ListCommand), typeof(BalanceCommand))]
    public class WalletCommands
    {
        private readonly IServiceProvider _serviceProvider;

        public WalletCommands(IServiceProvider serviceProvider)
        {
            _serviceProvider = serviceProvider;
        }

        [Command("create", Description = "Create a new wallet")]
        public class CreateCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public CreateCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-n|--name", Description = "Wallet name")]
            public string Name { get; }

            [Option("-p|--password", Description = "Wallet password")]
            public string Password { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var walletManager = _serviceProvider.GetRequiredService<IWalletManager>();
                    var wallet = await walletManager.CreateWalletAsync(Name, Password);

                    _console.WriteLine("Wallet created successfully:");
                    _console.WriteLine($"Name: {wallet.Name}");
                    _console.WriteLine($"Address: {wallet.Address}");
                    _console.WriteLine($"Public Key: {wallet.PublicKey}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error creating wallet: {ex.Message}");
                }
            }
        }

        [Command("import", Description = "Import a wallet")]
        public class ImportCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public ImportCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-f|--file", Description = "Wallet file path")]
            public string FilePath { get; }

            [Option("-p|--password", Description = "Wallet password")]
            public string Password { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var walletManager = _serviceProvider.GetRequiredService<IWalletManager>();
                    var wallet = await walletManager.ImportWalletAsync(FilePath, Password);

                    _console.WriteLine("Wallet imported successfully:");
                    _console.WriteLine($"Name: {wallet.Name}");
                    _console.WriteLine($"Address: {wallet.Address}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error importing wallet: {ex.Message}");
                }
            }
        }

        [Command("export", Description = "Export a wallet")]
        public class ExportCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public ExportCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-a|--address", Description = "Wallet address")]
            public string Address { get; }

            [Option("-f|--file", Description = "Export file path")]
            public string FilePath { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var walletManager = _serviceProvider.GetRequiredService<IWalletManager>();
                    await walletManager.ExportWalletAsync(Address, FilePath);

                    _console.WriteLine($"Wallet exported successfully to {FilePath}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error exporting wallet: {ex.Message}");
                }
            }
        }

        [Command("list", Description = "List all wallets")]
        public class ListCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public ListCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var walletManager = _serviceProvider.GetRequiredService<IWalletManager>();
                    var wallets = await walletManager.GetWalletsAsync();

                    _console.WriteLine("Wallets:");
                    foreach (var wallet in wallets)
                    {
                        _console.WriteLine($"Name: {wallet.Name}");
                        _console.WriteLine($"Address: {wallet.Address}");
                        _console.WriteLine($"Balance: {await wallet.GetBalanceAsync()}");
                        _console.WriteLine();
                    }
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error listing wallets: {ex.Message}");
                }
            }
        }

        [Command("balance", Description = "Check wallet balance")]
        public class BalanceCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public BalanceCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-a|--address", Description = "Wallet address")]
            public string Address { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var walletManager = _serviceProvider.GetRequiredService<IWalletManager>();
                    var wallet = await walletManager.GetWalletAsync(Address);
                    var balance = await wallet.GetBalanceAsync();

                    _console.WriteLine($"Wallet Balance:");
                    _console.WriteLine($"Address: {wallet.Address}");
                    _console.WriteLine($"Balance: {balance}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error checking balance: {ex.Message}");
                }
            }
        }
    }
} 