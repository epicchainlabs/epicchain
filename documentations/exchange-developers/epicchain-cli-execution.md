---
title: EpicChain CLI Execution
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';



# EpicChain-CLI Usage Guide

## Introduction

The EpicChain-CLI client serves as both a node in the P2P network and a cross-platform wallet for handling various asset transactions. This guide provides detailed information on how to use the EpicChain-CLI client, including security policies, network port configurations, and wallet management.

## EpicChain-CLI Security Policies

- **White List or Firewall:** It is essential for exchanges to use a white list or firewall to block external server requests. Failure to do so can result in significant security risks.
- **Wallet Security:** EpicChain-CLI does not provide remote switching on/off of the wallet, nor does it verify the process when opening a wallet. Exchanges should set their security policies accordingly. The wallet must be kept open at all times to respond to user withdrawal requests.
- **Firewall Configuration:** For security reasons, wallets should be run on independent servers with properly configured firewalls.

### Network Port Configurations

| Network    | Mainnet | Testnet |
|------------|---------|---------|
| JSON-RPC via HTTPS | 10111   | 20111   |
| JSON-RPC via HTTP  | 10112   | 20112   |
| P2P                | 10113   | 20113   |
| WebSocket          | 10114   | 20114   |

## About EpicChain-CLI

EpicChain-CLI is a command-line client (wallet) for developers. It provides two primary ways for developers to interact with it:

1. **Using the CLI commands:** You can perform various operations such as creating a wallet, generating an address, etc.
2. **Using Remote Procedure Call (RPC):** This allows you to perform actions like transferring to a designated address, acquiring block information, and more.

EpicChain-CLI offers the following features:

- **Wallet Management:** Manage assets through the command-line interface.
- **Starting the Wallet:** To enable the wallet, navigate to the EpicChain-CLI directory and run the following command:
  ```
  dotnet epicchain-cli.dll
  ```
- **Checking Available Commands:** To see all available commands, use the following command:
  ```
  help
  ```

## Creating a Wallet

Exchanges need to create an online wallet to manage user deposit addresses. A wallet stores information about accounts (public and private keys) and contracts. It is crucial for users to keep their wallet files and passwords secure.

### Supported Wallet Formats

EpicChain-CLI supports wallets in two formats: sqlite wallet (.db3) and NEP6 standard wallet (.json). For exchanges, the sqlite wallet is recommended.

### Steps to Create a Wallet

1. Enter the following command:
   ```
   create wallet <path>
   ```
   `<path>` is the wallet path and filename. The file extension can be .db3 or .json, depending on the wallet type. For example:
   ```
   create wallet /home/mywallet.db3
   ```
2. Set a password for the wallet.

## Generating Deposit Addresses

A wallet can store multiple addresses, and the exchange needs to generate a deposit address for each user.

### Methods for Generating Deposit Addresses

1. **Dynamic Generation:** When a user deposits for the first time, the program dynamically generates an address. Use the `getnewaddress` method through the RpcServer API to achieve this.
2. **Batch Generation:** The exchange can create a batch of addresses in advance and assign them to users when they deposit for the first time. To generate addresses in batch, run the following command:
   ```
   create address [n]
   ```
   `[n]` is optional and specifies the number of addresses to generate (default is 1). For example, to generate 100 addresses, enter:
   ```
   create address 100
   ```

Using the batch generation method is recommended for better wallet management and stability.










<br/>