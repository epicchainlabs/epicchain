// =============================================================================================
//  © Copyright (C) 2021-2025 EpicChain Labs. All rights reserved.
// =============================================================================================
//
//  File: MainService.Blockchain.cs
//  Project: EpicChain Labs - Core Blockchain Infrastructure
//  Author: Xmoohad (Muhammad Ibrahim Muhammad)
//
// ---------------------------------------------------------------------------------------------
//  Description:
//  This file is an integral part of the EpicChain Labs ecosystem, a forward-looking, open-source
//  blockchain initiative founded by Xmoohad. The EpicChain project aims to create a robust,
//  decentralized, and developer-friendly blockchain infrastructure that empowers innovation,
//  transparency, and digital sovereignty.
//
// ---------------------------------------------------------------------------------------------
//  Licensing:
//  This file is distributed under the permissive MIT License, which grants anyone the freedom
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of this
//  software. These rights are granted with the understanding that the original license notice
//  and copyright attribution remain intact.
//
//  For the full license text, please refer to the LICENSE file included in the root directory of
//  this repository or visit the official MIT License page at:
//  ➤ https://opensource.org/licenses/MIT
//
// ---------------------------------------------------------------------------------------------
//  Community and Contribution:
//  EpicChain Labs is deeply rooted in the principles of open-source development. We believe that
//  collaboration, transparency, and inclusiveness are the cornerstones of sustainable technology.
//
//  This file, like all components of the EpicChain ecosystem, is offered to the global development
//  community to explore, extend, and improve. Whether you're fixing bugs, optimizing performance,
//  or building new features, your contributions are welcome and appreciated.
//
//  By contributing to this project, you become part of a community dedicated to shaping the future
//  of blockchain technology. Join us in our mission to create more secure, scalable, and accessible
//  digital infrastructure for all.
//
// ---------------------------------------------------------------------------------------------
//  Terms of Use:
//  Redistribution and usage of this file in both source and compiled (binary) forms—with or without
//  modification—are fully permitted under the MIT License. Users of this software are expected to
//  adhere to the simple and clear guidelines established in the LICENSE file.
//
//  By using this file and other components of the EpicChain Labs project, you acknowledge and agree
//  to the terms of the MIT License. This ensures that the ethos of free and open software development
//  continues to flourish and remain protected.
//
// ---------------------------------------------------------------------------------------------
//  Final Note:
//  EpicChain Labs remains committed to pushing the boundaries of blockchain innovation. Whether
//  you're an experienced developer, a researcher, a student, or simply a curious enthusiast, we
//  invite you to explore the possibilities of EpicChain—and contribute toward a decentralized future.
//
//  Learn more about the project, get involved, or access full documentation at:
//  ➤ https://epic-chain.org
//
// =============================================================================================



using EpicChain.ConsoleService;
using EpicChain.Network.P2P.Payloads;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using System;
using System.Linq;

namespace EpicChain.CLI
{
    partial class MainService
    {
        /// <summary>
        /// Process "export blocks" command
        /// </summary>
        /// <param name="start">Start</param>
        /// <param name="count">Number of blocks</param>
        /// <param name="path">Path</param>
        [ConsoleCommand("export blocks", Category = "Blockchain Commands")]
        private void OnExportBlocksStartCountCommand(uint start, uint count = uint.MaxValue, string? path = null)
        {
            uint height = NativeContract.Ledger.CurrentIndex(EpicChainSystem.StoreView);
            if (height < start)
            {
                ConsoleHelper.Error("invalid start height.");
                return;
            }

            count = Math.Min(count, height - start + 1);

            if (string.IsNullOrEmpty(path))
            {
                path = $"chain.{start}.acc";
            }

            WriteBlocks(start, count, path, true);
        }

