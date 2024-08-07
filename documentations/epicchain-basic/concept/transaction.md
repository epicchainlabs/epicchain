---
title: Transaction
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';



# EpicChain Transactions: An In-depth Exploration

Transactions represent the cornerstone of interactions within the EpicChain network, facilitating wallet operations, smart contracts, and account interactions. This guide provides a detailed overview of the transaction data structure on the EpicChain blockchain, underscoring the pivotal roles of various components in the transaction process.

## Transaction Structure

The transaction data structure is meticulously designed to ensure seamless execution of transactions across the EpicChain network.

- **Version**: Indicates the transaction structure version to maintain backward compatibility. The current version is `0`.
- **Nonce**: A random number enhancing transaction uniqueness.
- **Sysfee**: The system fee compensates for network resource consumption.
- **Netfee**: The network fee rewards validators for packaging transactions.
- **ValidUntilBlock**: Defines the transaction's validity period.
- **Signers**: Specifies the transaction sender and the effective scope of the signature.
- **Attributes**: Additional transaction details.
- **Script**: The executable script on the EpicChainVM.
- **Witnesses**: Scripts validating the transaction's integrity.

### Version

The `version` field enables future updates to the transaction structure while ensuring backward compatibility. 

### Signers

The `Signers` field prioritizes the transaction sender's script hash. It transitions from the UTXO model of EpicChain N2 to a more refined approach in EpicChain , where native assets are treated as NEP-17 tokens.

### Sysfee and Netfee

- **Sysfee**: System fees are contingent upon the transaction's script – its size and the complexity of EpicChainVM instructions executed. This fee structure evolves from EpicChain , removing the 10 EpicPulse free system fee. The fee is calculated based on the opcode set used in the script.

- **Netfee**: The network fee, paid for transaction submission, directly rewards consensus node efforts in block generation. It includes a base fee per transaction, with transactions requiring a fee greater or equivalent to this base fee for execution.

### Attributes

Transactions can include up to 16 additional attributes, each defined by its usage type, and internal and external data size.

### Script

The `script` executed on the EpicChainVM defines the transaction's effects. This could range from transferring assets to more complex operations involving smart contracts.

### Witnesses

`Witnesses` ensure a transaction's validity and integrity, consisting of `InvocationScript` and `VerificationScript`. The verification script, often derived from the wallet account, plays a crucial role in transaction validation.

#### Invocation Script

The invocation script adds signatures to the transaction, supporting multiple signatures for multi-signature contracts through sequential operations.

#### Verification Script

This script includes standard address scripts and multi-signature address scripts, obtainable directly from wallet accounts. It may also serve as a custom authentication contract script.

## Practical Implications

Understanding the intricate structure of EpicChain transactions is essential for developers aiming to build robust applications on this platform. By delving into the mechanisms of `Sysfee`, `Netfee`, `Signers`, and `Witnesses`, developers can optimize transaction efficiency, ensuring seamless interactions with the EpicChain network.

This guide serves as a foundational resource for those looking to navigate the complexities of blockchain transactions on EpicChain, ensuring that each component is leveraged to its full potential for the development of innovative blockchain solutions.



# Understanding Transaction Serialization in EpicChain

Transaction serialization is a critical process in blockchain technology, allowing transactions to be encoded for network transmission. In EpicChain, this process adheres to a specific sequence and format, ensuring efficient and secure communication within the network. This guide delves into the serialization process of transactions in EpicChain, highlighting key considerations for developers.

### Serialization Order and Format

The order in which fields of a transaction are serialized is pivotal for maintaining the integrity and interoperability of data across the EpicChain network. The fields are serialized in the following sequence:

1. **Version**: Transaction version identifier.
2. **Nonce**: A random number enhancing security by preventing replay attacks.
3. **SystemFee**: The fee paid for computational resources used during transaction execution.
4. **NetworkFee**: The fee paid to validators for processing the transaction.
5. **ValidUntilBlock**: Designates the validity period of the transaction.
6. **Signers**: Serialized with the `WriteVarInt(length)` prefix to indicate the quantity, followed by serialization of each signer.
7. **Attributes**: Similar to Signers, attributes require a `WriteVarInt(length)` prefix, followed by the serialization of attribute elements.
8. **Script**: Contains the executable code, prefixed by `WriteVarInt(length)` indicating the script byte array length.
9. **Witnesses**: Serialized with a `WriteVarInt(length)` prefix, detailing the transaction's verification scripts.

### WriteVarInt Method

The `WriteVarInt` method dynamically adjusts the storage size based on the value, optimizing the use of storage space and ensuring efficient data transmission. The encoding logic is as follows:

- For values less than `0xFD`, store as `byte(value)`.
- For values up to `0xFFFF`, prefix with `0xFD` followed by the value as `ushort`.
- For values up to `0xFFFFFFFF`, prefix with `0xFE` followed by the value as `uint`.
- For values larger than `0xFFFFFFFF`, prefix with `0xFF` followed by the value.

### Transaction Signature

A transaction's signature is generated by signing the serialized data (excluding the witnesses part) using the ECDSA method. This signature is then incorporated into the transaction's `witnesses` field, completing the transaction structure.

