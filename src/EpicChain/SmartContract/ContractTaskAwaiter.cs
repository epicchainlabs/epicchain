// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractTaskAwaiter.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System;
using System.Runtime.CompilerServices;
using System.Threading;

namespace EpicChain.SmartContract
{
    class ContractTaskAwaiter : INotifyCompletion
    {
        private Action continuation;
        private Exception exception;

        public bool IsCompleted { get; private set; }

        public void GetResult()
        {
            if (exception is not null)
                throw exception;
        }

        public void SetResult() => RunContinuation();

        public virtual void SetResult(ApplicationEngine engine) => SetResult();

        public void SetException(Exception exception)
        {
            this.exception = exception;
            RunContinuation();
        }

        public void OnCompleted(Action continuation)
        {
            Interlocked.CompareExchange(ref this.continuation, continuation, null);
        }

        protected void RunContinuation()
        {
            IsCompleted = true;
            continuation?.Invoke();
        }
    }

    class ContractTaskAwaiter<T> : ContractTaskAwaiter
    {
        private T result;

        public new T GetResult()
        {
            base.GetResult();
            return result;
        }

        public void SetResult(T result)
        {
            this.result = result;
            RunContinuation();
        }

        public override void SetResult(ApplicationEngine engine)
        {
            SetResult((T)engine.Convert(engine.Pop(), new InteropParameterDescriptor(typeof(T))));
        }
    }
}
