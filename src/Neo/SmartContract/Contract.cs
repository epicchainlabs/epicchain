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
// Contract.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography.ECC;
using Neo.VM;
using System;
using System.Collections.Generic;
using System.Linq;

namespace Neo.SmartContract
{
    /// <summary>
    /// Represents a contract that can be invoked.
    /// </summary>
    public class Contract
    {
        /// <summary>
        /// The script of the contract.
        /// </summary>
        public byte[] Script;

        /// <summary>
        /// The parameters of the contract.
        /// </summary>
        public ContractParameterType[] ParameterList;

        private UInt160 _scriptHash;
        /// <summary>
        /// The hash of the contract.
        /// </summary>
        public virtual UInt160 ScriptHash
        {
            get
            {
                if (_scriptHash == null)
                {
                    _scriptHash = Script.ToScriptHash();
                }
                return _scriptHash;
            }
        }

        /// <summary>
        /// Creates a new instance of the <see cref="Contract"/> class.
        /// </summary>
        /// <param name="parameterList">The parameters of the contract.</param>
        /// <param name="redeemScript">The script of the contract.</param>
        /// <returns>The created contract.</returns>
        public static Contract Create(ContractParameterType[] parameterList, byte[] redeemScript)
        {
            return new Contract
            {
                Script = redeemScript,
                ParameterList = parameterList
            };
        }

        /// <summary>
        /// Constructs a special contract with empty script, will get the script with scriptHash from blockchain when doing the verification.
        /// </summary>
        /// <param name="scriptHash">The hash of the contract.</param>
        /// <param name="parameterList">The parameters of the contract.</param>
        /// <returns>The created contract.</returns>
        public static Contract Create(UInt160 scriptHash, params ContractParameterType[] parameterList)
        {
            return new Contract
            {
                Script = Array.Empty<byte>(),
                _scriptHash = scriptHash,
                ParameterList = parameterList
            };
        }

        /// <summary>
        /// Creates a multi-sig contract.
        /// </summary>
        /// <param name="m">The minimum number of correct signatures that need to be provided in order for the verification to pass.</param>
        /// <param name="publicKeys">The public keys of the contract.</param>
        /// <returns>The created contract.</returns>
        public static Contract CreateMultiSigContract(int m, IReadOnlyCollection<ECPoint> publicKeys)
        {
            return new Contract
            {
                Script = CreateMultiSigRedeemScript(m, publicKeys),
                ParameterList = Enumerable.Repeat(ContractParameterType.Signature, m).ToArray()
            };
        }

        /// <summary>
        /// Creates the script of multi-sig contract.
        /// </summary>
        /// <param name="m">The minimum number of correct signatures that need to be provided in order for the verification to pass.</param>
        /// <param name="publicKeys">The public keys of the contract.</param>
        /// <returns>The created script.</returns>
        public static byte[] CreateMultiSigRedeemScript(int m, IReadOnlyCollection<ECPoint> publicKeys)
        {
            if (!(1 <= m && m <= publicKeys.Count && publicKeys.Count <= 1024))
                throw new ArgumentException();
            using ScriptBuilder sb = new();
            sb.EmitPush(m);
            foreach (ECPoint publicKey in publicKeys.OrderBy(p => p))
            {
                sb.EmitPush(publicKey.EncodePoint(true));
            }
            sb.EmitPush(publicKeys.Count);
            sb.EmitSysCall(ApplicationEngine.System_Crypto_CheckMultisig);
            return sb.ToArray();
        }

        /// <summary>
        /// Creates a signature contract.
        /// </summary>
        /// <param name="publicKey">The public key of the contract.</param>
        /// <returns>The created contract.</returns>
        public static Contract CreateSignatureContract(ECPoint publicKey)
        {
            return new Contract
            {
                Script = CreateSignatureRedeemScript(publicKey),
                ParameterList = new[] { ContractParameterType.Signature }
            };
        }

        /// <summary>
        /// Creates the script of signature contract.
        /// </summary>
        /// <param name="publicKey">The public key of the contract.</param>
        /// <returns>The created script.</returns>
        public static byte[] CreateSignatureRedeemScript(ECPoint publicKey)
        {
            using ScriptBuilder sb = new();
            sb.EmitPush(publicKey.EncodePoint(true));
            sb.EmitSysCall(ApplicationEngine.System_Crypto_CheckSig);
            return sb.ToArray();
        }

        /// <summary>
        /// Gets the BFT address for the specified public keys.
        /// </summary>
        /// <param name="pubkeys">The public keys to be used.</param>
        /// <returns>The BFT address.</returns>
        public static UInt160 GetBFTAddress(IReadOnlyCollection<ECPoint> pubkeys)
        {
            return CreateMultiSigRedeemScript(pubkeys.Count - (pubkeys.Count - 1) / 3, pubkeys).ToScriptHash();
        }
    }
}
