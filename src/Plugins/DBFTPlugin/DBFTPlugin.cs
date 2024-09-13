// Copyright (C) 2021-2024 EpicChain Labs.

//
// DBFTPlugin.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using EpicChain.IEventHandlers;
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Plugins.DBFTPlugin.Consensus;
using EpicChain.Wallets;

namespace EpicChain.Plugins.DBFTPlugin
{
    public class DBFTPlugin : Plugin, IServiceAddedHandler, IMessageReceivedHandler, IWalletChangedHandler
    {
        private IWalletProvider walletProvider;
        private IActorRef consensus;
        private bool started = false;
        private EpicChainSystem EpicChainSystem;
        private Settings settings;

        public override string Description => "Consensus plugin with dBFT algorithm.";

        public override string ConfigFile => System.IO.Path.Combine(RootPath, "DBFTPlugin.json");

        protected override UnhandledExceptionPolicy ExceptionPolicy => settings.ExceptionPolicy;

        public DBFTPlugin()
        {
            RemoteNode.MessageReceived += ((IMessageReceivedHandler)this).RemoteNode_MessageReceived_Handler;
        }

        public DBFTPlugin(Settings settings) : this()
        {
            this.settings = settings;
        }

        public override void Dispose()
        {
            RemoteNode.MessageReceived -= ((IMessageReceivedHandler)this).RemoteNode_MessageReceived_Handler;
        }

        protected override void Configure()
        {
            settings ??= new Settings(GetConfiguration());
        }

        protected override void OnSystemLoaded(EpicChainSystem system)
        {
            if (system.Settings.Network != settings.Network) return;
            EpicChainSystem = system;
            EpicChainSystem.ServiceAdded += ((IServiceAddedHandler)this).EpicChainSystem_ServiceAdded_Handler;
        }

        void IServiceAddedHandler.EpicChainSystem_ServiceAdded_Handler(object sender, object service)
        {
            if (service is not IWalletProvider provider) return;
            walletProvider = provider;
            EpicChainSystem.ServiceAdded -= ((IServiceAddedHandler)this).EpicChainSystem_ServiceAdded_Handler;
            if (settings.AutoStart)
            {
                walletProvider.WalletChanged += ((IWalletChangedHandler)this).IWalletProvider_WalletChanged_Handler;
            }
        }

        void IWalletChangedHandler.IWalletProvider_WalletChanged_Handler(object sender, Wallet wallet)
        {
            walletProvider.WalletChanged -= ((IWalletChangedHandler)this).IWalletProvider_WalletChanged_Handler;
            Start(wallet);
        }

        [ConsoleCommand("start consensus", Category = "Consensus", Description = "Start consensus service (dBFT)")]
        private void OnStart()
        {
            Start(walletProvider.GetWallet());
        }

        public void Start(Wallet wallet)
        {
            if (started) return;
            started = true;
            consensus = EpicChainSystem.ActorSystem.ActorOf(ConsensusService.Props(EpicChainSystem, settings, wallet));
            consensus.Tell(new ConsensusService.Start());
        }

        bool IMessageReceivedHandler.RemoteNode_MessageReceived_Handler(EpicChainSystem system, Message message)
        {
            if (message.Command == MessageCommand.Transaction)
            {
                Transaction tx = (Transaction)message.Payload;
                if (tx.SystemFee > settings.MaxBlockSystemFee)
                    return false;
                consensus?.Tell(tx);
            }
            return true;
        }
    }
}