        [ConsoleCommand("show block", Category = "Blockchain Commands")]
        private void OnShowBlockCommand(string indexOrHash)
        {
            lock (syncRoot)
            {
                Block? block = null;

                if (uint.TryParse(indexOrHash, out var index))
                    block = NativeContract.Ledger.GetBlock(EpicChainSystem.StoreView, index);
                else if (UInt256.TryParse(indexOrHash, out var hash))
                    block = NativeContract.Ledger.GetBlock(EpicChainSystem.StoreView, hash);
                else
                {
                    ConsoleHelper.Error("Enter a valid block index or hash.");
                    return;
                }

                if (block is null)
                {
                    ConsoleHelper.Error($"Block {indexOrHash} doesn't exist.");
                    return;
                }

                DateTime blockDatetime = new DateTime(1970, 1, 1, 0, 0, 0, 0, DateTimeKind.Utc);
                blockDatetime = blockDatetime.AddMilliseconds(block.Timestamp).ToLocalTime();

                ConsoleHelper.Info("", "-------------", "Block", "-------------");
                ConsoleHelper.Info();
                ConsoleHelper.Info("", "      Timestamp: ", $"{blockDatetime}");
                ConsoleHelper.Info("", "          Index: ", $"{block.Index}");
                ConsoleHelper.Info("", "           Hash: ", $"{block.Hash}");
                ConsoleHelper.Info("", "          Nonce: ", $"{block.Nonce}");
                ConsoleHelper.Info("", "     MerkleRoot: ", $"{block.MerkleRoot}");
                ConsoleHelper.Info("", "       PrevHash: ", $"{block.PrevHash}");
                ConsoleHelper.Info("", "  NextConsensus: ", $"{block.NextConsensus}");
                ConsoleHelper.Info("", "   PrimaryIndex: ", $"{block.PrimaryIndex}");
                ConsoleHelper.Info("", "  PrimaryPubKey: ", $"{NativeContract.EpicChain.GetCommittee(EpicChainSystem.GetSnapshotCache())[block.PrimaryIndex]}");
                ConsoleHelper.Info("", "        Version: ", $"{block.Version}");
                ConsoleHelper.Info("", "           Size: ", $"{block.Size} Byte(s)");
                ConsoleHelper.Info();

                ConsoleHelper.Info("", "-------------", "Witness", "-------------");
                ConsoleHelper.Info();
                ConsoleHelper.Info("", "    Invocation Script: ", $"{Convert.ToBase64String(block.Witness.InvocationScript.Span)}");
                ConsoleHelper.Info("", "  Verification Script: ", $"{Convert.ToBase64String(block.Witness.VerificationScript.Span)}");
                ConsoleHelper.Info("", "           ScriptHash: ", $"{block.Witness.ScriptHash}");
                ConsoleHelper.Info("", "                 Size: ", $"{block.Witness.Size} Byte(s)");
                ConsoleHelper.Info();

                ConsoleHelper.Info("", "-------------", "Transactions", "-------------");
                ConsoleHelper.Info();

                if (block.Transactions.Length == 0)
                {
                    ConsoleHelper.Info("", "  No Transaction(s)");
                }
                else
                {
                    foreach (var tx in block.Transactions)
                        ConsoleHelper.Info($"  {tx.Hash}");
                }
                ConsoleHelper.Info();
                ConsoleHelper.Info("", "--------------------------------------");
            }
        }

