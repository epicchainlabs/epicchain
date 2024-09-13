// Copyright (C) 2021-2024 EpicChain Labs.

//
// AssetDescriptor.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Persistence;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Native;
using EpicChain.VM;
using System;

namespace EpicChain.Wallets
{
    /// <summary>
    /// Represents the descriptor of an asset.
    /// </summary>
    public class AssetDescriptor
    {
        /// <summary>
        /// The id of the asset.
        /// </summary>
        public UInt160 AssetId { get; }

        /// <summary>
        /// The name of the asset.
        /// </summary>
        public string AssetName { get; }

        /// <summary>
        /// The symbol of the asset.
        /// </summary>
        public string Symbol { get; }

        /// <summary>
        /// The number of decimal places of the token.
        /// </summary>
        public byte Decimals { get; }

        /// <summary>
        /// Initializes a new instance of the <see cref="AssetDescriptor"/> class.
        /// </summary>
        /// <param name="snapshot">The snapshot used to read data.</param>
        /// <param name="settings">The <see cref="ProtocolSettings"/> used by the <see cref="ApplicationEngine"/>.</param>
        /// <param name="asset_id">The id of the asset.</param>
        public AssetDescriptor(DataCache snapshot, ProtocolSettings settings, UInt160 asset_id)
        {
            var contract = NativeContract.ContractManagement.GetContract(snapshot, asset_id);
            if (contract is null) throw new ArgumentException(null, nameof(asset_id));

            byte[] script;
            using (ScriptBuilder sb = new())
            {
                sb.EmitDynamicCall(asset_id, "decimals", CallFlags.ReadOnly);
                sb.EmitDynamicCall(asset_id, "symbol", CallFlags.ReadOnly);
                script = sb.ToArray();
            }
            using ApplicationEngine engine = ApplicationEngine.Run(script, snapshot, settings: settings, epicpulse: 0_30000000L);
            if (engine.State != VMState.HALT) throw new ArgumentException(null, nameof(asset_id));
            AssetId = asset_id;
            AssetName = contract.Manifest.Name;
            Symbol = engine.ResultStack.Pop().GetString();
            Decimals = (byte)engine.ResultStack.Pop().GetInteger();
        }

        public override string ToString()
        {
            return AssetName;
        }
    }
}
