---
title: Wallet
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';


## Overview
Wallets are integral to interacting with the EpicChain network, enabling users to perform critical functions such as transfers, contract deployments, and asset registrations. This document outlines the technical framework for designing and modifying EpicChain wallets, ensuring adherence to the established protocol standards and patterns.

## Accounts and Addresses in EpicChain

### Conceptual Flow
In EpicChain, an account embodies a smart contract, while an address represents the contract's script. The process of generating an address from a private key involves deriving the public key from the private key and then transforming this public key into an address. The figure below illustrates this conceptual flow:

![Private Key to Address Flow Diagram](#)

### Private Key
A **private key** is a randomly generated number between 1 and `n` (where `n` is a constant slightly less than `2^256`), typically represented as a 256-bit (32 bytes) number. EpicChain supports two main encoding formats for private keys:

- **Hexstring Format**: A hexadecimal character string representing the byte array.
- **WIF (Wallet Import Format)**: This format prefixes the original 32-byte data with `0x80` and suffixes it with `0x01`, followed by Base58Check encoding.

#### Example:

| Format    | Value |
|-----------|-------|
| byte[]    | `[0xc7, 0x13, ... , 0x62]` |
| hexstring | `c7134d6...a6962` |
| wif       | `L3tgppXLg...E16PBncSU` |

### Public Key
Derived from the private key through the ECC algorithm, the **public key** is a point `(X, Y)` on the curve. EpicChain employs the `secp256r1` curve, differing from Bitcoin’s curve. Public keys are available in two formats within EpicChain:

- **Uncompressed Public Key**: `0x04` followed by the `X` and `Y` coordinates (32 bytes each).
- **Compressed Public Key**: `0x02` or `0x03` (indicating the Y-coordinate's parity) followed by the `X` coordinate (32 bytes).

#### Example:

| Format                  | Value |
|-------------------------|-------|
| Private Key             | `c7134d6...a6962` |
| Public Key (Compressed) | `035a928...2b61a` |
| Public Key (Uncompressed) | `045a928...0e113f` |

### Address Generation
The transformation from a public key to an address in EpicChain involves multiple cryptographic operations, leading to a uniquely identifiable string. Recent updates in EpicChain's protocol replace traditional opcode-based verification (e.g., OpCode.CheckSig) with interoperable service calls such as `SysCall "EpicChain.Crypto.CheckSig".hash2uint`.

## Recommendations for Wallet Developers
Developers designing EpicChain wallets should adhere to the following guidelines:
- Ensure compatibility with both Hexstring and WIF private key formats for user convenience.
- Support both compressed and uncompressed public key formats to accommodate various user preferences and technical requirements.
- Implement the updated address generation process that leverages EpicChain’s current cryptographic verification methods, incorporating the necessary interoperable service calls for signature verification.

By following these technical stipulations, developers can create versatile and compliant wallets that enhance user interaction with the EpicChain network, fostering a secure and efficient blockchain ecosystem.




# EpicChain Wallet Address Generation Guide

Creating an EpicChain wallet entails generating a set of cryptographic keys and transforming these into a human-readable address. This document is intended for blockchain engineers and developers involved in creating or integrating EpicChain wallets. It outlines the step-by-step process from generating a private key to obtaining the wallet's address and corresponding script hash.

## Ordinary Address Generation

### 1. Private Key

A private key in EpicChain is a randomly generated value that adheres to specific cryptographic standards. The key formats supported include:

- **Hexstring Format**: A string representation using hexadecimal characters.
- **WIF (Wallet Import Format)**: An encoding format that adds a prefix and suffix around the original data and applies Base58Check encoding.

**Example**:

- Private Key: `087780053c374394a48d685aacf021804fa9fab19537d16194ee215e825942a0`
- Public Key (Compressed): `03cdb067d930fd5adaa6c68545016044aaddec64ba39e548250eaea551172e535c`

### 2. Public Key

The public key is derived from the private key using ECC (Elliptic Curve Cryptography) and can be represented in two formats:

- **Uncompressed**: `0x04` + X-coordinate (32 bytes) + Y-coordinate (32 bytes)
- **Compressed**: `0x02` or `0x03` (depending on the parity of the Y-coordinate) + X-coordinate (32 bytes)

### 3. Address Generation

To generate an address from the public key, follow these steps:

1. **Build a CheckSig script**:
   - Combine `0x0C`, `0x21` + Public Key (Compressed, 33 bytes), `0x41`, and a specific suffix (`0x56e7b327`).
2. **Calculate the script hash**:
   - Perform SHA256 followed by RIPEMD160 on the script, yielding a 20-byte value.
3. **Add version prefix**:
   - Prepend the EpicChain protocol version byte (`0x35` for version 53).
4. **Base58Check encoding**:
   - Apply Base58Check encoding to the aforementioned data to get the address.

**Example of Ordinary Address**:

- Private Key: `087780053c374394a48d685aacf021804fa9fab19537d16194ee215e825942a0`
- Address: `NNLi44dJNXtDNSBkofB48aTVYtb1zZrNEs`

## Multi-Signature Address Generation

For creating a multi-signature wallet, which requires N-of-M signatures for transactions:

1. **Construct the CheckMultiSig script**:
   - Use `emitPush(N)`, public keys of participants (compressed, 33 bytes each), `emitPush(M)`, and a specific suffix (`0x9ed0dc3a`).
2. **Calculate script hash** and **apply version prefix** as described above.

**Example**:

- Private Keys: Two private keys provided.
- Address: `NZ3pqnc1hMN8EHW55ZnCnu8B2wooXJHCyr`

## Note on `emitPush` Usage

The `emitPush` function is vital for encoding numbers within the script. Its opcode varies depending on the number's size, adjusting from pushing small integers directly to handling larger numbers by specifying their size and padding them accordingly.

## Wallet Address and ScriptHash

In EpicChain, each wallet address is tied to a script hash, which acts as a unique identifier on the blockchain. Developers must be adept at converting between address formats and understanding the endianess (big and little) of script hashes for effective wallet interactions.

**Conversion Example**:

- Address: `NUnLWXALK2G6gYa7RadPLRiQYunZHnncxg`
- Big-endian ScriptHash: `0xed7cc6f5f2dd842d384f254bc0c2d58fb69a4761`
- Little-endian ScriptHash: `61479ab68fd5c2c04b254f382d84ddf2f5c67ced`

**Data Converter Tool**: For conversions between address formats and endianess, developers can utilize data conversion tools to ensure accuracy in their implementations.

This guide provides a foundation for developers to navigate the complexities of EpicChain wallet address generation, enabling secure and effective user interactions within the EpicChain ecosystem.




# EpicChain Wallet Files: A Comprehensive Guide

## Introduction
EpicChain facilitates secure and efficient management of digital assets through two principal wallet file types: `.db3` and `.json` (compliant with the EEP6 standard). This guide provides an in-depth technical overview of both wallet file formats, detailing their structure, encryption methodologies, and usage within the EpicChain ecosystem.

### db3 Wallet Files

#### Structure
The `.db3` wallet file format, commonly utilized by exchanges for managing extensive account information, relies on SQLite for data storage. It comprises four primary tables:

1. **Account**
   - `PrivateKeyEncrypted`: AES256 encrypted private key (VarBinary[96]).
   - `PublicKeyHash`: SHA256 and RIPEMD160 hashed public key (Binary[20]) serving as the primary key.

2. **Address**
   - `ScriptHash`: Identifies an address and acts as the primary key (Binary[20]).

3. **Contract**
   - `RawData`: Contains contract data (VarBinary).
   - `ScriptHash` & `PublicKeyHash`: Link to Address and Account tables respectively, serve as primary key and index.

4. **Key**
   - `Name` & `Value`: Stores AES256 attributes like `PasswordHash`, `IV`, `MasterKey`, and `Version`.

#### Encryption
The `.db3` format employs the AES symmetric encryption algorithm, providing a secure mechanism for wallet data encryption and decryption.

### EEP6 Wallet Files

#### Format
EEP6 `.json` wallets are structured as follows, with a detailed example provided:
```json
{
    "name": null,
    "version": "3.0",
    "scrypt": {
        "n": 16384,
        "r": 8,
        "p": 8
    },
    "accounts": [
        {
            "address": "Nf8iN8CABre87oDaDrHSnMAyVoU9jYa2FR",
            "key": "6PYM9DxRY8RMhKHp512xExRVLeB9DSkW2cCKCe65oXgL4tD2kaJX2yb9vD",
            "contract": {
                "script": "DCEDYgBftumtbwC64LbngHbZPDVrSMrEuHXNP0tJzPlOdL5BdHR2qg==",
                "parameters": [{"name": "signature", "type": "Signature"}],
                "deployed": false
            }
        }
    ],
    "extra": null
}
```

#### Encryption
EEP6 wallets leverage the `scrypt` algorithm for key encryption and decryption, with the process divided into encryption and decryption phases, outlined below:

1. **Address Derived From Public Key**
   - Address hash is obtained from the SHA256(SHA256(Address)) operation.
   
2. **scrypt Derived Key Calculation**
   - Generates a 64-byte `derived key` from the user's password and the address hash. This key is split into two halves: `derivedhalf1` and `derivedhalf2`.

3. **Encryption**
   - The private key is XOR-ed with `derivedhalf1`, then AES-encrypted with `derivedhalf2`, resulting in the `encryptedkey`.
   - The `EEP2Key` is formulated using `0x01 + 0x42 + 0xe0 + address hash + encryptedkey` and Base58Check encoded.

4. **Decryption**
   - Using the password and `addresshash`, `scrypt` algorithm recovers `derivedkey`.
   - `Encryptedkey` is decrypted using `AES256` with `derivedhalf2`, then XOR-ed with `derivedhalf1` to retrieve the private key.

#### scrypt Parameters
- `ciphertext`: User's password (UTF-8 format)
- `salt`: Address hash
- Parameters(`n=16384`, `r=8`, `p=8`, `length=64`)

### Converting Wallet Address and ScriptHash
- **Address**: `NUnLWXALK2G6gYa7RadPLRiQYunZHnncxg`
- **Big-endian ScriptHash**: `0xed7cc6f5f2dd842d384f254bc0c2d58fb69a4761`
- **Little-endian ScriptHash**: `61479ab68fd5c2c04b254f382d84ddf2f5c67ced`

Conversion tools, like "Data Converter", are invaluable for transitions between address formats and script hash endianess, further facilitating wallet integration and application development on EpicChain.

# EpicChain Signature and Wallet Implementation Guide

This guide delves into the technical essentials of transaction signing, wallet function implementation, and choosing between full-node and SPV (Simplified Payment Verification) wallet types within the EpicChain ecosystem. It aims to equip developers with the knowledge to incorporate these core functionalities into their applications.

## Transaction Signature

EpicChain utilizes the ECDSA (Elliptic Curve Digital Signature Algorithm) for transaction signatures. The ECC (Elliptic Curve Cryptography) curve used is either `nistP256` or `Secp256r1`, combined with `SHA256` for hashing.

### C# Implementation for Signature:

```csharp
public static byte[] Sign(byte[] message, byte[] prikey, byte[] pubkey)
{
    using (var ecdsa = ECDsa.Create(new ECParameters
    {
        Curve = ECCurve.NamedCurves.nistP256,
        D = prikey,
        Q = new ECPoint
        {
            X = pubkey[..32],
            Y = pubkey[32..]
        }
    }))
    {
        return ecdsa.SignData(message, HashAlgorithmName.SHA256);
    }
}
```

**Example**:

- **Data**: "hello world"
- **PrivateKey**: "f72b8fab85fdcc1bdd20b107e5da1ab4713487bc88fc53b5b134f5eddeaa1a19"
- **PublicKey**: "031f64da8a38e6c1e5423a72ddd6d4fc4a777abe537e5cb5aa0425685cda8e063b"
- **Signature**: A 64-byte signature generated by the above method.

## Wallet Functionality

Developing an EpicChain wallet involves implementing various core functions ranging from key management to transaction broadcasting. Below outlines the essential wallet functionality.

### Key Management

- **Import/Export Wallet File**: Handle account information via `.db3` or `.json` formats.
- **Unlock Wallet**: Authenticate the user for secure access.
- **Create/Import/Export Private Key**: Manage private keys in various formats, ensuring secure generation and storage.
- **Generate/Import/Export Public Key and Address**: Derive public keys and addresses from private keys, enabling transaction sending and receiving.

### Wallet Operations

- **Transfer**: Facilitate EpicChain asset transfers between addresses.
- **Sign**: Digitally sign data, establishing transaction authenticity.
- **Claim EpicPulse**: Retrieve EpicPulse rewards attributed to EpicChain holdings.
- **Get Balance/Transaction**: Provide account balance and transaction history visibility.

### Multi-Signature and Smart Contract

- **Construct Multi-Signature Contract**: Enable transactions that require signatures from multiple parties.
- **Deploy/Test Smart Contract**: Tools for deploying and testing smart contracts within the EpicChain network.

## Wallet Types

### Full-Node Wallet

A full-node wallet, like the EpicChain-CLI, maintains a complete copy of the blockchain data, contributing to the network's peer-to-peer layer. It requires substantial storage due to holding all on-chain data.

### SPV Wallet

Contrarily, SPV wallets prioritize storage efficiency by only saving block headers. They verify transactions using bloom filters and merkle trees, suitable for mobile or lightweight client applications.

#### SPV Wallet Interaction:

1. **Filter Configuration**: The SPV wallet configures a bloom filter and communicates its parameters to a full node.
2. **Transaction Queries**: Transactions are requested from the full node, filtered and verified through the bloom filter and merkle tree path.
3. **Filter Clearance**: Instructs the full node to clear the configured bloom filter after use.

This guide provides a foundational overview for developers looking to integrate wallet functionality and secure transaction capabilities into their EpicChain applications. Proper implementation of these features is crucial for developing robust and user-friendly blockchain solutions.






<br/>