        [ConsoleCommand("show tx", Category = "Blockchain Commands")]
        public void OnShowTransactionCommand(UInt256 hash)
        {
            lock (syncRoot)
            {
                var tx = NativeContract.Ledger.GetTransactionState(EpicChainSystem.StoreView, hash);

                if (tx is null)
                {
                    ConsoleHelper.Error($"Transaction {hash} doesn't exist.");
                    return;
                }

                var block = NativeContract.Ledger.GetHeader(EpicChainSystem.StoreView, tx.BlockIndex);

                DateTime transactionDatetime = new DateTime(1970, 1, 1, 0, 0, 0, 0, DateTimeKind.Utc);
                transactionDatetime = transactionDatetime.AddMilliseconds(block.Timestamp).ToLocalTime();

                ConsoleHelper.Info("", "-------------", "Transaction", "-------------");
                ConsoleHelper.Info();
                ConsoleHelper.Info("", "        Timestamp: ", $"{transactionDatetime}");
                ConsoleHelper.Info("", "             Hash: ", $"{tx.Transaction.Hash}");
                ConsoleHelper.Info("", "            Nonce: ", $"{tx.Transaction.Nonce}");
                ConsoleHelper.Info("", "           Sender: ", $"{tx.Transaction.Sender}");
                ConsoleHelper.Info("", "  ValidUntilBlock: ", $"{tx.Transaction.ValidUntilBlock}");
                ConsoleHelper.Info("", "       FeePerByte: ", $"{tx.Transaction.FeePerByte} datoshi");
                ConsoleHelper.Info("", "       NetworkFee: ", $"{tx.Transaction.NetworkFee} datoshi");
                ConsoleHelper.Info("", "        SystemFee: ", $"{tx.Transaction.SystemFee} datoshi");
                ConsoleHelper.Info("", "           Script: ", $"{Convert.ToBase64String(tx.Transaction.Script.Span)}");
                ConsoleHelper.Info("", "          Version: ", $"{tx.Transaction.Version}");
                ConsoleHelper.Info("", "       BlockIndex: ", $"{block.Index}");
                ConsoleHelper.Info("", "        BlockHash: ", $"{block.Hash}");
                ConsoleHelper.Info("", "             Size: ", $"{tx.Transaction.Size} Byte(s)");
                ConsoleHelper.Info();

                ConsoleHelper.Info("", "-------------", "Signers", "-------------");
                ConsoleHelper.Info();

                foreach (var signer in tx.Transaction.Signers)
                {
                    if (signer.Rules.Length == 0)
                        ConsoleHelper.Info("", "             Rules: ", "[]");
                    else
                        ConsoleHelper.Info("", "             Rules: ", $"[{string.Join(", ", signer.Rules.Select(s => $"\"{s.ToJson()}\""))}]");
                    ConsoleHelper.Info("", "           Account: ", $"{signer.Account}");
                    ConsoleHelper.Info("", "            Scopes: ", $"{signer.Scopes}");
                    if (signer.AllowedContracts.Length == 0)
                        ConsoleHelper.Info("", "  AllowedContracts: ", "[]");
                    else
                        ConsoleHelper.Info("", "  AllowedContracts: ", $"[{string.Join(", ", signer.AllowedContracts.Select(s => s.ToString()))}]");
                    if (signer.AllowedGroups.Length == 0)
                        ConsoleHelper.Info("", "     AllowedGroups: ", "[]");
                    else
                        ConsoleHelper.Info("", "     AllowedGroups: ", $"[{string.Join(", ", signer.AllowedGroups.Select(s => s.ToString()))}]");
                    ConsoleHelper.Info("", "              Size: ", $"{signer.Size} Byte(s)");
                    ConsoleHelper.Info();
                }

                ConsoleHelper.Info("", "-------------", "Witnesses", "-------------");
                ConsoleHelper.Info();
                foreach (var witness in tx.Transaction.Witnesses)
                {
                    ConsoleHelper.Info("", "    InvocationScript: ", $"{Convert.ToBase64String(witness.InvocationScript.Span)}");
                    ConsoleHelper.Info("", "  VerificationScript: ", $"{Convert.ToBase64String(witness.VerificationScript.Span)}");
                    ConsoleHelper.Info("", "          ScriptHash: ", $"{witness.ScriptHash}");
                    ConsoleHelper.Info("", "                Size: ", $"{witness.Size} Byte(s)");
                    ConsoleHelper.Info();
                }

