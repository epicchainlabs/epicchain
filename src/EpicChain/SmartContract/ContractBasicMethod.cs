// Copyright (C) 2021-2024 EpicChain Labs.

//
// ContractBasicMethod.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


namespace EpicChain.SmartContract
{
    /// <summary>
    /// This class provides a guideline for basic methods used in the EpicChain blockchain, offering
    /// a generalized interaction mechanism for smart contract deployment, verification, updates, and destruction.
    /// </summary>
    public record ContractBasicMethod
    {
        /// <summary>
        /// The verification method. This must be called when withdrawing tokens from the contract.
        /// If the contract address is included in the transaction signature, this method verifies the signature.
        /// Example:
        /// <code>
        ///     public static bool Verify() => Runtime.CheckWitness(Owner);
        /// </code>
        /// <code>
        /// {
        ///   "name": "verify",
        ///   "safe": false,
        ///   "parameters": [],
        ///   "returntype": "bool"
        /// }
        /// </code>
        /// </summary>
        public static string Verify { get; } = "verify";

        /// <summary>
        /// The initialization method. Compiled into the <see cref="Manifest"/> file if any function uses the initialize statement.
        /// These functions are executed first when loading the contract.
        /// Example:
        /// <code>
        ///     private static readonly UInt160 owner = "NdUL5oDPD159KeFpD5A9zw5xNF1xLX6nLT";
        /// </code>
        /// </summary>
        public static string Initialize { get; } = "_initialize";

        /// <summary>
        /// The deployment method. Automatically executed by the ContractManagement contract when a contract is first deployed or updated.
        /// <code>
        /// {
        ///     "name": "_deploy",
        ///     "safe": false,
        ///     "parameters": [
        ///     {
        ///         "name": "data",
        ///         "type": "Any"
        ///     },
        ///     {
        ///         "name": "update",
        ///         "type": "Boolean"
        ///     }
        ///     ],
        ///     "returntype": "Void"
        /// }
        /// </code>
        /// </summary>
        public static string Deploy { get; } = "_deploy";

        /// <summary>
        /// The update method. Requires <see cref="NefFile"/> or <see cref="Manifest"/>, or both, and is passed to _deploy.
        /// Should verify the signer's address using SYSCALL <code>EpicChain.Runtime.CheckWitness</code>.
        /// <code>
        /// {
        ///   "name": "update",
        ///   "safe": false,
        ///   "parameters": [
        ///     {
        ///       "name": "nefFile",
        ///       "type": "ByteArray"
        ///     },
        ///     {
        ///       "name": "manifest",
        ///       "type": "ByteArray"
        ///     },
        ///     {
        ///       "name": "data",
        ///       "type": "Any"
        ///     }
        ///   ],
        ///   "returntype": "Void"
        /// }
        /// </code>
        /// </summary>
        public static string Update { get; } = "update";

        /// <summary>
        /// The destruction method. Deletes all the storage of the contract.
        /// Should verify the signer's address using SYSCALL <code>EpicChain.Runtime.CheckWitness</code>.
        /// Any tokens in the contract must be transferred before destruction.
        /// <code>
        /// {
        ///   "name": "destroy",
        ///   "safe": false,
        ///   "parameters": [],
        ///   "returntype": "Void"
        /// }
        /// </code>
        /// </summary>
        public static string Destroy { get; } = "destroy";

        /// <summary>
        /// Parameter counts for the methods.
        /// -1 represents the method can take arbitrary parameters.
        /// </summary>
        public static int VerifyPCount { get; } = -1;
        public static int InitializePCount { get; } = 0;
        public static int DeployPCount { get; } = 2;
        public static int UpdatePCount { get; } = 3;
        public static int DestroyPCount { get; } = 0;
    }
}
