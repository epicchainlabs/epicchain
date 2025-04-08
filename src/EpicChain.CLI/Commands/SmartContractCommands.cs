using System;
using System.Threading.Tasks;
using EpicChain;
using EpicChain.SmartContract;
using McMaster.Extensions.CommandLineUtils;
using Microsoft.Extensions.DependencyInjection;

namespace EpicChain.CLI.Commands
{
    [Command("contract", Description = "Smart contract management commands")]
    [Subcommand(typeof(DeployCommand), typeof(InvokeCommand), typeof(GetCommand), typeof(ListCommand))]
    public class SmartContractCommands
    {
        private readonly IServiceProvider _serviceProvider;

        public SmartContractCommands(IServiceProvider serviceProvider)
        {
            _serviceProvider = serviceProvider;
        }

        [Command("deploy", Description = "Deploy a smart contract")]
        public class DeployCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public DeployCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-f|--file", Description = "Contract file path")]
            public string ContractFile { get; }

            [Option("-a|--address", Description = "Deployer address")]
            public string Address { get; }

            [Option("-p|--password", Description = "Wallet password")]
            public string Password { get; }

            [Option("-g|--gas", Description = "Gas limit")]
            public long GasLimit { get; } = 1000000;

            public async Task OnExecuteAsync()
            {
                try
                {
                    var contractManager = _serviceProvider.GetRequiredService<IContractManager>();
                    var walletManager = _serviceProvider.GetRequiredService<IWalletManager>();
                    
                    var wallet = await walletManager.GetWalletAsync(Address);
                    var contract = await System.IO.File.ReadAllTextAsync(ContractFile);
                    
                    var result = await contractManager.DeployContractAsync(wallet, contract, GasLimit, Password);

                    _console.WriteLine("Contract deployed successfully:");
                    _console.WriteLine($"Contract Address: {result.ContractAddress}");
                    _console.WriteLine($"Transaction Hash: {result.TransactionHash}");
                    _console.WriteLine($"Gas Used: {result.GasUsed}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error deploying contract: {ex.Message}");
                }
            }
        }

        [Command("invoke", Description = "Invoke a smart contract method")]
        public class InvokeCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public InvokeCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-c|--contract", Description = "Contract address")]
            public string ContractAddress { get; }

            [Option("-m|--method", Description = "Method name")]
            public string MethodName { get; }

            [Option("-a|--address", Description = "Caller address")]
            public string Address { get; }

            [Option("-p|--password", Description = "Wallet password")]
            public string Password { get; }

            [Option("-g|--gas", Description = "Gas limit")]
            public long GasLimit { get; } = 1000000;

            [Option("-d|--data", Description = "Method parameters (JSON)")]
            public string Parameters { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var contractManager = _serviceProvider.GetRequiredService<IContractManager>();
                    var walletManager = _serviceProvider.GetRequiredService<IWalletManager>();
                    
                    var wallet = await walletManager.GetWalletAsync(Address);
                    var result = await contractManager.InvokeContractAsync(
                        wallet, 
                        ContractAddress, 
                        MethodName, 
                        Parameters, 
                        GasLimit, 
                        Password
                    );

                    _console.WriteLine("Contract method invoked successfully:");
                    _console.WriteLine($"Transaction Hash: {result.TransactionHash}");
                    _console.WriteLine($"Gas Used: {result.GasUsed}");
                    _console.WriteLine($"Return Value: {result.ReturnValue}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error invoking contract: {ex.Message}");
                }
            }
        }

        [Command("get", Description = "Get contract information")]
        public class GetCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public GetCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-c|--contract", Description = "Contract address")]
            public string ContractAddress { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var contractManager = _serviceProvider.GetRequiredService<IContractManager>();
                    var contract = await contractManager.GetContractAsync(ContractAddress);

                    _console.WriteLine("Contract Information:");
                    _console.WriteLine($"Address: {contract.Address}");
                    _console.WriteLine($"Creator: {contract.Creator}");
                    _console.WriteLine($"Code Hash: {contract.CodeHash}");
                    _console.WriteLine($"Storage Hash: {contract.StorageHash}");
                    _console.WriteLine($"Methods: {string.Join(", ", contract.Methods)}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error getting contract: {ex.Message}");
                }
            }
        }

        [Command("list", Description = "List deployed contracts")]
        public class ListCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public ListCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-a|--address", Description = "Filter by creator address")]
            public string CreatorAddress { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var contractManager = _serviceProvider.GetRequiredService<IContractManager>();
                    var contracts = await contractManager.GetContractsAsync(CreatorAddress);

                    _console.WriteLine("Deployed Contracts:");
                    foreach (var contract in contracts)
                    {
                        _console.WriteLine($"Address: {contract.Address}");
                        _console.WriteLine($"Creator: {contract.Creator}");
                        _console.WriteLine($"Code Hash: {contract.CodeHash}");
                        _console.WriteLine();
                    }
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error listing contracts: {ex.Message}");
                }
            }
        }
    }
} 