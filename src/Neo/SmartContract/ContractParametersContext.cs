// EpicChain Copyright Project (2021-2024)
// 
// Copyright (c) 2021-2024 EpicChain
// 
// EpicChain is an innovative blockchain network developed and maintained by xmoohad. This copyright project outlines the rights and responsibilities associated with the EpicChain software and its related components.
// 
// 1. Copyright Holder:
//    - xmoohad
// 
// 2. Project Name:
//    - EpicChain
// 
// 3. Project Description:
//    - EpicChain is a decentralized blockchain network that aims to revolutionize the way digital assets are managed, traded, and secured. With its innovative features and robust architecture, EpicChain provides a secure and efficient platform for various decentralized applications (dApps) and digital asset management.
// 
// 4. Copyright Period:
//    - The copyright for the EpicChain software and its related components is valid from 2021 to 2024.
// 
// 5. Copyright Statement:
//    - All rights reserved. No part of the EpicChain software or its related components may be reproduced, distributed, or transmitted in any form or by any means, without the prior written permission of the copyright holder, except in the case of brief quotations embodied in critical reviews and certain other noncommercial uses permitted by copyright law.
// 
// 6. License:
//    - The EpicChain software is licensed under the EpicChain Software License, a custom license that governs the use, distribution, and modification of the software. The EpicChain Software License is designed to promote the free and open development of the EpicChain network while protecting the interests of the copyright holder.
// 
// 7. Open Source:
//    - EpicChain is an open-source project, and its source code is available to the public under the terms of the EpicChain Software License. Developers are encouraged to contribute to the development of EpicChain and create innovative applications on top of the EpicChain network.
// 
// 8. Disclaimer:
//    - The EpicChain software and its related components are provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. In no event shall the copyright holder or contributors be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the EpicChain software or its related components.
// 
// 9. Contact Information:
//    - For inquiries regarding the EpicChain copyright project, please contact xmoohad at [email address].
// 
// 10. Updates:
//     - This copyright project may be updated or modified from time to time to reflect changes in the EpicChain project or to address new legal or regulatory requirements. Users and developers are encouraged to check the latest version of the copyright project periodically.

//
// ContractParametersContext.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography.ECC;
using Neo.IO;
using Neo.Json;
using Neo.Network.P2P.Payloads;
using Neo.Persistence;
using Neo.VM;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using static Neo.SmartContract.Helper;

namespace Neo.SmartContract
{
    /// <summary>
    /// The context used to add witnesses for <see cref="IVerifiable"/>.
    /// </summary>
    public class ContractParametersContext
    {
        private class ContextItem
        {
            public readonly byte[] Script;
            public readonly ContractParameter[] Parameters;
            public readonly Dictionary<ECPoint, byte[]> Signatures;

            public ContextItem(Contract contract)
            {
                this.Script = contract.Script;
                this.Parameters = contract.ParameterList.Select(p => new ContractParameter { Type = p }).ToArray();
                this.Signatures = new Dictionary<ECPoint, byte[]>();
            }

            public ContextItem(JObject json)
            {
                this.Script = Convert.FromBase64String(json["script"].AsString());
                this.Parameters = ((JArray)json["parameters"]).Select(p => ContractParameter.FromJson((JObject)p)).ToArray();
                this.Signatures = ((JObject)json["signatures"]).Properties.Select(p => new
                {
                    PublicKey = ECPoint.Parse(p.Key, ECCurve.Secp256r1),
                    Signature = Convert.FromBase64String(p.Value.AsString())
                }).ToDictionary(p => p.PublicKey, p => p.Signature);
            }

            public JObject ToJson()
            {
                JObject json = new();
                json["script"] = Convert.ToBase64String(Script);
                json["parameters"] = new JArray(Parameters.Select(p => p.ToJson()));
                json["signatures"] = new JObject();
                foreach (var signature in Signatures)
                    json["signatures"][signature.Key.ToString()] = Convert.ToBase64String(signature.Value);
                return json;
            }
        }

        /// <summary>
        /// The <see cref="IVerifiable"/> to add witnesses.
        /// </summary>
        public readonly IVerifiable Verifiable;

        /// <summary>
        /// The snapshot used to read data.
        /// </summary>
        public readonly DataCache Snapshot;

        /// <summary>
        /// The magic number of the network.
        /// </summary>
        public readonly uint Network;

        private readonly Dictionary<UInt160, ContextItem> ContextItems;

        /// <summary>
        /// Determines whether all witnesses are ready to be added.
        /// </summary>
        public bool Completed
        {
            get
            {
                if (ContextItems.Count < ScriptHashes.Count)
                    return false;
                return ContextItems.Values.All(p => p != null && p.Parameters.All(q => q.Value != null));
            }
        }

