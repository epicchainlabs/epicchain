// Copyright (C) 2021-2024 EpicChain Labs.

//
// TestEngine.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.VM;
using EpicChain.VM.Types;
using System;

namespace EpicChain.Test.Types
{
    class TestEngine : ExecutionEngine
    {
        public Exception FaultException { get; private set; }

        public TestEngine() : base(ComposeJumpTable()) { }

        private static JumpTable ComposeJumpTable()
        {
            JumpTable jumpTable = new JumpTable();
            jumpTable[OpCode.SYSCALL] = OnSysCall;
            return jumpTable;
        }

        private static void OnSysCall(ExecutionEngine engine, Instruction instruction)
        {
            uint method = instruction.TokenU32;

            if (method == 0x77777777)
            {
                engine.CurrentContext.EvaluationStack.Push(StackItem.FromInterface(new object()));
                return;
            }

            if (method == 0xaddeadde)
            {
                engine.JumpTable.ExecuteThrow(engine, "error");
                return;
            }

            throw new Exception();
        }

        protected override void OnFault(Exception ex)
        {
            FaultException = ex;
            base.OnFault(ex);
        }
    }
}
