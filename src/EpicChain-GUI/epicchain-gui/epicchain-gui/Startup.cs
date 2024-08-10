using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net.WebSockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Threading.Tasks.Dataflow;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Components.Forms;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.WebSockets;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Neo.Common;
using Neo.Models.Jobs;
using Neo.Services;


namespace Neo
{
    public class Startup
    {

        public IConfiguration Configuration { get; }

        public string ContentRootPath { get; set; }
        public Startup(IConfiguration configuration, IHostEnvironment env)
        {
            Configuration = configuration;

#if DEBUG
            var root = env.ContentRootPath;
            ContentRootPath = Path.Combine(root, "ClientApp");

            if (Directory.Exists(ContentRootPath))
            {
                CommandLineTool.Run("npm run dev",ContentRootPath);
            }
#endif
        }


        // This method gets called by the runtime. Use this method to add services to the container.
        public void ConfigureServices(IServiceCollection services)
        {
            services.AddWebSocketInvoker();
            services.AddSingleton<NotificationService>();
            services.AddSingleton<JsonRpcMiddleware>();
            services.AddWebSockets(option =>
            {

            });
        }

        // This method gets called by the runtime. Use this method to configure the HTTP request pipeline.
        public void Configure(IApplicationBuilder app, IWebHostEnvironment env)
        {
            app.UseWebSockets();
            app.UseMiddleware<JsonRpcMiddleware>();
            app.UseMiddleware<WebSocketHubMiddleware>();

            var notify = app.UseNotificationService();
            notify.Register(new SyncHeightJob(TimeSpan.FromSeconds(5)));
            notify.Register(new SyncWalletJob(TimeSpan.FromSeconds(11)));
            notify.Register(new TransactionConfirmJob(TimeSpan.FromSeconds(7)));

            //app.UseSpa(spa =>
            //{
            //    spa.Options.SourcePath = "ClientApp";

            //    if (env.IsDevelopment())
            //    {
            //        spa.UseReactDevelopmentServer(npmScript: "start");
            //    }
            //});
        }

    }
}
