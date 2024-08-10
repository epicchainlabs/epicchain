using System.Collections.Generic;

namespace Neo.Models.Contracts
{
    public class InvokeEventValueModel
    {
        public UInt160 Contract { get; set; }
        public string EventName { get; set; }
        public object EventParameters { get; set; }
    }
}