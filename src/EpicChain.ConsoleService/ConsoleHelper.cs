// Copyright (C) 2021-2024 EpicChain Labs.

//
// ConsoleHelper.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System;
using System.Security;
using System.Text;

namespace EpicChain.ConsoleService
{
    public static class ConsoleHelper
    {
        private static readonly ConsoleColorSet InfoColor = new(ConsoleColor.Cyan);
        private static readonly ConsoleColorSet WarningColor = new(ConsoleColor.Yellow);
        private static readonly ConsoleColorSet ErrorColor = new(ConsoleColor.Red);

        public static bool ReadingPassword { get; private set; } = false;

        /// <summary>
        /// Info handles message in the format of "[tag]:[message]",
        /// avoid using Info if the `tag` is too long
        /// </summary>
        /// <param name="values">The log message in pairs of (tag, message)</param>
        public static void Info(params string[] values)
        {
            var currentColor = new ConsoleColorSet();

            for (int i = 0; i < values.Length; i++)
            {
                if (i % 2 == 0)
                    InfoColor.Apply();
                else
                    currentColor.Apply();
                Console.Write(values[i]);
            }
            currentColor.Apply();
            Console.WriteLine();
        }

        /// <summary>
        /// Use warning if something unexpected happens
        /// or the execution result is not correct.
        /// Also use warning if you just want to remind
        /// user of doing something.
        /// </summary>
        /// <param name="msg">Warning message</param>
        public static void Warning(string msg)
        {
            Log("Warning", WarningColor, msg);
        }

        /// <summary>
        /// Use Error if the verification or input format check fails
        /// or exception that breaks the execution of interactive
        /// command throws.
        /// </summary>
        /// <param name="msg">Error message</param>
        public static void Error(string msg)
        {
            Log("Error", ErrorColor, msg);
        }

        private static void Log(string tag, ConsoleColorSet colorSet, string msg)
        {
            var currentColor = new ConsoleColorSet();

            colorSet.Apply();
            Console.Write($"{tag}: ");
            currentColor.Apply();
            Console.WriteLine(msg);
        }

        public static string ReadUserInput(string prompt, bool password = false)
        {
            const string t = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
            var sb = new StringBuilder();

            if (!string.IsNullOrEmpty(prompt))
            {
                Console.Write(prompt + ": ");
            }

            if (password) ReadingPassword = true;
            var prevForeground = Console.ForegroundColor;
            Console.ForegroundColor = ConsoleColor.Yellow;

            if (Console.IsInputRedirected)
            {
                sb.Append(Console.ReadLine());
            }
            else
            {
                ConsoleKeyInfo key;
                do
                {
                    key = Console.ReadKey(true);

                    if (t.IndexOf(key.KeyChar) != -1)
                    {
                        sb.Append(key.KeyChar);
                        Console.Write(password ? '*' : key.KeyChar);
                    }
                    else if (key.Key == ConsoleKey.Backspace && sb.Length > 0)
                    {
                        sb.Length--;
                        Console.Write("\b \b");
                    }
                } while (key.Key != ConsoleKey.Enter);
            }

            Console.ForegroundColor = prevForeground;
            if (password) ReadingPassword = false;
            Console.WriteLine();
            return sb.ToString();
        }

        public static SecureString ReadSecureString(string prompt)
        {
            const string t = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
            SecureString securePwd = new SecureString();
            ConsoleKeyInfo key;

            if (!string.IsNullOrEmpty(prompt))
            {
                Console.Write(prompt + ": ");
            }

            ReadingPassword = true;
            Console.ForegroundColor = ConsoleColor.Yellow;

            do
            {
                key = Console.ReadKey(true);
                if (t.IndexOf(key.KeyChar) != -1)
                {
                    securePwd.AppendChar(key.KeyChar);
                    Console.Write('*');
                }
                else if (key.Key == ConsoleKey.Backspace && securePwd.Length > 0)
                {
                    securePwd.RemoveAt(securePwd.Length - 1);
                    Console.Write(key.KeyChar);
                    Console.Write(' ');
                    Console.Write(key.KeyChar);
                }
            } while (key.Key != ConsoleKey.Enter);

            Console.ForegroundColor = ConsoleColor.White;
            ReadingPassword = false;
            Console.WriteLine();
            securePwd.MakeReadOnly();
            return securePwd;
        }
    }
}
