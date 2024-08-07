---
title: EpicChain Legacy
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







# Transitioning to EpicChain N3: An Overview for Developers

As the blockchain landscape evolves, so does its underlying technology. EpicChain introduces a significant overhaul from EpicChain Legacy, aiming to enrich the ecosystem with enhanced features, improved performance, and a more intuitive development experience. For developers looking to migrate their EpicChain Legacy contracts to EpicChain N3, understanding these differences is crucial. This document outlines the substantial changes and will be an essential guide through your transition.

## Development Environment Updates

### .NET Core Upgrade

- **EpicChain Legacy**: Utilized .NET Core 3.0.
- **EpicChain N3**: Upgraded to .NET Core 6.0, requiring an update to the corresponding SDK.

### Visual Studio Extension

- **EpicChain N3**: The Visual Studio extensions have received updates, necessitating the uninstallation of the old `EpicChainContractPlugin` plugin. Developers must compile and install the latest `EpicChainContractPlugin` to leverage new contract templates filled with major updates.

## Compiler Transformation

EpicChain bids farewell to the former `EpicChain.Compiler.MSIL` in favor of the `nccs` (EpicChain.Compiler.CSharp) compiler, marking a pivotal shift:

- **Visual Basic Support**: Discontinued in favor of C#.
- **Compilation Process**: The ability to directly compile C# code to smart contracts without the need for intermediate language IL. 
- **Compilation Flexibility**: Direct compilation of solutions, projects, and C# files is now supported.
- **C# Features**: Enhanced support for a broader set of C# features.
- **Deterministic Compilation**: Ensured deterministic compilation given consistent code and compiler use.
- **File Format Transition**: Transitioned from `.abi` to manifest files and from `.nvm` to `.nef` files. Also, the compile directory has changed from `bin/debug` to `bin/sc`.

## Contract Template Adjustments

### Namespace Modifications
Transitioning to EpicChain entails adjustments to namespaces, reflecting the updated libraries and frameworks:

- **EpicChain Legacy**: Utilized specific namespaces focused around the smart contract framework and services.
- **EpicChain N3**: Embraces a broader and more integrated approach to namespaces, aligning with the enhanced feature set of N3.

### Contract Feature Enhancements

- **Contract Information**: EpicChain integrates contract information directly into the contract file via C# features. This shift encompasses a broad range of contract metadata such as author, email, trust levels, permission settings, supported standards, and descriptions.

- **Contract Functionalities**:
  - **EpicChain Legacy**: Required explicit declarations for utilizing storage, enabling dynamic calls, and accepting NEP-5 assets.
  - **EpicChain N3**: Offers storage usage and dynamic calls by default. The introduction of `OnNEP17Payment` and `OnNEP11Payment` methods caters to handling NEP-17 and NEP-11 assets, respectively.

- **NEP Standards Support**:
  - **EpicChain Legacy**: Explicit code declaration was necessary to specify supported NEP standards.
  - **EpicChain N3**: Simplified declaration directly within the contract class, enhancing clarity and ease of implementation.

### Declaration of Static Variables

The methodology for declaring static variables, particularly for owner script hashes, has been streamlined in EpicChain N3, offering a clearer and more concise syntactical format.

## Embracing EpicChain N3

The transition from EpicChain Legacy to EpicChain unearths a wealth of opportunities for developers, from enhanced contract capabilities and development tools to improved blockchain performance and flexibility. By familiarizing yourself with the outlined differences and updating your development practices accordingly, you'll be well-equipped to leverage the full potential of EpicChain for your smart contract development endeavors.




# EpicChain Evolution: A Comprehensive Guide to Blockchain Innovations

The transition from EpicChain Legacy to the current EpicChain architecture marks a significant evolution in blockchain technology, introducing more efficiency, security, and usability in smart contract deployment and execution. This guide dives deep into the core changes, including the introduction of new methods, events, and principles underlying these blockchain innovations.

## Methods and Events

### Main Method
- **EpicChain Legacy**: Required developers to write a redundant `main` method for contract method jumping, leading to unnecessary boilerplate code.
- **EpicChain**: The `main` method has been removed, simplifying smart contract structure. Developers no longer need to define this method, streamlining the development process.

### Verification Method
- **EpicChain Legacy**:
  ```solidity
  public static object Main(string method, object[] args) {
    if (Runtime.Trigger == TriggerType.Verification) {
      return IsOwner();
    }
  }
  ```
- **EpicChain**: Introduced an independent `Verify` method, enhancing clarity and simplicity.
  ```solidity
  public static bool Verify() => IsOwner();
  ```

### Method Naming
- **EpicChain Legacy**: Methods required explicit naming, using attributes like `[DisplayName("balanceOf")]` to conform to naming rules.
- **EpicChain**: Automatic lowercase compilation of the first letter in method names, dismissing the need for DisplayName attributes, although still supported for compatibility.

### Contract Initialization and Deployment
- **EpicChain Legacy**: Required manual execution of an initialization method post-deployment.
- **EpicChain**: Includes an automatic `_deploy` method executed post-deployment, streamlining contract initiation.

### Update and Destroy Methods
- **EpicChain Legacy**: Developers had to manually write `Update` and `Destroy` methods.
- **EpicChain**: Incorporates built-in `Update` and `Destroy` methods in the contract template, enhancing ease of management.

### Transfer Event Naming
- **EpicChain Legacy**: Employed 'transfer' as the event name.
- **EpicChain**: Uses 'Transfer', standardizing event names across contracts.

## Permission and Security

### WitnessScope
**EpicChain** introduces the *WitnessScope* concept, refining permissions and security by defaulting to entry contract signature usage and allowing users to modify permissions.

### Permission, Trust, and Contract Groups
**EpicChain N3** upgrades the system with:
- Contract invocation permissions necessitating explicit declarations.
- The integration of contract Groups and Trusts to facilitate security warnings from wallets.
- The *CallFlag* concept, restricting called contract behaviors.

### Safe Methods
**EpicChain** adds `[Safe]` attributes to methods, allowing read-only execution to bolster contract security.

## Contract Framework and Classes

**EpicChain** expands its framework by introducing native contracts such as Ledger, ContractManagement, CryptoLib, and StdLib, among others. These changes significantly uplift the blockchain's functionality, moving interoperable services to native contracts and streamlining operations.

The **Runtime** and **Transaction** classes receive extensive upgrades for better state management and data structure fit. Additionally, a new **Crypto** class enhances the system's cryptographic capabilities.

## Storage

**EpicChain N3** provides enhanced support for `StorageMap`, including static variable declaration and improved search functionalities, optimizing data storage and management.

## TokenSale Operations

### Token Sale Implementation
- **EpicChain Legacy**: Token sale operations were cumbersome, analyzing transaction inputs/outputs manually.
- **EpicChain N3**: Utilizes `OnEEP17Payment` for streamlined TokenSale operations, directly obtaining sender and transfer amounts.

## User Operations

The process for users to participate in Token Sales is streamlined in **EpicChain**, bypassing manual transaction constructions and directly triggering contract methods through asset transfers.

## Exception Handling and Calls

### Exceptions
**EpicChain N3** improves transparency in contract executions by printing exception messages as call results, enhancing debugging and user experience.

### Static and Dynamic Calls
**EpicChain N3** simplifies contract interactions with enhancements in static and dynamic call methodologies, making contract-to-contract communication more straightforward and secure.





















<br/>