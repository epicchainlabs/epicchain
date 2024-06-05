// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ContractTaskAwaiter.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System;
using System.Runtime.CompilerServices;
using System.Threading;

namespace Neo.SmartContract
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
