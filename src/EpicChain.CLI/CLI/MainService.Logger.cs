// =============================================================================================
//  © Copyright (C) 2021-2025 EpicChain Labs. All rights reserved.
// =============================================================================================
//
//  File: MainService.Logger.cs
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



using EpicChain.ConsoleService;
using EpicChain.IEventHandlers;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using static System.IO.Path;

namespace EpicChain.CLI
{
    partial class MainService : ILoggingHandler
    {
        private static readonly ConsoleColorSet DebugColor = new(ConsoleColor.Cyan);
        private static readonly ConsoleColorSet InfoColor = new(ConsoleColor.White);
        private static readonly ConsoleColorSet WarningColor = new(ConsoleColor.Yellow);
        private static readonly ConsoleColorSet ErrorColor = new(ConsoleColor.Red);
        private static readonly ConsoleColorSet FatalColor = new(ConsoleColor.Red);

        private readonly object syncRoot = new();
        private bool _showLog = Settings.Default.Logger.ConsoleOutput;

        private void Initialize_Logger()
        {
            Utility.Logging += ((ILoggingHandler)this).Utility_Logging_Handler;
        }

        private void Dispose_Logger()
        {
            Utility.Logging -= ((ILoggingHandler)this).Utility_Logging_Handler;
        }

        /// <summary>
        /// Process "console log off" command to turn off console log
        /// </summary>
        [ConsoleCommand("console log off", Category = "Log Commands")]
        private void OnLogOffCommand()
        {
            _showLog = false;
        }

        /// <summary>
        /// Process "console log on" command to turn on the console log
        /// </summary>
        [ConsoleCommand("console log on", Category = "Log Commands")]
        private void OnLogOnCommand()
        {
            _showLog = true;
        }

        private static void GetErrorLogs(StringBuilder sb, Exception ex)
        {
            sb.AppendLine(ex.GetType().ToString());
            sb.AppendLine(ex.Message);
            sb.AppendLine(ex.StackTrace);
            if (ex is AggregateException ex2)
            {
                foreach (Exception inner in ex2.InnerExceptions)
                {
                    sb.AppendLine();
                    GetErrorLogs(sb, inner);
                }
            }
            else if (ex.InnerException != null)
            {
                sb.AppendLine();
                GetErrorLogs(sb, ex.InnerException);
            }
        }

        void ILoggingHandler.Utility_Logging_Handler(string source, LogLevel level, object message)
        {
            if (!Settings.Default.Logger.Active)
                return;

            if (message is Exception ex)
            {
                var sb = new StringBuilder();
                GetErrorLogs(sb, ex);
                message = sb.ToString();
            }

            lock (syncRoot)
            {
                DateTime now = DateTime.Now;
                var log = $"[{now.TimeOfDay:hh\\:mm\\:ss\\.fff}]";
                if (_showLog)
                {
                    var currentColor = new ConsoleColorSet();
                    var messages = message is string msg ? Parse(msg) : new[] { message.ToString() };
                    ConsoleColorSet logColor;
                    string logLevel;
                    switch (level)
                    {
                        case LogLevel.Debug: logColor = DebugColor; logLevel = "DEBUG"; break;
                        case LogLevel.Error: logColor = ErrorColor; logLevel = "ERROR"; break;
                        case LogLevel.Fatal: logColor = FatalColor; logLevel = "FATAL"; break;
                        case LogLevel.Info: logColor = InfoColor; logLevel = "INFO"; break;
                        case LogLevel.Warning: logColor = WarningColor; logLevel = "WARN"; break;
                        default: logColor = InfoColor; logLevel = "INFO"; break;
                    }
                    logColor.Apply();
                    Console.Write($"{logLevel} {log} \t{messages[0],-20}");
                    for (var i = 1; i < messages.Length; i++)
                    {
                        if (messages[i]?.Length > 20)
                        {
                            messages[i] = $"{messages[i]![..10]}...{messages[i]![(messages[i]!.Length - 10)..]}";
                        }
                        Console.Write(i % 2 == 0 ? $"={messages[i]} " : $" {messages[i]}");
                    }
                    currentColor.Apply();
                    Console.WriteLine();
                }

                if (string.IsNullOrEmpty(Settings.Default.Logger.Path)) return;
                var sb = new StringBuilder(source);
                foreach (var c in GetInvalidFileNameChars())
                    sb.Replace(c, '-');
                var path = Combine(Settings.Default.Logger.Path, sb.ToString());
                Directory.CreateDirectory(path);
                path = Combine(path, $"{now:yyyy-MM-dd}.log");
                try
                {
                    File.AppendAllLines(path, new[] { $"[{level}]{log} {message}" });
                }
                catch (IOException)
                {
                    Console.WriteLine("Error writing the log file: " + path);
                }
            }
        }

        /// <summary>
        /// Parse the log message
        /// </summary>
        /// <param name="message">expected format [key1 = msg1 key2 = msg2]</param>
        /// <returns></returns>
        private static string[] Parse(string message)
        {
            var equals = message.Trim().Split('=');

            if (equals.Length == 1) return new[] { message };

            var messages = new List<string>();
            foreach (var t in @equals)
            {
                var msg = t.Trim();
                var parts = msg.Split(' ');
                var d = parts.Take(parts.Length - 1);

                if (parts.Length > 1)
                {
                    messages.Add(string.Join(" ", d));
                }
                var last = parts.LastOrDefault();
                if (last is not null)
                {
                    messages.Add(last);
                }
            }

            return messages.ToArray();
        }
    }
}
