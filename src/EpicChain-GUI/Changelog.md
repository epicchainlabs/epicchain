# EpicChain-GUI

Welcome to EpicChain-GUI, the premier graphical user interface for interacting with the EpicChain blockchain. This document outlines the latest updates and features available in EpicChain-GUI, including compatibility details and installation instructions. Whether you're a developer or an end user, EpicChain-GUI offers a comprehensive suite of tools for managing and interacting with the EpicChain blockchain.

## [Latest Version]

### Compatibility
This version of EpicChain-GUI is fully compatible with EpicChain, leveraging the core functionalities of [EpicChain CLI v3.0.0-preview3](https://github.com/epicchainlabs/epicchain-cli/releases/tag/v3.0.0-preview3).

### New Features

#### 1. Blockchain Data
- **Asset Management**: View a detailed list of assets along with related transactions for comprehensive asset tracking.
- **Opcode Conversion**: Convert Hex to Opcode for Witness and script elements within transactions, facilitating easier analysis and debugging.

#### 2. Wallet Enhancements
- **Multi-Signature Support**: Added support for multi-signature addresses to enhance security and control over transactions.
- **Multiple Transfers**: Enables users to perform multiple asset transfers in a single operation, streamlining the transaction process.
- **Batch Transfer**: Support for batch processing of asset transfers, improving efficiency for large-scale transactions.
- **Pending Transactions**: Display detailed information for pending transactions, providing visibility into transaction statuses.
- **Password Management**: New functionality to change wallet passwords, enhancing security and user control.

#### 3. Smart Contract Support
- **Contract Details**: Access detailed information about deployed smart contracts, including metadata and state.
- **Transaction Analysis**: Analyze and display all transactions related to a specific smart contract for better tracking and management.
- **Parameter Switching**: Support for switching data types for parameters when invoking smart contracts, improving flexibility and usability.
- **Contract Management**: Support for migrating and destroying smart contracts, offering comprehensive management capabilities.

#### 4. Advanced Features
- **Multi-Signature Transactions**: Enable signing and broadcasting of transactions for multi-signature addresses, providing enhanced security and flexibility.

### Updates
- **Backend Compatibility**: Updated the backend node to ensure full compatibility with EpicChain.
- **Transaction Analysis**: Improved transaction analysis to better identify and categorize contract-related transactions.
- **Gas Claiming**: Implemented measures to prevent duplicate gas claims, ensuring fair usage and preventing exploitation.

## [Initial Release of EpicChain-GUI] for EpicChain

### Announcements
This release marks the initial launch of EpicChain-GUI, designed to be fully compatible with EpicChain. As this is the first version, it includes the essential features necessary for effective blockchain interaction. We encourage early adopters to explore these features and provide feedback to help shape future updates.

### Dependencies
Ensure that the following dependencies are installed:
- [.NET Core 6.0](https://dotnet.microsoft.com/download) for running the application.
- [Node.js](https://nodejs.org/) for managing JavaScript dependencies.

### Features Added

#### 1. Blockchain Data
- **Block Information**: View a comprehensive list of blocks and detailed information for each block, including associated transactions.
- **Block Search**: Search for blocks by their height to quickly locate specific blocks.
- **Transaction Tracking**: Access lists of confirmed and pending transactions on the blockchain.
- **Transaction Details**: Detailed information for each transaction, including application logs and basic data.
- **Asset Issuance**: Retrieve and view details of issued assets and related transactions.

#### 2. Wallet
- **Wallet Management**: Create new wallets or open existing ones with ease.
- **Wallet Import**: Import wallets using private keys or encrypted NEP2 keys.
- **Balance Inquiry**: Check balances for all assets within a wallet.
- **Wallet Details**: Display detailed wallet information, including public keys, private keys, and Wallet Import Format (WIF).
- **Asset Transfer**: Perform asset transfers between accounts.
- **Transaction Queries**: Query and view related transactions.
- **Account Management**: Add or remove accounts/addresses from the wallet.

#### 3. Smart Contract Support
- **Contract Search**: Locate smart contracts by their hash.
- **Contract Information**: View basic information and metadata about smart contracts.
- **Contract Deployment**: Deploy new smart contracts using NEF and manifest files.
- **Contract Invocation**: Invoke smart contracts via manifest files, facilitating interaction with deployed contracts.

#### 4. Advanced Features
- **Voting and Registration**: Vote for consensus nodes and register as a consensus node candidate.
- **Data Conversion**: Convert between string and hexString formats, as well as wallet addresses and script hashes.

#### 5. Settings
- **Multi-Language Support**: Available in both Chinese and English to accommodate a diverse user base.
- **Network Switching**: Switch between testnet and private chain networks easily.

### Download and Verification

| File Hash                                                    |
| ------------------------------------------------------------ |
| **File Name**: EpicChain-GUI-1.0.0.exe <br/> **SHA1**: 21e93d28ac4b2a30562e494a26732a71de6b8bf4 <br/> **SHA256**: e6def79b7df30a3c03fea90404946322822547371a68052685c2d574f7493e7d |
| **File Name**: EpicChain-GUI-1.0.0.dmg <br/> **SHA1**: 2d19884aaa76c88ac72c5185712d19b1ce509e76 <br/> **SHA256**: f471c57822036025a1f3b88a67e8b6a117580451edde92039bd3f697e079bd5a |

Thank you for choosing EpicChain-GUI. We look forward to your feedback and contributions to enhance this tool further!