### Example JSON-formatted Transaction

Below is an example of a serialized transaction in JSON format, demonstrating the integration of various components, from signers to the executable script, all encoded for network transmission:

```json
{
  "hash": "0xd2b24b57ea05821766877241a51e17eae06ed66a6c72adb5727f8ba701d995be",
  "size": 265,
  "version": 0,
  "nonce": 739807055,
  "sender": "NMDf1XCbioM7ZrPZAdQKQt8nnx3fWr1wdr",
  "sys_fee": "9007810",
  "net_fee": "1264390",
  "valid_until_block": 2102402,
  "signers": [
    {
      "account": "0xdf93ea5a0283c01e8cdfae891ff700faad70500e",
      "scopes": "FeeOnly"
    },
    {
      "account": "0xdf93ea5a0283c01e8cdfae891ff700faad70500e",
      "scopes": "CalledByEntry"
    }
  ],
  "attributes": [],
  "script": "EQwUDlBwrfoA9x+Jrt+MHsCDAlrqk98MFA5QcK36APcfia7fjB7AgwJa6pPfE8AMCHRyYW5zZmVyDBSJdyDYzXb08Aq/o3wO3YicII/em0FifVtSOA==",
  "witnesses": [
    {
      "invocation": "DEDy/g4Lt+FTMBHHF84TSVXG9aSNODOjj0aPaJq8uOc6eMzqr8rARqpB4gWGXNfzLyh9qKvE++6f6XoZeaEoUPeH",
      "verification": "DCECCJr46zTvjDE0jA5v5jrry4Wi8Wm7Agjf6zGH/7/1EVELQQqQatQ="
    }
  ]
}
```

This example illustrates the composition and encoding of a transaction, ready for transmission within the EpicChain network. By adhering to the outlined serialization process, developers can ensure their applications interact seamlessly with the EpicChain blockchain, maintaining the fidelity and security of transactions.



# Advanced Signature Control in EpicChain: WitnessScope and WitnessRule

EpicChain N3 introduces enhanced controls over transaction signatures through the `WitnessScope` and `WitnessRule` mechanisms. This advancement allows users more granular control over their signatures, specifying where and how these signatures can be used, thereby enhancing security and preventing unauthorized use of user signatures in contracts.

## Understanding Signature Scopes

The `signers` field in a transaction structure undergoes modification to accommodate the specification of signature scopes. The `scopes` field defines the effective range of the signature, with the following types available:

- **None (0x00)**: Limits the signature's use to transactions only, disabling its effectiveness in contracts.
- **CalledByEntry (0x01)**: Restricts signature effectiveness to the contract script directly called by the entry.
- **CustomContracts (0x10)**: Allows the signature to be effective only for specified contract scripts. It can work in conjunction with `CalledByEntry`.
- **CustomGroups (0x20)**: The signature is valid for contracts within a specified group, and can also be combined with `CalledByEntry`.
- **Global (0x80)**: Makes the signature globally effective, bearing high risk as contracts may transfer all address-held assets. This scope is recommended only for highly trusted contracts.
- **WitnessRules (0x40)**: Requires defining specific rules and scopes for the signature's application.

## WitnessRule: Defining Signature Application Rules

The `WitnessRule` mechanism allows defining actions (`Allow` or `Deny`) based on conditions specified by `WitnessCondition`. This logical structure ensures that signatures are validated against the defined conditions before their application.

### WitnessCondition Types

- **Boolean**: Directly evaluates to true or false.
- **Not**: Logical negation of another condition.
- **And**: Logical conjunction requiring all nested conditions to be true.
- **Or**: Logical disjunction requiring at least one nested condition to be true.
- **ScriptHash**: Checks if the current contract matches a specified hash.
- **Group**: Verifies if the current contract's public key matches a specified group.
- **CalledByEntry**: Verifies if the current contract is called by entry, aligning with `CallByEntry` scope.
- **CalledByContract**: Checks if the immediate caller contract's hash matches a specified value.
- **CalledByGroup**: Verifies if the public key of the immediate caller contract matches a specified group.

## Practical JSON Example

Below is a practical JSON-format example demonstrating how to define the `WitnessScope` and `WitnessRule` in a transaction:

```json
{
    "account": "NdUL5oDPD159KeFpD5A9zw5xNF1xLX6nLT",
    "scopes": "WitnessRules",
    "rules": [{
        "action": "Allow",
        "condition": {
            "type": "Not",
            "expression": {
                "type": "And",
                "expressions": [{
                        "type": "ScriptHash",
                        "hash": "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"
                    }, {
                        "type": "CalledByEntry"
                    }
                ]
            }
        }
    }]
}
```

This example showcases a transaction rule set to `Allow`, provided that the transaction does not meet both conditions of being from a specified contract hash and directly called by entry. Such detailed specifications empower users with precise control over their transaction signatures, ensuring enhanced security across the EpicChain network.

As the blockchain ecosystem evolves, implementing such controls becomes crucial for fostering trust and security in decentralized environments. EpicChain N3's introduction of `WitnessScope` and `WitnessRule` heralds a significant step forward in achieving these objectives.





<br/>