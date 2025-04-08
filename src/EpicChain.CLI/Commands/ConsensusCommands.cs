using System;
using System.Threading.Tasks;
using EpicChain.Consensus;
using McMaster.Extensions.CommandLineUtils;
using Microsoft.Extensions.DependencyInjection;

namespace EpicChain.CLI.Commands
{
    [Command("consensus", Description = "Consensus management commands")]
    [Subcommand(typeof(StartConsensusCommand), typeof(StopConsensusCommand), typeof(StatusCommand), typeof(ValidatorCommand))]
    public class ConsensusCommands
    {
        private readonly IServiceProvider _serviceProvider;

        public ConsensusCommands(IServiceProvider serviceProvider)
        {
            _serviceProvider = serviceProvider;
        }

        [Command("start", Description = "Start the consensus process")]
        public class StartConsensusCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public StartConsensusCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-v|--view", Description = "Start with specific view number")]
            public uint? ViewNumber { get; }

            [Option("-f|--force", Description = "Force start consensus")]
            public bool Force { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    _console.WriteLine("Starting consensus process...");

                    var consensus = _serviceProvider.GetRequiredService<ConsensusEngine>();
                    await consensus.StartAsync();

                    _console.WriteLine("Consensus process started successfully");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error starting consensus: {ex.Message}");
                }
            }
        }

        [Command("stop", Description = "Stop the consensus process")]
        public class StopConsensusCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public StopConsensusCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-f|--force", Description = "Force stop consensus")]
            public bool Force { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    _console.WriteLine("Stopping consensus process...");

                    var consensus = _serviceProvider.GetRequiredService<ConsensusEngine>();
                    consensus.Stop();

                    _console.WriteLine("Consensus process stopped successfully");
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error stopping consensus: {ex.Message}");
                }
            }
        }

        [Command("status", Description = "Show consensus status")]
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
                    var consensus = _serviceProvider.GetRequiredService<ConsensusEngine>();
                    var state = _serviceProvider.GetRequiredService<IConsensusState>();

                    _console.WriteLine("Consensus Status:");
                    _console.WriteLine($"Running: {consensus.IsRunning}");
                    _console.WriteLine($"View Number: {state.ViewNumber}");
                    _console.WriteLine($"Primary Validator: {state.PrimaryValidator}");
                    _console.WriteLine($"Validator Count: {state.Validators.Count}");

                    if (Verbose)
                    {
                        _console.WriteLine("\nDetailed Status:");
                        _console.WriteLine($"Current Block: {state.CurrentBlock?.Index}");
                        _console.WriteLine($"Vote Count: {state.GetVoteCount()}");
                        _console.WriteLine($"Can Commit: {state.CanCommit()}");
                        _console.WriteLine($"Last Block Time: {state.GetLastBlockTimestamp()}");
                        _console.WriteLine($"Timeout: {state.GetTimeout()}");
                    }
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error getting consensus status: {ex.Message}");
                }
            }
        }

        [Command("validator", Description = "Validator management commands")]
        public class ValidatorCommand
        {
            private readonly IServiceProvider _serviceProvider;
            private readonly IConsole _console;

            public ValidatorCommand(IServiceProvider serviceProvider, IConsole console)
            {
                _serviceProvider = serviceProvider;
                _console = console;
            }

            [Option("-r|--register", Description = "Register as a validator")]
            public bool Register { get; }

            [Option("-u|--unregister", Description = "Unregister as a validator")]
            public bool Unregister { get; }

            [Option("-p|--power", Description = "Set voting power")]
            public long? VotingPower { get; }

            public async Task OnExecuteAsync()
            {
                try
                {
                    var consensus = _serviceProvider.GetRequiredService<ConsensusEngine>();
                    var validator = _serviceProvider.GetRequiredService<IValidator>();

                    if (Register)
                    {
                        consensus.RegisterValidator(validator);
                        _console.WriteLine("Successfully registered as validator");
                    }
                    else if (Unregister)
                    {
                        consensus.UnregisterValidator(validator.Address);
                        _console.WriteLine("Successfully unregistered as validator");
                    }
                    else if (VotingPower.HasValue)
                    {
                        await consensus.UpdateValidatorPowerAsync(validator.Address, VotingPower.Value);
                        _console.WriteLine($"Updated voting power to {VotingPower.Value}");
                    }
                    else
                    {
                        _console.WriteLine("Validator Status:");
                        _console.WriteLine($"Is Validator: {validator.IsValidator}");
                        _console.WriteLine($"Address: {validator.Address}");
                        _console.WriteLine($"Voting Power: {validator.VotingPower}");
                    }
                }
                catch (Exception ex)
                {
                    _console.Error.WriteLine($"Error managing validator: {ex.Message}");
                }
            }
        }
    }
} 