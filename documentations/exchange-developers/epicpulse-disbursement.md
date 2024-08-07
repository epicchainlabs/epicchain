---
title: EpicPulse Disbursement
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Guide to Distributing and Claiming EpicPulse on EpicChain

EpicPulse plays a pivotal role in the EpicChain ecosystem, as it represents the operational token required for transactions, smart contracts execution, and access to various services on the blockchain. Here's an in-depth look into managing and distributing EpicPulse to users, especially for exchanges.

## Understanding EpicPulse

**EpicPulse** is essentially the fuel of the EpicChain blockchain, necessary for transaction fees, deploying smart contracts, and other network services. It is automatically generated in the system when EpicChain (the native asset) is transferred, based on specific algorithms.

## Generating EpicPulse

In EpicChain N3, EpicPulse is generated with each EpicChain transaction that occurs on an address. The amount of EpicPulse claimed after a transaction is determined by a function of the EpicChain amount and the time elapsed since the last EpicChain transaction.

### Calculating Claimed EpicPulse

The formula for calculating claimed EpicPulse is:

```
ClaimedEpicPulse = f(EpicChainAmount, Δtconst)
```

Where:

- **Δtconst** = `t_end - t_start`
  - `t_end` is the current timestamp of the transaction.
  - `t_start` is the timestamp of the last EpicChain transaction on the address.

This calculation ensures that the amount of EpicPulse claimed is directly proportional to both the quantity of EpicChain transacted and the duration it was held.

## Distributing EpicPulse to Users

For exchanges hosting multiple user addresses in a single wallet, accurate distribution of EpicPulse to users demands precise computation. This involves capturing snapshots of user balances at regular intervals and employing a weighted average calculation for non-uniform intervals.

### Snapshot Approach

Implementing a shorter interval for snapshots increases the accuracy of EpicPulse distribution. However, if intervals vary, a weighted average method is recommended to maintain precision.

### Fixed Income on EpicPulse

In the EpicChain N3 network, exchange users receive a fixed income of 10% of the total EpicPulse they're eligible to claim, aligning with the EpicPulse distribution rules.

## RPC Methods for Querying EpicPulse

Exchanges can leverage specific RPC methods to retrieve information about users' unclaimed EpicPulse amounts:

- **getunclaimedepicpulse**: Returns the amount of EpicPulse yet to be claimed by the wallet's addresses.

## Claiming EpicPulse

EpicPulse is claimed automatically when EpicChain is transferred into or out of an address. For users with unclaimed EpicPulse, initiating a self-transfer (sending EpicChain to their own address) triggers the claiming process.

### Steps for Claiming EpicPulse via EpicChain-CLI

1. **Initialization**: Start the EpicChain CLI.
   
   ```bash
   dotnet epicchain-cli.dll
   ```
   
2. **Verify Client Version**: Ensure you're running the compatible version.
   
   ```bash
   version
   ```
   
3. **Check Synchronization**: Confirm the client is fully synchronized with the blockchain.
   
   ```bash
   show state
   ```
   
4. **Wallet Management**: Open the wallet involved in the transaction.
   
   ```bash
   open wallet /path/to/wallet.json
   ```
   
5. **Inspect Wallet**: List the assets and addresses in the wallet.
   
   ```bash
   list asset
   ```
   
6. **Review EpicPulse Balance**: Check the current EpicPulse balance.
   
   ```bash
   show epicpulse
   ```
   
7. **Trigger EpicPulse Claiming**: Transfer EpicChain to your own address.
   
   ```bash
   send epicchain <YourAddress> 1
   ```
   
8. **Confirm Asset Balance**: Verify the balance post-transaction to ensure EpicPulse was claimed.
   
   ```bash
   list asset
   ```

By executing these steps, users can efficiently claim their EpicPulse, ensuring they have adequate resources for transactions and operations on the EpicChain network. Exchanges can streamline this process for their users, enhancing the overall ecosystem's functionality and user experience.






<br/>