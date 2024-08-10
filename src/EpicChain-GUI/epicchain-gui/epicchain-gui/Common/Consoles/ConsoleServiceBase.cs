using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Runtime.Loader;
using System.Security;
using System.ServiceProcess;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Neo.Services;

namespace Neo.Common.Consoles
{
    public abstract class ConsoleServiceBase
    {
        protected virtual string Depends => null;
        protected virtual string Prompt => "service";

        public abstract string ServiceName { get; }

        protected bool ShowPrompt { get; set; } = true;
        public bool ReadingPassword { get; set; } = false;

        private bool _running;
        private readonly CancellationTokenSource _shutdownTokenSource = new CancellationTokenSource();
        private readonly CountdownEvent _shutdownAcknowledged = new CountdownEvent(1);

        protected virtual bool OnCommand(string[] args)
        {
            switch (args[0].ToLower())
            {
                case "":
                    return true;
                case "clear":
                    System.Console.Clear();
                    return true;
                case "exit":
                    return false;
                case "version":
                    System.Console.WriteLine(Assembly.GetEntryAssembly().GetVersion());
                    return true;
                default:
                    System.Console.WriteLine("error: command not found " + args[0]);
                    return true;
            }
        }

        public virtual void OnStart(string[] args)
        {
            // Register sigterm event handler
            AssemblyLoadContext.Default.Unloading += SigTermEventHandler;
            // Register sigint event handler
            System.Console.CancelKeyPress += CancelHandler;
        }

        protected internal virtual void OnStop()
        {
            _shutdownAcknowledged.Signal();
        }

        private static string[] ParseCommandLine(string line)
        {
            List<string> outputArgs = new List<string>();
            using (StringReader reader = new StringReader(line))
            {
                while (true)
                {
                    switch (reader.Peek())
                    {
                        case -1:
                            return outputArgs.ToArray();
                        case ' ':
                            reader.Read();
                            break;
                        case '\"':
                            outputArgs.Add(ParseCommandLineString(reader));
                            break;
                        default:
                            outputArgs.Add(ParseCommandLineArgument(reader));
                            break;
                    }
                }
            }
        }

        private static string ParseCommandLineArgument(TextReader reader)
        {
            StringBuilder sb = new StringBuilder();
            while (true)
            {
                int c = reader.Read();
                switch (c)
                {
                    case -1:
                    case ' ':
                        return sb.ToString();
                    default:
                        sb.Append((char)c);
                        break;
                }
            }
        }

        private static string ParseCommandLineString(TextReader reader)
        {
            if (reader.Read() != '\"') throw new FormatException();
            StringBuilder sb = new StringBuilder();
            while (true)
            {
                int c = reader.Peek();
                switch (c)
                {
                    case '\"':
                        reader.Read();
                        return sb.ToString();
                    case '\\':
                        sb.Append(ParseEscapeCharacter(reader));
                        break;
                    default:
                        reader.Read();
                        sb.Append((char)c);
                        break;
                }
            }
        }

        private static char ParseEscapeCharacter(TextReader reader)
        {
            if (reader.Read() != '\\') throw new FormatException();
            int c = reader.Read();
            switch (c)
            {
                case -1:
                    throw new FormatException();
                case 'n':
                    return '\n';
                case 'r':
                    return '\r';
                case 't':
                    return '\t';
                case 'x':
                    StringBuilder sb = new StringBuilder();
                    for (int i = 0; i < 2; i++)
                    {
                        int h = reader.Read();
                        if (h >= '0' && h <= '9' || h >= 'A' && h <= 'F' || h >= 'a' && h <= 'f')
                            sb.Append((char)h);
                        else
                            throw new FormatException();
                    }
                    return (char)byte.Parse(sb.ToString(), NumberStyles.AllowHexSpecifier);
                default:
                    return (char)c;
            }
        }