                ConsoleHelper.Info("", "-------------", "Attributes", "-------------");
                ConsoleHelper.Info();
                if (tx.Transaction.Attributes.Length == 0)
                {
                    ConsoleHelper.Info("", "  No Attribute(s).");
                }
                else
                {
                    foreach (var attribute in tx.Transaction.Attributes)
                    {
                        switch (attribute)
                        {
                            case Conflicts c:
                                ConsoleHelper.Info("", "  Type: ", $"{c.Type}");
                                ConsoleHelper.Info("", "  Hash: ", $"{c.Hash}");
                                ConsoleHelper.Info("", "  Size: ", $"{c.Size} Byte(s)");
                                break;
                            case OracleResponse o:
                                ConsoleHelper.Info("", "    Type: ", $"{o.Type}");
                                ConsoleHelper.Info("", "      Id: ", $"{o.Id}");
                                ConsoleHelper.Info("", "    Code: ", $"{o.Code}");
                                ConsoleHelper.Info("", "  Result: ", $"{Convert.ToBase64String(o.Result.Span)}");
                                ConsoleHelper.Info("", "    Size: ", $"{o.Size} Byte(s)");
                                break;
                            case HighPriorityAttribute p:
                                ConsoleHelper.Info("", "    Type: ", $"{p.Type}");
                                break;
                            case NotValidBefore n:
                                ConsoleHelper.Info("", "    Type: ", $"{n.Type}");
                                ConsoleHelper.Info("", "  Height: ", $"{n.Height}");
                                break;
                            default:
                                ConsoleHelper.Info("", "  Type: ", $"{attribute.Type}");
                                ConsoleHelper.Info("", "  Size: ", $"{attribute.Size} Byte(s)");
                                break;
                        }
                    }
                }
                ConsoleHelper.Info();
                ConsoleHelper.Info("", "--------------------------------------");
            }
        }

        [ConsoleCommand("show contract", Category = "Blockchain Commands")]
        public void OnShowContractCommand(string nameOrHash)
        {
            lock (syncRoot)
            {
                ContractState? contract = null;

                if (UInt160.TryParse(nameOrHash, out var scriptHash))
                    contract = NativeContract.ContractManagement.GetContract(EpicChainSystem.StoreView, scriptHash);
                else
                {
                    var nativeContract = NativeContract.Contracts.SingleOrDefault(s => s.Name.Equals(nameOrHash, StringComparison.InvariantCultureIgnoreCase));

                    if (nativeContract != null)
                        contract = NativeContract.ContractManagement.GetContract(EpicChainSystem.StoreView, nativeContract.Hash);
                }

                if (contract is null)
                {
                    ConsoleHelper.Error($"Contract {nameOrHash} doesn't exist.");
                    return;
                }

                ConsoleHelper.Info("", "-------------", "Contract", "-------------");
                ConsoleHelper.Info();
                ConsoleHelper.Info("", "                Name: ", $"{contract.Manifest.Name}");
                ConsoleHelper.Info("", "                Hash: ", $"{contract.Hash}");
                ConsoleHelper.Info("", "                  Id: ", $"{contract.Id}");
                ConsoleHelper.Info("", "       UpdateCounter: ", $"{contract.UpdateCounter}");
                ConsoleHelper.Info("", "  SupportedStandards: ", $"{string.Join(" ", contract.Manifest.SupportedStandards)}");
                ConsoleHelper.Info("", "            Checksum: ", $"{contract.Nef.CheckSum}");
                ConsoleHelper.Info("", "            Compiler: ", $"{contract.Nef.Compiler}");
                ConsoleHelper.Info("", "          SourceCode: ", $"{contract.Nef.Source}");
                ConsoleHelper.Info("", "              Trusts: ", $"[{string.Join(", ", contract.Manifest.Trusts.Select(s => s.ToJson()?.GetString()))}]");
                if (contract.Manifest.Extra is not null)
                {
                    foreach (var extra in contract.Manifest.Extra.Properties)
                    {
                        ConsoleHelper.Info("", $"  {extra.Key,18}: ", $"{extra.Value?.GetString()}");
                    }
                }
                ConsoleHelper.Info();

                ConsoleHelper.Info("", "-------------", "Groups", "-------------");
                ConsoleHelper.Info();
                if (contract.Manifest.Groups.Length == 0)
                {
                    ConsoleHelper.Info("", "  No Group(s).");
                }
                else
                {
                    foreach (var group in contract.Manifest.Groups)
                    {
                        ConsoleHelper.Info("", "     PubKey: ", $"{group.PubKey}");
                        ConsoleHelper.Info("", "  Signature: ", $"{Convert.ToBase64String(group.Signature)}");
                    }
                }
                ConsoleHelper.Info();

                ConsoleHelper.Info("", "-------------", "Permissions", "-------------");
                ConsoleHelper.Info();
                foreach (var permission in contract.Manifest.Permissions)
                {
                    ConsoleHelper.Info("", "  Contract: ", $"{permission.Contract.ToJson()?.GetString()}");
                    if (permission.Methods.IsWildcard)
                        ConsoleHelper.Info("", "   Methods: ", "*");
                    else
                        ConsoleHelper.Info("", "   Methods: ", $"{string.Join(", ", permission.Methods)}");
                    ConsoleHelper.Info();
                }

                ConsoleHelper.Info("", "-------------", "Methods", "-------------");
                ConsoleHelper.Info();
                foreach (var method in contract.Manifest.Abi.Methods)
                {
                    ConsoleHelper.Info("", "        Name: ", $"{method.Name}");
                    ConsoleHelper.Info("", "        Safe: ", $"{method.Safe}");
                    ConsoleHelper.Info("", "      Offset: ", $"{method.Offset}");
                    ConsoleHelper.Info("", "  Parameters: ", $"[{string.Join(", ", method.Parameters.Select(s => s.Type.ToString()))}]");
                    ConsoleHelper.Info("", "  ReturnType: ", $"{method.ReturnType}");
                    ConsoleHelper.Info();
                }

                ConsoleHelper.Info("", "-------------", "Script", "-------------");
                ConsoleHelper.Info();
                ConsoleHelper.Info($"  {Convert.ToBase64String(contract.Nef.Script.Span)}");
                ConsoleHelper.Info();
                ConsoleHelper.Info("", "--------------------------------");
            }
        }
    }
}
