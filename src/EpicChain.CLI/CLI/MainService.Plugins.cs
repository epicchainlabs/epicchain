// Copyright (C) 2021-2024 EpicChain Labs.

//
// MainService.Plugins.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
