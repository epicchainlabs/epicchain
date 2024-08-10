using System;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.Runtime.InteropServices;

namespace Neo.Common
{
    public class CommandLineTool
    {

        public static readonly bool IsWindows = RuntimeInformation.IsOSPlatform(OSPlatform.Windows);
        public static readonly bool IsMac = RuntimeInformation.IsOSPlatform(OSPlatform.OSX);
        public static readonly bool IsLinux = RuntimeInformation.IsOSPlatform(OSPlatform.Linux);

        public static readonly ConcurrentQueue<Process> CurrentProcesses = new ConcurrentQueue<Process>();

        private static string shell
        {
            get
            {
                return IsWindows ? "cmd" : "bash";
            }
        }

        private static string shellArg
        {
            get
            {
                return IsWindows ? "/c " : "-c ";
            }
        }
        

        public static Process Run(string command, string workDirectory = "")
        {
            Process p = new Process();
            p.StartInfo.FileName = shell;
            p.StartInfo.Arguments = shellArg + command;
            p.StartInfo.WorkingDirectory = workDirectory;
            p.OutputDataReceived += (s, r) =>
            {
                if (r.Data == null)
                {
                    Console.WriteLine($"close");
                    p = null;
                    return;
                }
                Console.WriteLine(r.Data);
            };
            p.Start();
            CurrentProcesses.Enqueue(p);
            return p;
        }

        public static Process Run(string command, string workDirectory = "", Action<string> receiveOutput = null)
        {
            Process p = new Process();
            //设置要启动的应用程序
            p.StartInfo.FileName = shell;
            p.StartInfo.WorkingDirectory = workDirectory;
            //是否使用操作系统shell启动
            p.StartInfo.UseShellExecute = false;
            // 接受来自调用程序的输入信息
            p.StartInfo.RedirectStandardInput = true;
            //输出信息
            p.StartInfo.RedirectStandardOutput = true;
            // 输出错误
            p.StartInfo.RedirectStandardError = true;
            //p.StartInfo.StandardOutputEncoding=Encoding.Unicode;
            ;
            //不显示程序窗口
            //p.StartInfo.CreateNoWindow = true;
            p.OutputDataReceived += (s, r) =>
            {
                if (r.Data == null)
                {
                    Console.WriteLine($"close");
                    p = null;
                    return;
                }
                //Console.WriteLine(r.Data);
                //receiveOutput?.Invoke(r.Data);
            };
            //启动程序
            p.Start();
            p.StandardInput.WriteLine(command);
            p.BeginOutputReadLine();
            CurrentProcesses.Enqueue(p);
            return p;
        }


        public static void Close()
        {
            foreach (var currentProcess in CurrentProcesses)
            {
                currentProcess.Kill();
            }
        }
    }
}
