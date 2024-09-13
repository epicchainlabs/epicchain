// Copyright (C) 2021-2024 EpicChain Labs.

//
// Helper.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography;
using EpicChain.IO;
using EpicChain.Network.P2P;
using EpicChain.Network.P2P.Payloads;
using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using System;
using static EpicChain.SmartContract.Helper;

namespace EpicChain.Wallets
{
    /// <summary>
    /// A helper class related to wallets.
    /// </summary>
    public static class Helper
    {
        /// <summary>
        /// Signs an <see cref="IVerifiable"/> with the specified private key.
        /// </summary>
        /// <param name="verifiable">The <see cref="IVerifiable"/> to sign.</param>
        /// <param name="key">The private key to be used.</param>
        /// <param name="network">The magic number of the NEO network.</param>
        /// <returns>The signature for the <see cref="IVerifiable"/>.</returns>
        public static byte[] Sign(this IVerifiable verifiable, KeyPair key, uint network)
        {
            return Crypto.Sign(verifiable.GetSignData(network), key.PrivateKey);
        }

        /// <summary>
        /// Converts the specified script hash to an address.
        /// </summary>
        /// <param name="scriptHash">The script hash to convert.</param>
        /// <param name="version">The address version.</param>
        /// <returns>The converted address.</returns>
        public static string ToAddress(this UInt160 scriptHash, byte version)
        {
            Span<byte> data = stackalloc byte[21];
            data[0] = version;
            scriptHash.ToArray().CopyTo(data[1..]);
            return Base58.Base58CheckEncode(data);
        }

        /// <summary>
        /// Converts the specified address to a script hash.
        /// </summary>
        /// <param name="address">The address to convert.</param>
        /// <param name="version">The address version.</param>
        /// <returns>The converted script hash.</returns>
        public static UInt160 ToScriptHash(this string address, byte version)
        {
            byte[] data = address.Base58CheckDecode();
            if (data.Length != 21)
                throw new FormatException();
            if (data[0] != version)
                throw new FormatException();
            return new UInt160(data.AsSpan(1));
        }

        internal static byte[] XOR(byte[] x, byte[] y)
        {
            if (x.Length != y.Length) throw new ArgumentException();
            byte[] r = new byte[x.Length];
            for (int i = 0; i < r.Length; i++)
                r[i] = (byte)(x[i] ^ y[i]);
            return r;
        }

        /// <summary>
        /// Calculates the network fee for the specified transaction.
        /// In the unit of datoshi, 1 datoshi = 1e-8 EpicPulse
        /// </summary>
        /// <param name="tx">The transaction to calculate.</param>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="settings">Thr protocol settings to use.</param>
        /// <param name="accountScript">Function to retrive the script's account from a hash.</param>
        /// <param name="maxExecutionCost">The maximum cost that can be spent when a contract is executed.</param>
        /// <returns>The network fee of the transaction.</returns>
        public static long CalculateNetworkFee(this Transaction tx, DataCache snapshot, ProtocolSettings settings, Func<UInt160, byte[]> accountScript, long maxExecutionCost = ApplicationEngine.TestModeEpicPulse)
        {
            UInt160[] hashes = tx.GetScriptHashesForVerifying(snapshot);

            // base size for transaction: includes const_header + signers + attributes + script + hashes
            int size = Transaction.HeaderSize + tx.Signers.GetVarSize() + tx.Attributes.GetVarSize() + tx.Script.GetVarSize() + IO.Helper.GetVarSize(hashes.Length), index = -1;
            uint exec_fee_factor = NativeContract.Policy.GetExecFeeFactor(snapshot);
            long networkFee = 0;
            foreach (UInt160 hash in hashes)
            {
                index++;
                byte[] witnessScript = accountScript(hash);
                byte[] invocationScript = null;

                if (tx.Witnesses != null && witnessScript is null)
                {
                    // Try to find the script in the witnesses
                    Witness witness = tx.Witnesses[index];
                    witnessScript = witness?.VerificationScript.ToArray();

                    if (witnessScript is null || witnessScript.Length == 0)
                    {
                        // Then it's a contract-based witness, so try to get the corresponding invocation script for it
                        invocationScript = witness?.InvocationScript.ToArray();
                    }
                }

                if (witnessScript is null || witnessScript.Length == 0)
                {
                    var contract = NativeContract.ContractManagement.GetContract(snapshot, hash);
                    if (contract is null)
                        throw new ArgumentException($"The smart contract or address {hash} ({hash.ToAddress(settings.AddressVersion)}) is not found. " +
                            $"If this is your wallet address and you want to sign a transaction with it, make sure you have opened this wallet.");
                    var md = contract.Manifest.Abi.GetMethod(ContractBasicMethod.Verify, ContractBasicMethod.VerifyPCount);
                    if (md is null)
                        throw new ArgumentException($"The smart contract {contract.Hash} haven't got verify method");
                    if (md.ReturnType != ContractParameterType.Boolean)
                        throw new ArgumentException("The verify method doesn't return boolean value.");
                    if (md.Parameters.Length > 0 && invocationScript is null)
                        throw new ArgumentException("The verify method requires parameters that need to be passed via the witness' invocation script.");

                    // Empty verification and non-empty invocation scripts
                    var invSize = invocationScript?.GetVarSize() ?? Array.Empty<byte>().GetVarSize();
                    size += Array.Empty<byte>().GetVarSize() + invSize;

                    // Check verify cost
                    using ApplicationEngine engine = ApplicationEngine.Create(TriggerType.Verification, tx, snapshot.CloneCache(), settings: settings, epicpulse: maxExecutionCost);
                    engine.LoadContract(contract, md, CallFlags.ReadOnly);
                    if (invocationScript != null) engine.LoadScript(invocationScript, configureState: p => p.CallFlags = CallFlags.None);
                    if (engine.Execute() == VMState.FAULT) throw new ArgumentException($"Smart contract {contract.Hash} verification fault.");
                    if (!engine.ResultStack.Pop().GetBoolean()) throw new ArgumentException($"Smart contract {contract.Hash} returns false.");

                    maxExecutionCost -= engine.FeeConsumed;
                    if (maxExecutionCost <= 0) throw new InvalidOperationException("Insufficient EpicPulse.");
                    networkFee += engine.FeeConsumed;
                }
                else if (IsSignatureContract(witnessScript))
                {
                    size += 67 + witnessScript.GetVarSize();
                    networkFee += exec_fee_factor * SignatureContractCost();
                }
                else if (IsMultiSigContract(witnessScript, out int m, out int n))
                {
                    int size_inv = 66 * m;
                    size += IO.Helper.GetVarSize(size_inv) + size_inv + witnessScript.GetVarSize();
                    networkFee += exec_fee_factor * MultiSignatureContractCost(m, n);
                }
                // We can support more contract types in the future.
            }
            networkFee += size * NativeContract.Policy.GetFeePerByte(snapshot);
            foreach (TransactionAttribute attr in tx.Attributes)
            {
                networkFee += attr.CalculateNetworkFee(snapshot, tx);
            }
            return networkFee;
        }
    }
}
