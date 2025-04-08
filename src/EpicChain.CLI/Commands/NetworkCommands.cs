using System;
using System.Threading.Tasks;
using EpicChain;
using EpicChain.Network.P2P;
using McMaster.Extensions.CommandLineUtils;
using Microsoft.Extensions.DependencyInjection;

namespace EpicChain.CLI.Commands
{
    [Command("network", Description = "Network management commands")]
    [Subcommand(typeof(ConnectCommand), typeof(DisconnectCommand), typeof(ListCommand), typeof(InfoCommand))]
    public class NetworkCommands
    {
        private readonly IServiceProvider _serviceProvider;

        public NetworkCommands(IServiceProvider serviceProvider)
        {
            _serviceProvider = serviceProvider;
        }

        [Command("connect", Description = "Connect to a peer")]
        public class ConnectCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public ConnectCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-a|--address", Description = "Peer address")]
            public string Address { get; }

            [Option("-p|--port", Description = "Peer port")]
            public int Port { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var node = _serviceProvider.GetRequiredService<EpicChainSystem>();
                    await node.ConnectToPeerAsync(Address, Port);

                    _console.WriteLine($"Successfully connected to {Address}:{Port}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error connecting to peer: {ex.Message}");
                }
            }
        }

        [Command("disconnect", Description = "Disconnect from a peer")]
        public class DisconnectCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public DisconnectCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-a|--address", Description = "Peer address")]
            public string Address { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var node = _serviceProvider.GetRequiredService<EpicChainSystem>();
                    await node.DisconnectFromPeerAsync(Address);

                    _console.WriteLine($"Successfully disconnected from {Address}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error disconnecting from peer: {ex.Message}");
                }
            }
        }

        [Command("list", Description = "List connected peers")]
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
                    var node = _serviceProvider.GetRequiredService<EpicChainSystem>();
                    var peers = await node.GetConnectedPeersAsync();

                    _console.WriteLine("Connected Peers:");
                    foreach (var peer in peers)
                    {
                        _console.WriteLine($"Address: {peer.Address}");
                        _console.WriteLine($"Port: {peer.Port}");
                        _console.WriteLine($"Version: {peer.Version}");
                        _console.WriteLine($"Height: {peer.Height}");
                        _console.WriteLine();
                    }
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error listing peers: {ex.Message}");
                }
            }
        }

        [Command("info", Description = "Get network information")]
        public class InfoCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public InfoCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var node = _serviceProvider.GetRequiredService<EpicChainSystem>();
                    var info = await node.GetNetworkInfoAsync();

                    _console.WriteLine("Network Information:");
                    _console.WriteLine($"Node Version: {info.Version}");
                    _console.WriteLine($"Protocol Version: {info.ProtocolVersion}");
                    _console.WriteLine($"Connected Peers: {info.ConnectedPeers}");
                    _console.WriteLine($"Network Height: {info.NetworkHeight}");
                    _console.WriteLine($"Local Height: {info.LocalHeight}");
                    _console.WriteLine($"Network Time: {info.NetworkTime}");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error getting network info: {ex.Message}");
                }
            }
        }
    }
} 