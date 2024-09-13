// Copyright (C) 2021-2024 EpicChain Labs.

//
// EpicPulse.cs is a component of the EpicChain Labs project, founded by xmoohad. This file is
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


using EpicChain.Cryptography.ECC;
using EpicChain.Network.P2P.Payloads;

namespace EpicChain.SmartContract.Native
{
    /// <summary>
    /// Represents the EpicPulse token in the EpicChain system.
    /// </summary>
    public sealed class EpicPulse : FungibleToken<AccountState>
    {
        public override string Symbol => "XPP";
        public override byte Decimals => 8;

        internal EpicPulse()
        {
        }

        internal override ContractTask InitializeAsync(ApplicationEngine engine, Hardfork? hardfork)
        {
            if (hardfork == ActiveIn)
            {
                UInt160 account = Contract.GetBFTAddress(engine.ProtocolSettings.StandbyValidators);
                return Mint(engine, account, engine.ProtocolSettings.InitialEpicPulseDistribution, false);
            }
            return ContractTask.CompletedTask;
        }

        internal override async ContractTask OnPersistAsync(ApplicationEngine engine)
        {
            long totalNetworkFee = 0;
            foreach (Transaction tx in engine.PersistingBlock.Transactions)
            {
                await Burn(engine, tx.Sender, tx.SystemFee + tx.NetworkFee);
                totalNetworkFee += tx.NetworkFee;
            }
            ECPoint[] validators = EpicChain.GetNextBlockValidators(engine.SnapshotCache, engine.ProtocolSettings.ValidatorsCount);
            UInt160 primary = Contract.CreateSignatureRedeemScript(validators[engine.PersistingBlock.PrimaryIndex]).ToScriptHash();
            await Mint(engine, primary, totalNetworkFee, false);
        }
    }
}
