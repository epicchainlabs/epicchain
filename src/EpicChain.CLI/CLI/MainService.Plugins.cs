// =============================================================================================
//  © Copyright (C) 2021-2025 EpicChain Labs. All rights reserved.
// =============================================================================================
//
//  File: MainService.Plugins.cs
//  Project: EpicChain Labs - Core Blockchain Infrastructure
//  Author: Xmoohad (Muhammad Ibrahim Muhammad)
//
// ---------------------------------------------------------------------------------------------
//  Description:
//  This file is an integral part of the EpicChain Labs ecosystem, a forward-looking, open-source
//  blockchain initiative founded by Xmoohad. The EpicChain project aims to create a robust,
//  decentralized, and developer-friendly blockchain infrastructure that empowers innovation,
//  transparency, and digital sovereignty.
//
// ---------------------------------------------------------------------------------------------
//  Licensing:
//  This file is distributed under the permissive MIT License, which grants anyone the freedom
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of this
//  software. These rights are granted with the understanding that the original license notice
//  and copyright attribution remain intact.
//
//  For the full license text, please refer to the LICENSE file included in the root directory of
//  this repository or visit the official MIT License page at:
//  ➤ https://opensource.org/licenses/MIT
//
// ---------------------------------------------------------------------------------------------
//  Community and Contribution:
//  EpicChain Labs is deeply rooted in the principles of open-source development. We believe that
//  collaboration, transparency, and inclusiveness are the cornerstones of sustainable technology.
//
//  This file, like all components of the EpicChain ecosystem, is offered to the global development
//  community to explore, extend, and improve. Whether you're fixing bugs, optimizing performance,
//  or building new features, your contributions are welcome and appreciated.
//
//  By contributing to this project, you become part of a community dedicated to shaping the future
//  of blockchain technology. Join us in our mission to create more secure, scalable, and accessible
//  digital infrastructure for all.
//
// ---------------------------------------------------------------------------------------------
//  Terms of Use:
//  Redistribution and usage of this file in both source and compiled (binary) forms—with or without
//  modification—are fully permitted under the MIT License. Users of this software are expected to
//  adhere to the simple and clear guidelines established in the LICENSE file.
//
//  By using this file and other components of the EpicChain Labs project, you acknowledge and agree
//  to the terms of the MIT License. This ensures that the ethos of free and open software development
//  continues to flourish and remain protected.
//
// ---------------------------------------------------------------------------------------------
//  Final Note:
//  EpicChain Labs remains committed to pushing the boundaries of blockchain innovation. Whether
//  you're an experienced developer, a researcher, a student, or simply a curious enthusiast, we
//  invite you to explore the possibilities of EpicChain—and contribute toward a decentralized future.
//
//  Learn more about the project, get involved, or access full documentation at:
//  ➤ https://epic-chain.org
//
// =============================================================================================



using Akka.Util.Internal;
using Microsoft.Extensions.Configuration;
using EpicChain.ConsoleService;
using EpicChain.Plugins;
using System;
using System.Collections.Generic;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Json;
using System.Reflection;
using System.Text.Json.Nodes;
using System.Threading.Tasks;

namespace EpicChain.CLI
{
    partial class MainService
    {
        /// <summary>
        /// Process "install" command
        /// </summary>
        /// <param name="pluginName">Plugin name</param>
        /// <param name="downloadUrl">Custom plugins download url, this is optional.</param>
        [ConsoleCommand("install", Category = "Plugin Commands")]
        private void OnInstallCommand(string pluginName, string? downloadUrl = null)
        {
            if (PluginExists(pluginName))
            {
                ConsoleHelper.Warning($"Plugin already exist.");
                return;
            }

            var result = InstallPluginAsync(pluginName, downloadUrl).GetAwaiter().GetResult();
            if (result)
            {
                var asmName = Assembly.GetExecutingAssembly().GetName().Name;
                ConsoleHelper.Info("", $"Install successful, please restart \"{asmName}\".");
            }
        }

        /// <summary>
        /// Force to install a plugin again. This will overwrite
        /// existing plugin files, in case of any file missing or
        /// damage to the old version.
        /// </summary>
        /// <param name="pluginName">name of the plugin</param>
        [ConsoleCommand("reinstall", Category = "Plugin Commands", Description = "Overwrite existing plugin by force.")]
        private void OnReinstallCommand(string pluginName)
        {
            var result = InstallPluginAsync(pluginName, overWrite: true).GetAwaiter().GetResult();
            if (result)
            {
                var asmName = Assembly.GetExecutingAssembly().GetName().Name;
                ConsoleHelper.Info("", $"Reinstall successful, please restart \"{asmName}\".");
            }
        }

