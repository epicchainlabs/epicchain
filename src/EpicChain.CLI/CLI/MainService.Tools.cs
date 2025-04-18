// =============================================================================================
//  © Copyright (C) 2021-2025 EpicChain Labs. All rights reserved.
// =============================================================================================
//
//  File: MainService.Tools.cs
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
using EpicChain.Cryptography.ECC;
using EpicChain.Extensions;
using EpicChain.IO;
using EpicChain.SmartContract;
using EpicChain.Wallets;
using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Numerics;
using System.Reflection;
using System.Text;

namespace EpicChain.CLI
{
    partial class MainService
    {
        /// <summary>
        /// Process "parse" command
        /// </summary>
        [ConsoleCommand("parse", Category = "Base Commands", Description = "Parse a value to its possible conversions.")]
        private void OnParseCommand(string value)
        {
            value = Base64Fixed(value);

            var parseFunctions = new Dictionary<string, Func<string, string?>>();
            var methods = GetType().GetMethods(BindingFlags.NonPublic | BindingFlags.Instance);

            foreach (var method in methods)
            {
                var attribute = method.GetCustomAttribute<ParseFunctionAttribute>();
                if (attribute != null)
                {
                    parseFunctions.Add(attribute.Description, (Func<string, string?>)Delegate.CreateDelegate(typeof(Func<string, string?>), this, method));
                }
            }

            var any = false;

            foreach (var pair in parseFunctions)
            {
                var parseMethod = pair.Value;
                var result = parseMethod(value);

                if (result != null)
                {
                    ConsoleHelper.Info("", "-----", pair.Key, "-----");
                    ConsoleHelper.Info("", result, Environment.NewLine);
                    any = true;
                }
            }

            if (!any)
            {
                ConsoleHelper.Warning($"Was not possible to convert: '{value}'");
            }
        }

        /// <summary>
        /// Little-endian to Big-endian
        /// input:  ce616f7f74617e0fc4b805583af2602a238df63f
        /// output: 0x3ff68d232a60f23a5805b8c40f7e61747f6f61ce
        /// </summary>
        [ParseFunction("Little-endian to Big-endian")]
        private string? LittleEndianToBigEndian(string hex)
        {
            try
            {
                if (!IsHex(hex)) return null;
                return "0x" + hex.HexToBytes().Reverse().ToArray().ToHexString();
            }
            catch (FormatException)
            {
                return null;
            }
        }

        /// <summary>
        /// Big-endian to Little-endian
        /// input:  0x3ff68d232a60f23a5805b8c40f7e61747f6f61ce
        /// output: ce616f7f74617e0fc4b805583af2602a238df63f
        /// </summary>
        [ParseFunction("Big-endian to Little-endian")]
        private string? BigEndianToLittleEndian(string hex)
        {
            try
            {
                var hasHexPrefix = hex.StartsWith("0x", StringComparison.InvariantCultureIgnoreCase);
                hex = hasHexPrefix ? hex[2..] : hex;
                if (!hasHexPrefix || !IsHex(hex)) return null;
                return hex.HexToBytes().Reverse().ToArray().ToHexString();
            }
            catch
            {
                return null;
            }
        }

        /// <summary>
        /// String to Base64
        /// input:  Hello World!
        /// output: SGVsbG8gV29ybGQh
        /// </summary>
        [ParseFunction("String to Base64")]
        private string? StringToBase64(string strParam)
        {
            try
            {
                var bytearray = Utility.StrictUTF8.GetBytes(strParam);
                return Convert.ToBase64String(bytearray.AsSpan());
            }
            catch
            {
                return null;
            }
        }

        /// <summary>
        /// Big Integer to Base64
        /// input:  123456
        /// output: QOIB
        /// </summary>
        [ParseFunction("Big Integer to Base64")]
        private string? NumberToBase64(string strParam)
        {
            try
            {
                if (!BigInteger.TryParse(strParam, out var number))
                {
                    return null;
                }
                var bytearray = number.ToByteArray();
                return Convert.ToBase64String(bytearray.AsSpan());
            }
            catch
            {
                return null;
            }
        }

