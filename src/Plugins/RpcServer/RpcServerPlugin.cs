// Copyright (C) 2021-2024 EpicChain Labs.

//
// RpcServerPlugin.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Collections.Generic;
using System.Linq;

namespace EpicChain.Plugins.RpcServer
{
    public class RpcServerPlugin : Plugin
    {
        public override string Name => "RpcServer";
        public override string Description => "Enables RPC for the node";

        private Settings settings;
        private static readonly Dictionary<uint, RpcServer> servers = new();
        private static readonly Dictionary<uint, List<object>> handlers = new();

        public override string ConfigFile => System.IO.Path.Combine(RootPath, "RpcServer.json");
        protected override UnhandledExceptionPolicy ExceptionPolicy => settings.ExceptionPolicy;

        protected override void Configure()
        {
            settings = new Settings(GetConfiguration());
            foreach (RpcServerSettings s in settings.Servers)
                if (servers.TryGetValue(s.Network, out RpcServer server))
                    server.UpdateSettings(s);
        }

        public override void Dispose()
        {
            foreach (var (_, server) in servers)
                server.Dispose();
            base.Dispose();
        }

        protected override void OnSystemLoaded(EpicChainSystem system)
        {
            RpcServerSettings s = settings.Servers.FirstOrDefault(p => p.Network == system.Settings.Network);
            if (s is null) return;

            if (s.EnableCors && string.IsNullOrEmpty(s.RpcUser) == false && s.AllowOrigins.Length == 0)
            {
                Log("RcpServer: CORS is misconfigured!", LogLevel.Warning);
                Log($"You have {nameof(s.EnableCors)} and Basic Authentication enabled but " +
                $"{nameof(s.AllowOrigins)} is empty in config.json for RcpServer. " +
                "You must add url origins to the list to have CORS work from " +
                $"browser with basic authentication enabled. " +
                $"Example: \"AllowOrigins\": [\"http://{s.BindAddress}:{s.Port}\"]", LogLevel.Info);
            }

            RpcServer rpcRpcServer = new(system, s);

            if (handlers.Remove(s.Network, out var list))
            {
                foreach (var handler in list)
                {
                    rpcRpcServer.RegisterMethods(handler);
                }
            }

            rpcRpcServer.StartRpcServer();
            servers.TryAdd(s.Network, rpcRpcServer);
        }

        public static void RegisterMethods(object handler, uint network)
        {
            if (servers.TryGetValue(network, out RpcServer server))
            {
                server.RegisterMethods(handler);
                return;
            }
            if (!handlers.TryGetValue(network, out var list))
            {
                list = new List<object>();
                handlers.Add(network, list);
            }
            list.Add(handler);
        }
    }
}
