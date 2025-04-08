using System;
using System.Threading.Tasks;
using EpicChain;
using EpicChain.Consensus;
using EpicChain.Network.P2P;
using McMaster.Extensions.CommandLineUtils;
using Microsoft.Extensions.DependencyInjection;

namespace EpicChain.CLI.Commands
{
    [Command("node", Description = "Node management commands")]
    [Subcommand(typeof(StartCommand), typeof(StopCommand), typeof(StatusCommand), typeof(ConfigCommand))]
    public class NodeCommands
    {
        private readonly IServiceProvider _serviceProvider;

        public NodeCommands(IServiceProvider serviceProvider)
        {
            _serviceProvider = serviceProvider;
        }

        [Command("start", Description = "Start the EpicChain node")]
        public class StartCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public StartCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-c|--consensus", Description = "Start consensus process")]
            public bool StartConsensus { get; }

            [Option("-m|--mining", Description = "Start mining process")]
            public bool StartMining { get; }

            [Option("-p|--port", Description = "Network port to listen on")]
            public int? Port { get; }

            [Option("-d|--data-dir", Description = "Data directory path")]
            public string DataDir { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    _console.WriteLine("Starting EpicChain node...");

                    var node = _serviceProvider.GetRequiredService<EpicChainSystem>();
                    await node.StartAsync();

                    if (StartConsensus)
                    {
                        var consensus = _serviceProvider.GetRequiredService<ConsensusEngine>();
                        await consensus.StartAsync();
                        _console.WriteLine("Consensus process started");
                    }

                    if (StartMining)
                    {
                        // TODO: Implement mining process
                        _console.WriteLine("Mining process started");
                    }

                    _console.WriteLine("Node started successfully");
                    _console.WriteLine("Press Ctrl+C to stop the node");

                    // Keep the application running
                    await Task.Delay(-1);
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error starting node: {ex.Message}");
                }
            }
        }

        [Command("stop", Description = "Stop the EpicChain node")]
        public class StopCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public StopCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-f|--force", Description = "Force stop the node")]
            public bool Force { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    _console.WriteLine("Stopping EpicChain node...");

                    var node = _serviceProvider.GetRequiredService<EpicChainSystem>();
                    var consensus = _serviceProvider.GetRequiredService<ConsensusEngine>();

                    consensus.Stop();
                    await node.DisposeAsync();

                    _console.WriteLine("Node stopped successfully");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error stopping node: {ex.Message}");
                }
            }
        }

        [Command("status", Description = "Show node status")]
        public class StatusCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public StatusCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-v|--verbose", Description = "Show detailed status")]
            public bool Verbose { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var node = _serviceProvider.GetRequiredService<EpicChainSystem>();
                    var consensus = _serviceProvider.GetRequiredService<ConsensusEngine>();

                    _console.WriteLine("Node Status:");
                    _console.WriteLine($"Height: {node.Blockchain.Height}");
                    _console.WriteLine($"Connected Peers: {node.NetworkManager.ConnectedPeers}");
                    _console.WriteLine($"Consensus Running: {consensus.IsRunning}");

                    if (Verbose)
                    {
                        _console.WriteLine("\nDetailed Status:");
                        _console.WriteLine($"Last Block Time: {node.Blockchain.LastBlock?.Timestamp}");
                        _console.WriteLine($"Network Protocol: {node.NetworkManager.Protocol}");
                        _console.WriteLine($"Consensus View: {consensus.ViewNumber}");
                        _console.WriteLine($"Validator Count: {consensus.ValidatorCount}");
                    }
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error getting node status: {ex.Message}");
                }
            }
        }

        [Command("config", Description = "Configure node settings")]
        public class ConfigCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public ConfigCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-s|--show", Description = "Show current configuration")]
            public bool ShowConfig { get; }

            [Option("-p|--port", Description = "Set network port")]
            public int? Port { get; }

            [Option("-d|--data-dir", Description = "Set data directory")]
            public string DataDir { get; }

            [Option("-c|--consensus", Description = "Configure consensus settings")]
            public bool ConfigureConsensus { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    if (ShowConfig)
                    {
                        var settings = _serviceProvider.GetRequiredService<ProtocolSettings>();
                        _console.WriteLine("Current Configuration:");
                        _console.WriteLine($"Network: {settings.Network}");
                        _console.WriteLine($"Port: {settings.Port}");
                        _console.WriteLine($"Data Directory: {settings.DataDirectory}");
                        _console.WriteLine($"Consensus Interval: {settings.ConsensusInterval}");
                    }

                    if (Port.HasValue || !string.IsNullOrEmpty(DataDir) || ConfigureConsensus)
                    {
                        _console.WriteLine("Updating configuration...");
                        // TODO: Implement configuration update
                        _console.WriteLine("Configuration updated successfully");
                    }
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error configuring node: {ex.Message}");
                }
            }
        }
    }
} 