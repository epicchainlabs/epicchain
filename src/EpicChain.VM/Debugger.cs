// Copyright (C) 2021-2024 EpicChain Labs.

//
// Debugger.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Collections.Generic;

namespace EpicChain.VM
{
    /// <summary>
    /// A simple debugger for <see cref="ExecutionEngine"/>.
    /// </summary>
    public class Debugger
    {
        private readonly ExecutionEngine engine;
        private readonly Dictionary<Script, HashSet<uint>> break_points = new();

        /// <summary>
        /// Create a debugger on the specified <see cref="ExecutionEngine"/>.
        /// </summary>
        /// <param name="engine">The <see cref="ExecutionEngine"/> to attach the debugger.</param>
        public Debugger(ExecutionEngine engine)
        {
            this.engine = engine;
        }

        /// <summary>
        /// Add a breakpoint at the specified position of the specified script. The VM will break the execution when it reaches the breakpoint.
        /// </summary>
        /// <param name="script">The script to add the breakpoint.</param>
        /// <param name="position">The position of the breakpoint in the script.</param>
        public void AddBreakPoint(Script script, uint position)
        {
            if (!break_points.TryGetValue(script, out HashSet<uint>? hashset))
            {
                hashset = new HashSet<uint>();
                break_points.Add(script, hashset);
            }
            hashset.Add(position);
        }

        /// <summary>
        /// Start or continue execution of the VM.
        /// </summary>
        /// <returns>Returns the state of the VM after the execution.</returns>
        public VMState Execute()
        {
            if (engine.State == VMState.BREAK)
                engine.State = VMState.NONE;
            while (engine.State == VMState.NONE)
                ExecuteAndCheckBreakPoints();
            return engine.State;
        }

        private void ExecuteAndCheckBreakPoints()
        {
            engine.ExecuteNext();
            if (engine.State == VMState.NONE && engine.InvocationStack.Count > 0 && break_points.Count > 0)
            {
                if (break_points.TryGetValue(engine.CurrentContext!.Script, out HashSet<uint>? hashset) && hashset.Contains((uint)engine.CurrentContext.InstructionPointer))
                    engine.State = VMState.BREAK;
            }
        }

        /// <summary>
        /// Removes the breakpoint at the specified position in the specified script.
        /// </summary>
        /// <param name="script">The script to remove the breakpoint.</param>
        /// <param name="position">The position of the breakpoint in the script.</param>
        /// <returns>
        /// <see langword="true"/> if the breakpoint is successfully found and removed;
        /// otherwise, <see langword="false"/>.
        /// </returns>
        public bool RemoveBreakPoint(Script script, uint position)
        {
            if (!break_points.TryGetValue(script, out HashSet<uint>? hashset)) return false;
            if (!hashset.Remove(position)) return false;
            if (hashset.Count == 0) break_points.Remove(script);
            return true;
        }

        /// <summary>
        /// Execute the next instruction. If the instruction involves a call to a method, it steps into the method and breaks the execution on the first instruction of that method.
        /// </summary>
        /// <returns>The VM state after the instruction is executed.</returns>
        public VMState StepInto()
        {
            if (engine.State == VMState.HALT || engine.State == VMState.FAULT)
                return engine.State;
            engine.ExecuteNext();
            if (engine.State == VMState.NONE)
                engine.State = VMState.BREAK;
            return engine.State;
        }

        /// <summary>
        /// Execute until the currently executed method is returned.
        /// </summary>
        /// <returns>The VM state after the currently executed method is returned.</returns>
        public VMState StepOut()
        {
            if (engine.State == VMState.BREAK)
                engine.State = VMState.NONE;
            int c = engine.InvocationStack.Count;
            while (engine.State == VMState.NONE && engine.InvocationStack.Count >= c)
                ExecuteAndCheckBreakPoints();
            if (engine.State == VMState.NONE)
                engine.State = VMState.BREAK;
            return engine.State;
        }

        /// <summary>
        /// Execute the next instruction. If the instruction involves a call to a method, it does not step into the method (it steps over it instead).
        /// </summary>
        /// <returns>The VM state after the instruction is executed.</returns>
        public VMState StepOver()
        {
            if (engine.State == VMState.HALT || engine.State == VMState.FAULT)
                return engine.State;
            engine.State = VMState.NONE;
            int c = engine.InvocationStack.Count;
            do
            {
                ExecuteAndCheckBreakPoints();
            }
            while (engine.State == VMState.NONE && engine.InvocationStack.Count > c);
            if (engine.State == VMState.NONE)
                engine.State = VMState.BREAK;
            return engine.State;
        }
    }
}