        /// <summary>
        /// Download plugin from github release
        /// The function of download and install are divided
        /// for the consideration of `update` command that
        /// might be added in the future.
        /// </summary>
        /// <param name="pluginName">name of the plugin</param>
        /// <param name="pluginVersion"></param>
        /// <param name="customDownloadUrl">Custom plugin download url.</param>
        /// <param name="prerelease"></param>
        /// <returns>Downloaded content</returns>
        private static async Task<Stream> DownloadPluginAsync(string pluginName, Version pluginVersion, string? customDownloadUrl = null, bool prerelease = false)
        {
            ConsoleHelper.Info($"Downloading {pluginName} {pluginVersion}...");
            using var httpClient = new HttpClient();

            var asmName = Assembly.GetExecutingAssembly().GetName();
            httpClient.DefaultRequestHeaders.UserAgent.Add(new(asmName.Name!, asmName.Version!.ToString(3)));
            var url = customDownloadUrl == null ? Settings.Default.Plugins.DownloadUrl : new Uri(customDownloadUrl);
            var json = await httpClient.GetFromJsonAsync<JsonArray>(url) ?? throw new HttpRequestException($"Failed: {url}");
            var jsonRelease = json.AsArray()
                .SingleOrDefault(s =>
                    s != null &&
                    s["tag_name"]!.GetValue<string>() == $"v{pluginVersion.ToString(3)}" &&
                    s["prerelease"]!.GetValue<bool>() == prerelease) ?? throw new Exception($"Could not find Release {pluginVersion}");

            var jsonAssets = jsonRelease
                .AsObject()
                .SingleOrDefault(s => s.Key == "assets").Value ?? throw new Exception("Could not find any Plugins");

            var jsonPlugin = jsonAssets
                .AsArray()
                .SingleOrDefault(s =>
                    Path.GetFileNameWithoutExtension(
                        s!["name"]!.GetValue<string>()).Equals(pluginName, StringComparison.InvariantCultureIgnoreCase))
                ?? throw new Exception($"Could not find {pluginName}");

            var downloadUrl = jsonPlugin["browser_download_url"]!.GetValue<string>();
            return await httpClient.GetStreamAsync(downloadUrl);
        }

        /// <summary>
        /// Install plugin from stream
        /// </summary>
        /// <param name="pluginName">Name of the plugin</param>
        /// <param name="downloadUrl">Custom plugins download url.</param>
        /// <param name="installed">Dependency set</param>
        /// <param name="overWrite">Install by force for `update`</param>
        private async Task<bool> InstallPluginAsync(
            string pluginName,
            string? downloadUrl = null,
            HashSet<string>? installed = null,
            bool overWrite = false)
        {
            installed ??= new HashSet<string>();
            if (!installed.Add(pluginName)) return false;
            if (!overWrite && PluginExists(pluginName)) return false;

            try
            {

                using var stream = await DownloadPluginAsync(pluginName, Settings.Default.Plugins.Version, downloadUrl, Settings.Default.Plugins.Prerelease);

                using var zip = new ZipArchive(stream, ZipArchiveMode.Read);
                var entry = zip.Entries.FirstOrDefault(p => p.Name == "config.json");
                if (entry is not null)
                {
                    await using var es = entry.Open();
                    await InstallDependenciesAsync(es, installed, downloadUrl);
                }
                zip.ExtractToDirectory("./", true);
                return true;
            }
            catch (Exception ex)
            {
                ConsoleHelper.Error(ex?.InnerException?.Message ?? ex!.Message);
            }
            return false;
        }

        /// <summary>
        /// Install the dependency of the plugin
        /// </summary>
        /// <param name="config">plugin config path in temp</param>
        /// <param name="installed">Dependency set</param>
        /// <param name="downloadUrl">Custom plugin download url.</param>
        private async Task InstallDependenciesAsync(Stream config, HashSet<string> installed, string? downloadUrl = null)
        {
            var dependency = new ConfigurationBuilder()
                .AddJsonStream(config)
                .Build()
                .GetSection("Dependency");

            if (!dependency.Exists()) return;
            var dependencies = dependency.GetChildren().Select(p => p.Get<string>()).ToArray();
            if (dependencies.Length == 0) return;

            foreach (var plugin in dependencies.Where(p => p is not null && !PluginExists(p)))
            {
                ConsoleHelper.Info($"Installing dependency: {plugin}");
                await InstallPluginAsync(plugin!, downloadUrl, installed);
            }
        }