        private UInt160[] _ScriptHashes = null;
        /// <summary>
        /// Gets the script hashes to be verified for the <see cref="Verifiable"/>.
        /// </summary>
        public IReadOnlyList<UInt160> ScriptHashes => _ScriptHashes ??= Verifiable.GetScriptHashesForVerifying(Snapshot);

        /// <summary>
        /// Initializes a new instance of the <see cref="ContractParametersContext"/> class.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="verifiable">The <see cref="IVerifiable"/> to add witnesses.</param>
        /// <param name="network">The magic number of the network.</param>
        public ContractParametersContext(DataCache snapshot, IVerifiable verifiable, uint network)
        {
            this.Verifiable = verifiable;
            this.Snapshot = snapshot;
            this.ContextItems = new Dictionary<UInt160, ContextItem>();
            this.Network = network;
        }

        /// <summary>
        /// Adds a parameter to the specified witness script.
        /// </summary>
        /// <param name="contract">The contract contains the script.</param>
        /// <param name="index">The index of the parameter.</param>
        /// <param name="parameter">The value of the parameter.</param>
        /// <returns><see langword="true"/> if the parameter is added successfully; otherwise, <see langword="false"/>.</returns>
        public bool Add(Contract contract, int index, object parameter)
        {
            ContextItem item = CreateItem(contract);
            if (item == null) return false;
            item.Parameters[index].Value = parameter;
            return true;
        }

        /// <summary>
        /// Adds parameters to the specified witness script.
        /// </summary>
        /// <param name="contract">The contract contains the script.</param>
        /// <param name="parameters">The values of the parameters.</param>
        /// <returns><see langword="true"/> if the parameters are added successfully; otherwise, <see langword="false"/>.</returns>
        public bool Add(Contract contract, params object[] parameters)
        {
            ContextItem item = CreateItem(contract);
            if (item == null) return false;
            for (int index = 0; index < parameters.Length; index++)
            {
                item.Parameters[index].Value = parameters[index];
            }
            return true;
        }

        /// <summary>
        /// Adds a signature to the specified witness script.
        /// </summary>
        /// <param name="contract">The contract contains the script.</param>
        /// <param name="pubkey">The public key for the signature.</param>
        /// <param name="signature">The signature.</param>
        /// <returns><see langword="true"/> if the signature is added successfully; otherwise, <see langword="false"/>.</returns>
        public bool AddSignature(Contract contract, ECPoint pubkey, byte[] signature)
        {
            if (IsMultiSigContract(contract.Script, out _, out ECPoint[] points))
            {
                if (!points.Contains(pubkey)) return false;
                ContextItem item = CreateItem(contract);
                if (item == null) return false;
                if (item.Parameters.All(p => p.Value != null)) return false;
                if (!item.Signatures.TryAdd(pubkey, signature))
                    return false;
                if (item.Signatures.Count == contract.ParameterList.Length)
                {
                    Dictionary<ECPoint, int> dic = points.Select((p, i) => new
                    {
                        PublicKey = p,
                        Index = i
                    }).ToDictionary(p => p.PublicKey, p => p.Index);
                    byte[][] sigs = item.Signatures.Select(p => new
                    {
                        Signature = p.Value,
                        Index = dic[p.Key]
                    }).OrderByDescending(p => p.Index).Select(p => p.Signature).ToArray();
                    for (int i = 0; i < sigs.Length; i++)
                        if (!Add(contract, i, sigs[i]))
                            throw new InvalidOperationException();
                }
                return true;
            }
            else
            {
                int index = -1;
                for (int i = 0; i < contract.ParameterList.Length; i++)
                    if (contract.ParameterList[i] == ContractParameterType.Signature)
                        if (index >= 0)
                            throw new NotSupportedException();
                        else
                            index = i;

                if (index == -1)
                {
                    // unable to find ContractParameterType.Signature in contract.ParameterList
                    // return now to prevent array index out of bounds exception
                    return false;
                }
                ContextItem item = CreateItem(contract);
                if (item == null) return false;
                if (!item.Signatures.TryAdd(pubkey, signature))
                    return false;
                item.Parameters[index].Value = signature;
                return true;
            }
        }

        private ContextItem CreateItem(Contract contract)
        {
            if (ContextItems.TryGetValue(contract.ScriptHash, out ContextItem item))
                return item;
            if (!ScriptHashes.Contains(contract.ScriptHash))
                return null;
            item = new ContextItem(contract);
            ContextItems.Add(contract.ScriptHash, item);
            return item;
        }

