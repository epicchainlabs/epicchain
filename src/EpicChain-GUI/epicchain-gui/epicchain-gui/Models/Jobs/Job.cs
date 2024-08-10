using System;
using System.Threading.Tasks;

namespace Neo.Models.Jobs
{
    public abstract class Job
    {

        public DateTime StartTime { get; set; } = DateTime.Now;

        public DateTime LastTriggerTime { get; set; } = DateTime.MinValue;

        public TimeSpan IntervalTime { get; set; } = TimeSpan.MaxValue;

        public DateTime NextTriggerTime => LastTriggerTime + IntervalTime;

        public abstract Task<WsMessage> Invoke();

    }


}