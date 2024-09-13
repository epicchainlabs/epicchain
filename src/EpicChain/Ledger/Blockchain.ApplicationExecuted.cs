// Copyright (C) 2021-2024 EpicChain Labs.

//
// Blockchain.ApplicationExecuted.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using System.Linq;

namespace EpicChain.Ledger
{
    partial class Blockchain
    {
        partial class ApplicationExecuted
        {
            /// <summary>
            /// The transaction that contains the executed script. This field could be <see langword="null"/> if the contract is invoked by system.
            /// </summary>
            public Transaction Transaction { get; }

            /// <summary>
            /// The trigger of the execution.
            /// </summary>
            public TriggerType Trigger { get; }

            /// <summary>
            /// The state of the virtual machine after the contract is executed.
            /// </summary>
            public VMState VMState { get; }

            /// <summary>
            /// The exception that caused the execution to terminate abnormally. This field could be <see langword="null"/> if the execution ends normally.
            /// </summary>
            public Exception Exception { get; }

            /// <summary>
            /// EpicPulse spent to execute.
            /// </summary>
            public long EpicPulseConsumed { get; }

            /// <summary>
            /// Items on the stack of the virtual machine after execution.
            /// </summary>
            public StackItem[] Stack { get; }

            /// <summary>
            /// The notifications sent during the execution.
            /// </summary>
            public NotifyEventArgs[] Notifications { get; }

            internal ApplicationExecuted(ApplicationEngine engine)
            {
                Transaction = engine.ScriptContainer as Transaction;
                Trigger = engine.Trigger;
                VMState = engine.State;
                EpicPulseConsumed = engine.FeeConsumed;
                Exception = engine.FaultException;
                Stack = engine.ResultStack.ToArray();
                Notifications = engine.Notifications.ToArray();
            }
        }
    }
}
