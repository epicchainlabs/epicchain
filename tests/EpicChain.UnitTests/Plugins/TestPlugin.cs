// Copyright (C) 2021-2024 EpicChain Labs.

//
// TestPlugin.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using Microsoft.Extensions.Configuration;
using EpicChain.Ledger;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.Plugins;
using System;
using System.Collections.Generic;

namespace EpicChain.UnitTests.Plugins
{

    internal class TestPluginSettings(IConfigurationSection section) : PluginSettings(section)
    {
        public static TestPluginSettings Default { get; private set; }
        public static void Load(IConfigurationSection section)
        {
            Default = new TestPluginSettings(section);
        }
    }
    internal class TestNonPlugin
    {
        public TestNonPlugin()
        {
            Blockchain.Committing += OnCommitting;
            Blockchain.Committed += OnCommitted;
        }

        private static void OnCommitting(EpicChainSystem system, Block block, DataCache snapshot, IReadOnlyList<Blockchain.ApplicationExecuted> applicationExecutedList)
        {
            throw new NotImplementedException("Test exception from OnCommitting");
        }

        private static void OnCommitted(EpicChainSystem system, Block block)
        {
            throw new NotImplementedException("Test exception from OnCommitted");
        }
    }


    internal class TestPlugin : Plugin
    {
        private readonly UnhandledExceptionPolicy _exceptionPolicy;
        protected internal override UnhandledExceptionPolicy ExceptionPolicy => _exceptionPolicy;

        public TestPlugin(UnhandledExceptionPolicy exceptionPolicy = UnhandledExceptionPolicy.StopPlugin)
        {
            Blockchain.Committing += OnCommitting;
            Blockchain.Committed += OnCommitted;
            _exceptionPolicy = exceptionPolicy;
        }

        protected override void Configure()
        {
            TestPluginSettings.Load(GetConfiguration());
        }

        public void LogMessage(string message)
        {
            Log(message);
        }

        public bool TestOnMessage(object message)
        {
            return OnMessage(message);
        }

        public IConfigurationSection TestGetConfiguration()
        {
            return GetConfiguration();
        }

        protected override bool OnMessage(object message) => true;

        private void OnCommitting(EpicChainSystem system, Block block, DataCache snapshot, IReadOnlyList<Blockchain.ApplicationExecuted> applicationExecutedList)
        {
            throw new NotImplementedException();
        }

        private void OnCommitted(EpicChainSystem system, Block block)
        {
            throw new NotImplementedException();
        }
    }
}
