// Copyright (C) 2021-2024 The EpicChain Labs.
//
// RpcServerPlugin.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System.Collections.Generic;
using System.Linq;

namespace Neo.Plugins.RpcServer
{
    public class RpcServerPlugin : Plugin
    {
        public override string Name => "RpcServer";
        public override string Description => "Enables RPC for the node";

        private Settings settings;
        private static readonly Dictionary<uint, RpcServer> servers = new();
        private static readonly Dictionary<uint, List<object>> handlers = new();

        public override string ConfigFile => System.IO.Path.Combine(RootPath, "RpcServer.json");

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

        protected override void OnSystemLoaded(NeoSystem system)
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
