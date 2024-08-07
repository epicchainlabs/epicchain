---
title: Multi Core
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







# Building a Private Chain on Localhost with EpicChain

Embarking on blockchain development often begins in a controlled, isolated environment. Setting up a private chain on a localhost serves as a foundational step, especially for developers working on Windows systems. This guide walks you through the simple yet comprehensive process of establishing your own EpicChain private chain.

## **1. Installing EpicChain Node**

Start by installing **EpicChain-CLI**. Once installed, replicate the node folder four times, naming them `c1`, `c2`, `c3`, and `c4`, respectively.

For detailed steps on installing EpicChain-CLI, please refer to the [Installation Guide](https://EpicChain.org/cli-installation).

## **2. Creating Wallet Files**

Within **EpicChain-CLI**, generate four wallets named `1.json`, `2.json`, `3.json`, and `4.json`. Each wallet should be placed in its corresponding node folder.

## **3. Modifying config.json**

In every node folder, there’s a `config.json` file that needs specific adjustments:

- Ensure each node has unique and unoccupied port numbers.
- "UnlockWallet" should specify the wallet path and password for each node.
- Set both `StartConsensus` and `IsActive` to `true`.
- Define a **private chain ID** with any integer ranged between [0 - 4294967295].
- `StandbyCommittee` should include the public keys of all four wallets.
- For `SeedList`, use `localhost` as the IP address and specify the unique ports.

Example modifications for `c1/config.json` are shown below, and similar adjustments should be applied to other nodes, **adjusting ports and wallet paths accordingly**.

```json
{
  "ApplicationConfiguration": {
    ...,
    "P2P": {
      "Port": 21333,
      ...,
    "UnlockWallet": {
      "Path": "1.json",
      "Password": "1",
      ...
    }
  },
  "ProtocolConfiguration": {
    "Network": 5943216,
    ...,
    "StandbyCommittee": [
      "Public key 1",
      "Public key 2",
      ...
    ],
    "SeedList": [
      "localhost:21333",
      ...
    ]
  }
}
```

## **Installing Consensus Plugin**

For enabling consensus among nodes, download the **DBFTPlugin** and place it in the `Plugins` folder of each node’s CLI directory. Ensure the plugin `config.json` mirrors the network ID set in the node’s `config.json` and has `AutoStart` set to `true`.

## **4. Starting the Private Chain**

To simplify the launch process, create a `Run.cmd` file that contains commands to start each node. The format looks like this:

```bash
start cmd /k "cd c1 &&ping localhost -n 3 > nul&& dotnet EpicChain-cli.dll"
...
start cmd /k "cd c4 &&ping localhost -n 3 > nul&& dotnet EpicChain-cli.dll"
```

Execute `Run.cmd` within each node directory. Successful initiation is indicated by consensus information and an incrementing block height.

## **5. Withdrawing EpicChain and EpicPulse**

The genesis block generates a substantial amount of EpicChain and EpicPulse. Utilize **EpicChain-CLI** for withdrawing these assets for development and testing purposes.

### **Creating Multi-party Signature Address**

First, ensure at least three wallets have created the multi-signature address as outlined in `import multisigaddress` commands. This process involves the public keys from the `StandbyCommittee` list.

### **Transferring Assets**

To transfer EpicChain or EpicPulse from the multi-signature address to a standard address, use the `send` command in EpicChain-CLI. This process will require signing the transaction with multiple wallets involved in the multi-signature setup.

```bash
send <id|alias> <address> <value>
```

This concise guide provides a springboard into private chain development on EpicChain, ensuring a controlled, robust environment for blockchain innovation and testing.



















<br/>