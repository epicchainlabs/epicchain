using System;
using System.Collections.Generic;
using Neo.SmartContract;
using Neo.VM;
using Neo.VM.Types;

namespace Neo.Common.Storage
{
    public class ExecuteResultInfo
    {
        /// <summary>
        /// transaction hash
        /// </summary>
        public UInt256 TxId { get; set; }

        public TriggerType Trigger { get; set; }
        public VMState VMState { get; set; }
        public long GasConsumed { get; set; }

        /// <summary>
        /// execute result json array
        /// </summary>
        public StackItem[] ResultStack { get; set; }
        public List<NotificationInfo> Notifications { get; set; }
    }


    /// <summary>
    /// notify item for save
    /// </summary>
    public class NotificationInfo
    {
        /// <summary>
        /// contract script hash bin-endian, starts with "0x"
        /// </summary>
        public UInt160 Contract { get; set; }

        public string EventName { get; set; }

        public StackItem State { get; set; }

    }
}
