// Copyright (C) 2021-2024 The EpicChain Labs.
//
// ContractTask.cs file is part of the EpicChain ecosystem and is distributed under the MIT license.
// For details, see the accompanying LICENSE file in the main directory of the repository
// or visit http://www.opensource.org/licenses/mit-license.php.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met: 1) Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer. 2) Redistributions in binary form must reproduce the
// above copyright notice, this list of conditions and the following disclaimer.


using System.Runtime.CompilerServices;

namespace Neo.SmartContract
{
    [AsyncMethodBuilder(typeof(ContractTaskMethodBuilder))]
    class ContractTask
    {
        protected readonly ContractTaskAwaiter _awaiter;

        public static ContractTask CompletedTask { get; }

        static ContractTask()
        {
            CompletedTask = new ContractTask();
            CompletedTask.GetAwaiter().SetResult();
        }

        public ContractTask()
        {
            _awaiter = CreateAwaiter();
        }

        protected virtual ContractTaskAwaiter CreateAwaiter() => new();
        public ContractTaskAwaiter GetAwaiter() => _awaiter;
        public virtual object GetResult() => null;
    }

    [AsyncMethodBuilder(typeof(ContractTaskMethodBuilder<>))]
    class ContractTask<T> : ContractTask
    {
        public new static ContractTask<T> CompletedTask { get; }

        static ContractTask()
        {
            CompletedTask = new ContractTask<T>();
            CompletedTask.GetAwaiter().SetResult();
        }

        protected override ContractTaskAwaiter CreateAwaiter() => new ContractTaskAwaiter<T>();
#pragma warning disable CS0108 // Member hides inherited member; missing new keyword
        public ContractTaskAwaiter<T> GetAwaiter() => (ContractTaskAwaiter<T>)_awaiter;
#pragma warning restore CS0108 // Member hides inherited member; missing new keyword
        public override object GetResult() => GetAwaiter().GetResult();
    }
}
