// Copyright (C) 2021-2024 EpicChain Labs.

//
// Utility.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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
using Akka.Event;
using System.Text;

namespace EpicChain
{
    public delegate void LogEventHandler(string source, LogLevel level, object message);

    /// <summary>
    /// A utility class that provides common functions.
    /// </summary>
    public static class Utility
    {
        internal class Logger : ReceiveActor
        {
            public Logger()
            {
                Receive<InitializeLogger>(_ => Sender.Tell(new LoggerInitialized()));
                Receive<LogEvent>(e => Log(e.LogSource, (LogLevel)e.LogLevel(), e.Message));
            }
        }

        public static event LogEventHandler? Logging;

        /// <summary>
        /// A strict UTF8 encoding used in EpicChain system.
        /// </summary>
        public static Encoding StrictUTF8 { get; }

        static Utility()
        {
            StrictUTF8 = (Encoding)Encoding.UTF8.Clone();
            StrictUTF8.DecoderFallback = DecoderFallback.ExceptionFallback;
            StrictUTF8.EncoderFallback = EncoderFallback.ExceptionFallback;
        }

        /// <summary>
        /// Writes a log.
        /// </summary>
        /// <param name="source">The source of the log. Used to identify the producer of the log.</param>
        /// <param name="level">The level of the log.</param>
        /// <param name="message">The message of the log.</param>
        public static void Log(string source, LogLevel level, object message)
        {
            Logging?.Invoke(source, level, message);
        }
    }
}
