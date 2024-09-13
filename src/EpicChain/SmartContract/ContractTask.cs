// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractTask.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using System.Runtime.CompilerServices;

namespace EpicChain.SmartContract
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
