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
// KeyPair.cs file belongs to the neo project and is free
// software distributed under the MIT software license, see the
// accompanying file LICENSE in the main directory of the
// repository or http://www.opensource.org/licenses/mit-license.php
// for more details.
//
// Redistribution and use in source and binary forms with or without
// modifications are permitted.

using Neo.Cryptography;
using Neo.SmartContract;
using Neo.Wallets.NEP6;
using Org.BouncyCastle.Crypto.Generators;
using System;
using System.Security.Cryptography;
using System.Text;
using static Neo.Wallets.Helper;

namespace Neo.Wallets
{
    /// <summary>
    /// Represents a private/public key pair in wallets.
    /// </summary>
    public class KeyPair : IEquatable<KeyPair>
    {
        /// <summary>
        /// The private key.
        /// </summary>
        public readonly byte[] PrivateKey;

        /// <summary>
        /// The public key.
        /// </summary>
        public readonly Cryptography.ECC.ECPoint PublicKey;

        /// <summary>
        /// The hash of the public key.
        /// </summary>
        public UInt160 PublicKeyHash => PublicKey.EncodePoint(true).ToScriptHash();

        /// <summary>
        /// Initializes a new instance of the <see cref="KeyPair"/> class.
        /// </summary>
        /// <param name="privateKey">The private key in the <see cref="KeyPair"/>.</param>
        public KeyPair(byte[] privateKey)
        {
            if (privateKey.Length != 32 && privateKey.Length != 96 && privateKey.Length != 104)
                throw new ArgumentException(null, nameof(privateKey));
            this.PrivateKey = privateKey[^32..];
            if (privateKey.Length == 32)
            {
                this.PublicKey = Cryptography.ECC.ECCurve.Secp256r1.G * privateKey;
            }
            else
            {
                this.PublicKey = Cryptography.ECC.ECPoint.FromBytes(privateKey, Cryptography.ECC.ECCurve.Secp256r1);
            }
        }

        public bool Equals(KeyPair other)
        {
            if (ReferenceEquals(this, other)) return true;
            if (other is null) return false;
            return PublicKey.Equals(other.PublicKey);
        }

        public override bool Equals(object obj)
        {
            return Equals(obj as KeyPair);
        }

        /// <summary>
        /// Exports the private key in WIF format.
        /// </summary>
        /// <returns>The private key in WIF format.</returns>
        public string Export()
        {
            Span<byte> data = stackalloc byte[34];
            data[0] = 0x80;
            PrivateKey.CopyTo(data[1..]);
            data[33] = 0x01;
            string wif = Base58.Base58CheckEncode(data);
            data.Clear();
            return wif;
        }

        /// <summary>
        /// Exports the private key in NEP-2 format.
        /// </summary>
        /// <param name="passphrase">The passphrase of the private key.</param>
        /// <param name="version">The address version.</param>
        /// <param name="N">The N field of the <see cref="ScryptParameters"/> to be used.</param>
        /// <param name="r">The R field of the <see cref="ScryptParameters"/> to be used.</param>
        /// <param name="p">The P field of the <see cref="ScryptParameters"/> to be used.</param>
        /// <returns>The private key in NEP-2 format.</returns>
        public string Export(string passphrase, byte version, int N = 16384, int r = 8, int p = 8)
        {
            byte[] passphrasedata = Encoding.UTF8.GetBytes(passphrase);
            try
            {
                return Export(passphrasedata, version, N, r, p);
            }
            finally
            {
                passphrasedata.AsSpan().Clear();
            }
        }

        /// <summary>
        /// Exports the private key in NEP-2 format.
        /// </summary>
        /// <param name="passphrase">The passphrase of the private key.</param>
        /// <param name="version">The address version.</param>
        /// <param name="N">The N field of the <see cref="ScryptParameters"/> to be used.</param>
        /// <param name="r">The R field of the <see cref="ScryptParameters"/> to be used.</param>
        /// <param name="p">The P field of the <see cref="ScryptParameters"/> to be used.</param>
        /// <returns>The private key in NEP-2 format.</returns>
        public string Export(byte[] passphrase, byte version, int N = 16384, int r = 8, int p = 8)
        {
            UInt160 script_hash = Contract.CreateSignatureRedeemScript(PublicKey).ToScriptHash();
            string address = script_hash.ToAddress(version);
            byte[] addresshash = Encoding.ASCII.GetBytes(address).Sha256().Sha256()[..4];
            byte[] derivedkey = SCrypt.Generate(passphrase, addresshash, N, r, p, 64);
            byte[] derivedhalf1 = derivedkey[..32];
            byte[] derivedhalf2 = derivedkey[32..];
            byte[] encryptedkey = Encrypt(XOR(PrivateKey, derivedhalf1), derivedhalf2);
            Span<byte> buffer = stackalloc byte[39];
            buffer[0] = 0x01;
            buffer[1] = 0x42;
            buffer[2] = 0xe0;
            addresshash.CopyTo(buffer[3..]);
            encryptedkey.CopyTo(buffer[7..]);
            return Base58.Base58CheckEncode(buffer);
        }

        private static byte[] Encrypt(byte[] data, byte[] key)
        {
            using Aes aes = Aes.Create();
            aes.Key = key;
            aes.Mode = CipherMode.ECB;
            aes.Padding = PaddingMode.None;
            using ICryptoTransform encryptor = aes.CreateEncryptor();
            return encryptor.TransformFinalBlock(data, 0, data.Length);
        }

        public override int GetHashCode()
        {
            return PublicKey.GetHashCode();
        }

        public override string ToString()
        {
            return PublicKey.ToString();
        }
    }
}
