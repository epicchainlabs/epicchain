// Copyright (C) 2021-2024 EpicChain Labs.

//
// NotifyEventArgs.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.IO;
using EpicChain.Network.P2P.Payloads;
using EpicChain.VM;
using EpicChain.VM.Types;
using System;
using Array = EpicChain.VM.Types.Array;

namespace EpicChain.SmartContract
{
    /// <summary>
    /// The <see cref="EventArgs"/> of <see cref="ApplicationEngine.Notify"/>.
    /// </summary>
    public class NotifyEventArgs : EventArgs, IInteroperable
    {
        /// <summary>
        /// The container that containing the executed script.
        /// </summary>
        public IVerifiable ScriptContainer { get; }

        /// <summary>
        /// The script hash of the contract that sends the log.
        /// </summary>
        public UInt160 ScriptHash { get; }

        /// <summary>
        /// The name of the event.
        /// </summary>
        public string EventName { get; }

        /// <summary>
        /// The arguments of the event.
        /// </summary>
        public Array State { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="NotifyEventArgs"/> class.
        /// </summary>
        /// <param name="container">The container that containing the executed script.</param>
        /// <param name="script_hash">The script hash of the contract that sends the log.</param>
        /// <param name="eventName">The name of the event.</param>
        /// <param name="state">The arguments of the event.</param>
        public NotifyEventArgs(IVerifiable container, UInt160 script_hash, string eventName, Array state)
        {
            ScriptContainer = container;
            ScriptHash = script_hash;
            EventName = eventName;
            State = state;
        }

        public void FromStackItem(StackItem stackItem)
        {
            throw new NotSupportedException();
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter)
        {
            return new Array(referenceCounter)
                {
                    ScriptHash.ToArray(),
                    EventName,
                    State
                };
        }

        public StackItem ToStackItem(ReferenceCounter referenceCounter, ApplicationEngine engine)
        {
            if (engine.IsHardforkEnabled(Hardfork.HF_Domovoi))
            {
                return new Array(referenceCounter)
                {
                    ScriptHash.ToArray(),
                    EventName,
                    State.OnStack ? State : State.DeepCopy(true)
                };
            }

            return new Array(referenceCounter)
                {
                    ScriptHash.ToArray(),
                    EventName,
                    State
                };
        }
    }
}