        /// <summary>
        /// Converts the context from a JSON object.
        /// </summary>
        /// <param name="json">The context represented by a JSON object.</param>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <returns>The converted context.</returns>
        public static ContractParametersContext FromJson(JObject json, DataCache snapshot)
        {
            var type = typeof(ContractParametersContext).GetTypeInfo().Assembly.GetType(json["type"].AsString());
            if (!typeof(IVerifiable).IsAssignableFrom(type)) throw new FormatException();

            var verifiable = (IVerifiable)Activator.CreateInstance(type);
            byte[] data = Convert.FromBase64String(json["data"].AsString());
            MemoryReader reader = new(data);
            verifiable.DeserializeUnsigned(ref reader);
            if (json.ContainsProperty("hash"))
            {
                UInt256 hash = UInt256.Parse(json["hash"].GetString());
                if (hash != verifiable.Hash) throw new FormatException();
            }
            ContractParametersContext context = new(snapshot, verifiable, (uint)json["network"].GetInt32());
            foreach (var (key, value) in ((JObject)json["items"]).Properties)
            {
                context.ContextItems.Add(UInt160.Parse(key), new ContextItem((JObject)value));
            }
            return context;
        }

        /// <summary>
        /// Gets the parameter with the specified index from the witness script.
        /// </summary>
        /// <param name="scriptHash">The hash of the witness script.</param>
        /// <param name="index">The specified index.</param>
        /// <returns>The parameter with the specified index.</returns>
        public ContractParameter GetParameter(UInt160 scriptHash, int index)
        {
            return GetParameters(scriptHash)?[index];
        }

        /// <summary>
        /// Gets the parameters from the witness script.
        /// </summary>
        /// <param name="scriptHash">The hash of the witness script.</param>
        /// <returns>The parameters from the witness script.</returns>
        public IReadOnlyList<ContractParameter> GetParameters(UInt160 scriptHash)
        {
            if (!ContextItems.TryGetValue(scriptHash, out ContextItem item))
                return null;
            return item.Parameters;
        }

        /// <summary>
        /// Gets the signatures from the witness script.
        /// </summary>
        /// <param name="scriptHash">The hash of the witness script.</param>
        /// <returns>The signatures from the witness script.</returns>
        public IReadOnlyDictionary<ECPoint, byte[]> GetSignatures(UInt160 scriptHash)
        {
            if (!ContextItems.TryGetValue(scriptHash, out ContextItem item))
                return null;
            return item.Signatures;
        }

        /// <summary>
        /// Gets the witness script with the specified hash.
        /// </summary>
        /// <param name="scriptHash">The hash of the witness script.</param>
        /// <returns>The witness script.</returns>
        public byte[] GetScript(UInt160 scriptHash)
        {
            if (!ContextItems.TryGetValue(scriptHash, out ContextItem item))
                return null;
            return item.Script;
        }

        /// <summary>
        /// Gets the witnesses of the <see cref="Verifiable"/>.
        /// </summary>
        /// <returns>The witnesses of the <see cref="Verifiable"/>.</returns>
        /// <exception cref="InvalidOperationException">The witnesses are not ready to be added.</exception>
        public Witness[] GetWitnesses()
        {
            if (!Completed) throw new InvalidOperationException();
            Witness[] witnesses = new Witness[ScriptHashes.Count];
            for (int i = 0; i < ScriptHashes.Count; i++)
            {
                ContextItem item = ContextItems[ScriptHashes[i]];
                using ScriptBuilder sb = new();
                for (int j = item.Parameters.Length - 1; j >= 0; j--)
                {
                    sb.EmitPush(item.Parameters[j]);
                }
                witnesses[i] = new Witness
                {
                    InvocationScript = sb.ToArray(),
                    VerificationScript = item.Script ?? Array.Empty<byte>()
                };
            }
            return witnesses;
        }

        /// <summary>
        /// Parses the context from a JSON <see cref="string"/>.
        /// </summary>
        /// <param name="value">The JSON <see cref="string"/>.</param>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <returns>The parsed context.</returns>
        public static ContractParametersContext Parse(string value, DataCache snapshot)
        {
            return FromJson((JObject)JToken.Parse(value), snapshot);
        }

        /// <summary>
        /// Converts the context to a JSON object.
        /// </summary>
        /// <returns>The context represented by a JSON object.</returns>
        public JObject ToJson()
        {
            JObject json = new();
            json["type"] = Verifiable.GetType().FullName;
            json["hash"] = Verifiable.Hash.ToString();
            using (MemoryStream ms = new())
            using (BinaryWriter writer = new(ms, Utility.StrictUTF8))
            {
                Verifiable.SerializeUnsigned(writer);
                writer.Flush();
                json["data"] = Convert.ToBase64String(ms.ToArray());
            }
            json["items"] = new JObject();
            foreach (var item in ContextItems)
                json["items"][item.Key.ToString()] = item.Value.ToJson();
            json["network"] = Network;
            return json;
        }

        public override string ToString()
        {
            return ToJson().ToString();
        }
    }
}
