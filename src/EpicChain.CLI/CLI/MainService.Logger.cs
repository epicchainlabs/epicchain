// Copyright (C) 2021-2024 EpicChain Labs.

//
// MainService.Logger.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