        public string ReadUserInput(string prompt, bool password = false)
        {
            const string t = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
            StringBuilder sb = new StringBuilder();
            ConsoleKeyInfo key;

            if (!string.IsNullOrEmpty(prompt))
            {
                System.Console.Write(prompt + ": ");
            }

            if (password) ReadingPassword = true;
            var prevForeground = System.Console.ForegroundColor;
            System.Console.ForegroundColor = ConsoleColor.Yellow;

            if (System.Console.IsInputRedirected)
            {
                // neo-gui Console require it
                sb.Append(System.Console.ReadLine());
            }
            else
            {
                do
                {
                    key = System.Console.ReadKey(true);

                    if (t.IndexOf(key.KeyChar) != -1)
                    {
                        sb.Append(key.KeyChar);
                        if (password)
                        {
                            System.Console.Write('*');
                        }
                        else
                        {
                            System.Console.Write(key.KeyChar);
                        }
                    }
                    else if (key.Key == ConsoleKey.Backspace && sb.Length > 0)
                    {
                        sb.Length--;
                        System.Console.Write("\b \b");
                    }
                } while (key.Key != ConsoleKey.Enter);
            }

            System.Console.ForegroundColor = prevForeground;
            if (password) ReadingPassword = false;
            System.Console.WriteLine();
            return sb.ToString();
        }

        public SecureString ReadSecureString(string prompt)
        {
            const string t = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
            SecureString securePwd = new SecureString();
            ConsoleKeyInfo key;

            if (!string.IsNullOrEmpty(prompt))
            {
                System.Console.Write(prompt + ": ");
            }

            ReadingPassword = true;
            System.Console.ForegroundColor = ConsoleColor.Yellow;

            do
            {
                key = System.Console.ReadKey(true);
                if (t.IndexOf(key.KeyChar) != -1)
                {
                    securePwd.AppendChar(key.KeyChar);
                    System.Console.Write('*');
                }
                else if (key.Key == ConsoleKey.Backspace && securePwd.Length > 0)
                {
                    securePwd.RemoveAt(securePwd.Length - 1);
                    System.Console.Write(key.KeyChar);
                    System.Console.Write(' ');
                    System.Console.Write(key.KeyChar);
                }
            } while (key.Key != ConsoleKey.Enter);

            System.Console.ForegroundColor = ConsoleColor.White;
            ReadingPassword = false;
            System.Console.WriteLine();
            securePwd.MakeReadOnly();
            return securePwd;
        }

        private void TriggerGracefulShutdown()
        {
            if (!_running) return;
            _running = false;
            _shutdownTokenSource.Cancel();
            // Wait for us to have triggered shutdown.
            _shutdownAcknowledged.Wait();
        }

        private void SigTermEventHandler(AssemblyLoadContext obj)
        {
            TriggerGracefulShutdown();
        }

        private void CancelHandler(object sender, ConsoleCancelEventArgs e)
        {
            e.Cancel = true;
            TriggerGracefulShutdown();
        }

        public void Run(string[] args)
        {
            OnStart(args);
            RunConsole();
            OnStop();
        }

        protected string ReadLine()
        {
            Task<string> readLineTask = Task.Run(() => System.Console.ReadLine());

            try
            {
                readLineTask.Wait(_shutdownTokenSource.Token);
            }
            catch (OperationCanceledException)
            {
                return null;
            }

            return readLineTask.Result;
        }

        public void RunConsole()
        {
            _running = true;
            string[] emptyarg = new string[] { "" };
            if (Environment.OSVersion.Platform == PlatformID.Win32NT)
                try
                {
                    System.Console.Title = ServiceName;
                }
                catch { }

            System.Console.ForegroundColor = ConsoleColor.DarkGreen;
            System.Console.WriteLine($"{ServiceName} Version: {Assembly.GetEntryAssembly().GetVersion()}");
            System.Console.WriteLine();

            while (_running)
            {
                if (ShowPrompt)
                {
                    System.Console.ForegroundColor = ConsoleColor.Green;
                    System.Console.Write($"{Prompt}> ");
                }

                System.Console.ForegroundColor = ConsoleColor.Yellow;
                string line = ReadLine()?.Trim();
                if (line == null) break;
                System.Console.ForegroundColor = ConsoleColor.White;

                try
                {
                    string[] args = ParseCommandLine(line);
                    if (args.Length == 0)
                        args = emptyarg;

                    _running = OnCommand(args);
                }
                catch (Exception ex)
                {
                    System.Console.WriteLine($"error: {ex.Message}");
                }
            }

            System.Console.ResetColor();
        }
    }
}

