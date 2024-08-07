---
title: Deploying Smart Contracts
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




# Deploying Smart Contracts on EpicChain

## Overview

Smart contracts need deployment when they operate as storage mediums or are intended to be invoked by other contracts on the EpicChain blockchain. This guide details the deployment and invocation process for smart contracts on EpicChain using EpicChain-CLI.

### When to Deploy Contracts?

- Contracts storing data or being invoked by `System.Contract.Call`.
- Contracts implementing the EEP-17 standard.
- Not needed for contracts triggered only by the verification trigger without blockchain data interaction.

## Preparation

Before deployment, ensure the following:

- The contract should indeed be deployed.
- Have your contract file (*.nef) and its manifest (*.manifest.json) ready.
- Install EpicChain-CLI or EpicChain-GUI and synchronize with the blockchain.

## Deploying with EpicChain-CLI

To deploy your smart contract through the CLI, follow these steps:

1. **Open EpicChain-CLI**: Navigate to your CLI utility.

2. **Deploy Command**: Use the deploy command as shown below.

   ```shell
   deploy <nefFilePath> [manifestFile]
   

   - `<nefFilePath>`: The path to your contract NEF file.
   - `[manifestFile]`: Optional path to your contract Manifest JSON file.

3. **Example Command**:

   ```shell
   deploy EEP17.nef
   ```

   Or with the manifest:

   ```shell
   deploy EEP17.nef EEP17.manifest.json
   ```

4. **Follow the CLI Prompts**: The CLI will guide you through deploying the contract, including fee payment.

   Example output:

   ```shell
   Contract hash: 0xb7f4d011241ec13db16c0e3484bdd5dd9a536f26
   EpicPulse consumed: 10.0107577
   Network fee: 0.0345352
   Total fee: 10.0452929 EpicPulse
   Relay tx? (no|yes): yes
   ```

## Understanding Contract ScriptHash

After deployment, a unique **ScriptHash** identifies your contract. Convert the **ScriptHash** to a standard address format (big endian) to receive assets:

- ScriptHash (big endian): `0xb7f4d011241ec13db16c0e3484bdd5dd9a536f26`
- Corresponding address: `NPRCE9me1CdXBA6StQ7kff52p61rHQqnS7`

## Common Errors

Be aware of common errors:

- **Engine faulted**: Execution error within the contract.
- **Contract already exists**: A contract with the same hash is already deployed.
- **.nef is not matched with .manifest**: File mismatch or corruption.
- **Insufficient fee**: Insufficient EpicPulse to cover the deployment fee.

## Deploying with EpicChain-GUI

Alternatively, for a GUI approach:

1. **Open EpicChain-GUI**: Navigate to the Contract -> Deploy Contract section.

2. **Select Files**: Choose your `*.nef` and `*.manifest.json` files.

3. **Deploy Contract**: Click Deploy and follow the on-screen instructions.

Adhering to this guide will streamline the process of deploying your smart contracts on the EpicChain blockchain, setting the foundation for your blockchain-based applications and assets.





















<br/>