        private static bool IsHex(string str) => str.Length % 2 == 0 && str.All(c => (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F'));

        /// <summary>
        /// Fix for Base64 strings containing unicode
        /// input:  DCECbzTesnBofh/Xng1SofChKkBC7jhVmLxCN1vk\u002B49xa2pBVuezJw==
        /// output: DCECbzTesnBofh/Xng1SofChKkBC7jhVmLxCN1vk+49xa2pBVuezJw==
        /// </summary>
        /// <param name="str">Base64 strings containing unicode</param>
        /// <returns>Correct Base64 string</returns>
        private static string Base64Fixed(string str)
        {
            var sb = new StringBuilder();
            for (var i = 0; i < str.Length; i++)
            {
                if (str[i] == '\\' && i + 5 < str.Length && str[i + 1] == 'u')
                {
                    var hex = str.Substring(i + 2, 4);
                    if (IsHex(hex))
                    {
                        var bts = new byte[2];
                        bts[0] = (byte)int.Parse(hex.Substring(2, 2), NumberStyles.HexNumber);
                        bts[1] = (byte)int.Parse(hex.Substring(0, 2), NumberStyles.HexNumber);
                        sb.Append(Encoding.Unicode.GetString(bts));
                        i += 5;
                    }
                    else
                    {
                        sb.Append(str[i]);
                    }
                }
                else
                {
                    sb.Append(str[i]);
                }
            }
            return sb.ToString();
        }

        /// <summary>
        /// Address to ScriptHash (big-endian)
        /// input:  NejD7DJWzD48ZG4gXKDVZt3QLf1fpNe1PF
        /// output: 0x3ff68d232a60f23a5805b8c40f7e61747f6f61ce
        /// </summary>
        [ParseFunction("Address to ScriptHash (big-endian)")]
        private string? AddressToScripthash(string address)
        {
            try
            {
                var bigEndScript = address.ToScriptHash(EpicChainSystem.Settings.AddressVersion);
                return bigEndScript.ToString();
            }
            catch
            {
                return null;
            }
        }

        /// <summary>
        /// Address to ScriptHash (blittleig-endian)
        /// input:  NejD7DJWzD48ZG4gXKDVZt3QLf1fpNe1PF
        /// output: ce616f7f74617e0fc4b805583af2602a238df63f
        /// </summary>
        [ParseFunction("Address to ScriptHash (little-endian)")]
        private string? AddressToScripthashLE(string address)
        {
            try
            {
                var bigEndScript = address.ToScriptHash(EpicChainSystem.Settings.AddressVersion);
                return bigEndScript.ToArray().ToHexString();
            }
            catch
            {
                return null;
            }
        }

        /// <summary>
        /// Address to Base64
        /// input:  NejD7DJWzD48ZG4gXKDVZt3QLf1fpNe1PF
        /// output: zmFvf3Rhfg/EuAVYOvJgKiON9j8=
        /// </summary>
        [ParseFunction("Address to Base64")]
        private string? AddressToBase64(string address)
        {
            try
            {
                var script = address.ToScriptHash(EpicChainSystem.Settings.AddressVersion);
                return Convert.ToBase64String(script.ToArray().AsSpan());
            }
            catch
            {
                return null;
            }
        }

        /// <summary>
        /// ScriptHash to Address
        /// input:  0x3ff68d232a60f23a5805b8c40f7e61747f6f61ce
        /// output: NejD7DJWzD48ZG4gXKDVZt3QLf1fpNe1PF
        /// </summary>
        [ParseFunction("ScriptHash to Address")]
        private string? ScripthashToAddress(string script)
        {
            try
            {
                UInt160 scriptHash;
                if (script.StartsWith("0x"))
                {
                    if (!UInt160.TryParse(script, out scriptHash))
                    {
                        return null;
                    }
                }
                else
                {
                    if (!UInt160.TryParse(script, out UInt160 littleEndScript))
                    {
                        return null;
                    }
                    var bigEndScript = littleEndScript.ToArray().ToHexString();
                    if (!UInt160.TryParse(bigEndScript, out scriptHash))
                    {
                        return null;
                    }
                }

                return scriptHash.ToAddress(EpicChainSystem.Settings.AddressVersion);
            }
            catch
            {
                return null;
            }
        }

        /// <summary>
        /// Base64 to Address
        /// input:  zmFvf3Rhfg/EuAVYOvJgKiON9j8=
        /// output: NejD7DJWzD48ZG4gXKDVZt3QLf1fpNe1PF
        /// </summary>
        [ParseFunction("Base64 to Address")]
        private string? Base64ToAddress(string bytearray)
        {
            try
            {
                var result = Convert.FromBase64String(bytearray).Reverse().ToArray();
                var hex = result.ToHexString();

                if (!UInt160.TryParse(hex, out var scripthash))
                {
                    return null;
                }

                return scripthash.ToAddress(EpicChainSystem.Settings.AddressVersion);
            }
            catch
            {
                return null;
            }
        }

        /// <summary>
        /// Base64 to String
        /// input:  SGVsbG8gV29ybGQh
        /// output: Hello World!
        /// </summary>
        [ParseFunction("Base64 to String")]
        private string? Base64ToString(string bytearray)
        {
            try
            {
                var result = Convert.FromBase64String(bytearray);
                var utf8String = Utility.StrictUTF8.GetString(result);
                return IsPrintable(utf8String) ? utf8String : null;
            }
            catch
            {
                return null;
            }
        }

        /// <summary>
        /// Base64 to Big Integer
        /// input:  QOIB
        /// output: 123456
        /// </summary>
        [ParseFunction("Base64 to Big Integer")]
        private string? Base64ToNumber(string bytearray)
        {
            try
            {
                var bytes = Convert.FromBase64String(bytearray);
                var number = new BigInteger(bytes);
                return number.ToString();
            }
            catch
            {
                return null;
            }
        }

        /// <summary>
        /// Public Key to Address
        /// input:  03dab84c1243ec01ab2500e1a8c7a1546a26d734628180b0cf64e72bf776536997
        /// output: NU7RJrzNgCSnoPLxmcY7C72fULkpaGiSpJ
        /// </summary>
        [ParseFunction("Public Key to Address")]
        private string? PublicKeyToAddress(string pubKey)
        {
            if (ECPoint.TryParse(pubKey, ECCurve.Secp256r1, out var publicKey) == false)
                return null;
            return Contract.CreateSignatureContract(publicKey)
                .ScriptHash
                .ToAddress(EpicChainSystem.Settings.AddressVersion);
        }

        /// <summary>
        /// WIF to Public Key
        /// </summary>
        [ParseFunction("WIF to Public Key")]
        private string? WIFToPublicKey(string wif)
        {
            try
            {
                var privateKey = Wallet.GetPrivateKeyFromWIF(wif);
                var account = new KeyPair(privateKey);
                return account.PublicKey.ToArray().ToHexString();
            }
            catch (Exception)
            {
                return null;
            }
        }

        /// <summary>
        /// WIF to Address
        /// </summary>
        [ParseFunction("WIF to Address")]
        private string? WIFToAddress(string wif)
        {
            try
            {
                var pubKey = WIFToPublicKey(wif);
                return Contract.CreateSignatureContract(ECPoint.Parse(pubKey, ECCurve.Secp256r1)).ScriptHash.ToAddress(EpicChainSystem.Settings.AddressVersion);
            }
            catch (Exception)
            {
                return null;
            }
        }

        /// <summary>
        /// Base64 Smart Contract Script Analysis
        /// input: DARkYXRhAgBlzR0MFPdcrAXPVptVduMEs2lf1jQjxKIKDBT3XKwFz1abVXbjBLNpX9Y0I8SiChTAHwwIdHJhbnNmZXIMFKNSbimM12LkFYX/8KGvm2ttFxulQWJ9W1I=
        /// output:
        /// PUSHDATA1 data
        /// PUSHINT32 500000000
        /// PUSHDATA1 0x0aa2c42334d65f69b304e376559b56cf05ac5cf7
        /// PUSHDATA1 0x0aa2c42334d65f69b304e376559b56cf05ac5cf7
        /// PUSH4
        /// PACK
        /// PUSH15
        /// PUSHDATA1 transfer
        /// PUSHDATA1 0xa51b176d6b9bafa1f0ff8515e462d78c296e52a3
        /// SYSCALL System.Contract.Call
        /// </summary>
        [ParseFunction("Base64 Smart Contract Script Analysis")]
        private string? ScriptsToOpCode(string base64)
        {
            try
            {
                var bytes = Convert.FromBase64String(base64);
                var sb = new StringBuilder();
                var line = 0;

                foreach (var instruct in new VMInstruction(bytes))
                {
                    if (instruct.OperandSize == 0)
                        sb.AppendFormat("L{0:D04}:{1:X04} {2}{3}", line, instruct.Position, instruct.OpCode, Environment.NewLine);
                    else
                        sb.AppendFormat("L{0:D04}:{1:X04} {2,-10}{3}{4}", line, instruct.Position, instruct.OpCode, instruct.DecodeOperand(), Environment.NewLine);
                    line++;
                }

                return sb.ToString();
            }
            catch
            {
                return null;
            }
        }

        /// <summary>
        /// Checks if the string is null or cannot be printed.
        /// </summary>
        /// <param name="value">
        /// The string to test
        /// </param>
        /// <returns>
        /// Returns false if the string is null, or if it is empty, or if each character cannot be printed;
        /// otherwise, returns true.
        /// </returns>
        private static bool IsPrintable(string value)
        {
            return !string.IsNullOrWhiteSpace(value) && value.Any(c => !char.IsControl(c));
        }
    }
}