        /// <summary>
        /// Check that the plugin has all necessary files
        /// </summary>
        /// <param name="pluginName"> Name of the plugin</param>
        /// <returns></returns>
        private static bool PluginExists(string pluginName)
        {
            return Plugin.Plugins.Any(p => p.Name.Equals(pluginName, StringComparison.InvariantCultureIgnoreCase));
        }

        /// <summary>
        /// Process "uninstall" command
        /// </summary>
        /// <param name="pluginName">Plugin name</param>
        [ConsoleCommand("uninstall", Category = "Plugin Commands")]
        private void OnUnInstallCommand(string pluginName)
        {
            if (!PluginExists(pluginName))
            {
                ConsoleHelper.Error("Plugin not found");
                return;
            }

            foreach (var p in Plugin.Plugins)
            {
                try
                {
                    using var reader = File.OpenRead($"Plugins/{p.Name}/config.json");
                    if (new ConfigurationBuilder()
                        .AddJsonStream(reader)
                        .Build()
                        .GetSection("Dependency")
                        .GetChildren()
                        .Select(s => s.Get<string>())
                        .Any(a => a is not null && a.Equals(pluginName, StringComparison.InvariantCultureIgnoreCase)))
                    {
                        ConsoleHelper.Error($"{pluginName} is required by other plugins.");
                        ConsoleHelper.Info("Info: ", $"If plugin is damaged try to reinstall.");
                        return;
                    }
                }
                catch (Exception)
                {
                    // ignored
                }
            }
            try
            {
                Directory.Delete($"Plugins/{pluginName}", true);
            }
            catch (IOException) { }
            ConsoleHelper.Info("", "Uninstall successful, please restart epicchain-cli.");
        }

        /// <summary>
        /// Process "plugins" command
        /// </summary>
        [ConsoleCommand("plugins", Category = "Plugin Commands")]
        private void OnPluginsCommand()
        {
            try
            {
                var plugins = GetPluginListAsync().GetAwaiter().GetResult()?.ToArray() ?? [];
                var installedPlugins = Plugin.Plugins.ToList();

                var maxLength = installedPlugins.Count == 0 ? 0 : installedPlugins.Max(s => s.Name.Length);
                if (plugins.Length > 0)
                {
                    maxLength = Math.Max(maxLength, plugins.Max(s => s.Length));
                }

                plugins.Select(s => (name: s, installedPlugin: Plugin.Plugins.SingleOrDefault(pp => string.Equals(pp.Name, s, StringComparison.InvariantCultureIgnoreCase))))
                    .Concat(installedPlugins.Select(u => (name: u.Name, installedPlugin: (Plugin?)u)).Where(u => !plugins.Contains(u.name, StringComparer.InvariantCultureIgnoreCase)))
                    .OrderBy(u => u.name)
                    .ForEach((f) =>
                    {
                        if (f.installedPlugin != null)
                        {
                            var tabs = f.name.Length < maxLength ? "\t" : string.Empty;
                            ConsoleHelper.Info("", $"[Installed]\t {f.name,6}{tabs}", "  @", $"{f.installedPlugin.Version.ToString(3)}  {f.installedPlugin.Description}");
                        }
                        else
                            ConsoleHelper.Info($"[Not Installed]\t {f.name}");
                    });
            }
            catch (Exception ex)
            {
                ConsoleHelper.Error(ex!.InnerException?.Message ?? ex!.Message);
            }
        }

        private async Task<IEnumerable<string>> GetPluginListAsync()
        {
            using var httpClient = new HttpClient();

            var asmName = Assembly.GetExecutingAssembly().GetName();
            httpClient.DefaultRequestHeaders.UserAgent.Add(new(asmName.Name!, asmName.Version!.ToString(3)));

            var json = await httpClient.GetFromJsonAsync<JsonArray>(Settings.Default.Plugins.DownloadUrl)
                ?? throw new HttpRequestException($"Failed: {Settings.Default.Plugins.DownloadUrl}");
            return json.AsArray()
                .Where(w =>
                    w != null &&
                    w["tag_name"]!.GetValue<string>() == $"v{Settings.Default.Plugins.Version.ToString(3)}")
                .SelectMany(s => s!["assets"]!.AsArray())
                .Select(s => Path.GetFileNameWithoutExtension(s!["name"]!.GetValue<string>()))
                .Where(s => !s.StartsWith("epicchain-cli", StringComparison.InvariantCultureIgnoreCase));
        }
    }
}
