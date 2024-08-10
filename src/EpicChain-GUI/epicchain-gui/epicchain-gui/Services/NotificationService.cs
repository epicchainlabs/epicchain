using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.WebSockets;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Builder;
using Microsoft.Extensions.DependencyInjection;
using Neo.Common;
using Neo.Ledger;
using Neo.Models.Jobs;

namespace Neo.Services
{
    public class NotificationService
    {
        private readonly WebSocketHub _hub;
        private readonly List<Job> _jobs = new List<Job>();
        private bool _running = true;
        public NotificationService(WebSocketHub hub)
        {
            _hub = hub;
        }

        public async Task Start()
        {
            while (_running)
            {
                foreach (var job in _jobs)
                {
                    try
                    {
                        var now = DateTime.Now;
                        if (job.NextTriggerTime <= now)
                        {
                            var msg = await job.Invoke();
                            if (msg != null)
                            {
                                _hub.PushAll(msg);
                            }
                            job.LastTriggerTime = now;
                        }
                    }
                    catch (Exception e)
                    {
                        Console.WriteLine($"Job Error[{job}]:{e}");
                    }
                }
                await Task.Delay(TimeSpan.FromSeconds(1));
            }
        }

        public void Register(Job job)
        {
            _jobs.Add(job);
        }

        public void Stop()
        {
            _running = false;
        }
    }


    public static class NotificationServiceExtension
    {
        public static NotificationService UseNotificationService(this IApplicationBuilder app)
        {
            var service = app.ApplicationServices.GetService<NotificationService>();
            Task.Run(service.Start);
            return service;
        }
    }
}
