using System;
using System.IO;
using System.Net;
using System.Threading.Tasks;
using Microsoft.AspNetCore;
using Microsoft.AspNetCore.Hosting;
using Neo.Common;

namespace Neo
{
    class Program
    {
        public static GuiStarter Starter = new GuiStarter();

        static async Task Main(string[] args)
        {
            AppDomain.CurrentDomain.ProcessExit += new EventHandler(CurrentDomain_ProcessExit);

            Starter.Start(args);
            CreateWebHostBuilder(args).Build().Start();
            Starter.RunConsole();
            Starter.Stop();

        }

        public static IWebHostBuilder CreateWebHostBuilder(string[] args)
        {
            var url = "http://127.0.0.1:8081";
            var envPort = Environment.GetEnvironmentVariable("NEO_GUI_PORT");
            if (int.TryParse(envPort, out var port))
            {
                url = $"http://127.0.0.1:{port}";
            }
            return WebHost.CreateDefaultBuilder(args)
                .UseUrls(url)
                .UseStartup<Startup>();
        }


        public static void CurrentDomain_ProcessExit(object sender, EventArgs e)
        {
            CommandLineTool.Close();
        }
    }
}
