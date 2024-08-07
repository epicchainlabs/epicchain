---
title: EEP Manifest
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';









# Understanding NEF and Manifest Files in EpicChain N3

The EpicChain N3 framework introduces a pivotal shift from its legacy counterpart through the adoption of the EpicChain Executable Format (NEF) and Manifest files. These replacements for the AVM and ABI files herald a new era of smart contract development on EpicChain, streamlining deployment and enhancing the security and clarity of contract interactions. Let's delve into the intricacies of these formats and their integral roles in the EpicChain ecosystem.

## NEF: The EpicChain Executable Format

The NEF file is a crucial component of a smart contract on EpicChain N3, encapsulating various essential details of the contract in a structured manner. Here's a breakdown of its fields:

- **Magic**: A `uint32` identifier unique to the blockchain, serving as a magic number to verify the integrity and origin of the NEF file.
- **Compiler**: A `byte[64]` array providing the name and version of the compiler used to generate the contract, ensuring compatibility and traceability.
- **Source**: A `byte[64]` array specifying the source code address for the contract, offering transparency and verification of the contract's origins.
- **Reserve**: A `byte[2]` field reserved for future expansions and enhancements of the NEF structure, maintaining forward compatibility. This field must be set to `0`.
- **Tokens**: An array of `MethodToken` entries, detailing the method tokens required for dynamic invocation of methods within the contract or external contracts.
- **Script**: The raw bytecode of the contract (`Byte[]`), encapsulating the executable logic of the smart contract.
- **Checksum**: A `uint32` value representing the first four bytes after a double SHA256 hash of the NEF file, safeguarding against file corruption or tampering.

## Manifest: The Contract Declaration

The Manifest file is a declarative blueprint of the smart contract, specifying its functionalities, permissions, and interactions with other contracts. It plays a critical role in determining the contract’s behavior during execution. The fields in a Manifest file include:

- **Name**: A `string` representing the contract's name for identification within the ecosystem.
- **Groups**: An array of `ContractGroup` entries, allowing contracts to establish trust relationships within a group, thereby circumventing user warnings for inter-group contract calls. Each group is linked to a public key and requires a signature for verification.
- **SupportedStandards**: An array of `string[]`, signaling compliance with specific EEP standards, ensuring interoperability and standard behavior.
- **Abi**: The `ContractAbi` structure, adhering to EEP-14, outlines the contract's methods (including names, parameters, and return values) and events in detail.
- **Permissions**: An array of `ContractPermission` entries, defining the scope of contracts and methods the contract intends to interact with. This mechanism enhances security by limiting the contract's external call capabilities.
- **Trusts**: A `WildcardContainer<UInt160>` field, specifying contracts that are deemed trustworthy by this contract, reducing user friction during contract interactions.
- **Extra**: An object field for additional user-defined data, such as developer information, contact details, or any other relevant metadata about the contract.

### The Significance of NEF and Manifest Files

Together, the NEF and Manifest files forge a comprehensive and secure framework for deploying and executing smart contracts on EpicChain N3. They ensure that contracts are executed as intended while providing transparency and traceability. This framework enhances the overall solidity of the blockchain’s infrastructure, promoting a trustable and user-friendly environment for developers and users alike. With these tools, EpicChain N3 aspires to streamline smart contract deployment, foster innovation, and maintain a robust, secure blockchain ecosystem.

















<br/>