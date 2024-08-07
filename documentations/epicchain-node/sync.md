---
title: Synchronizing
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




To optimize the synchronization process and ensure your EpicChain-CLI client is fully up-to-date with the blockchain, utilizing an offline package is a highly efficient method. This process involves downloading a snapshot of the blockchain data up to a specific block height, which can significantly reduce the time your client spends synchronizing with the network. Here’s a detailed walkthrough of the process, including terminology explanations for a deeper understanding.

### Understanding Blockchain Synchronization

**Blockchain synchronization** is a process where a client (a software that connects to the blockchain network) downloads and verifies all past transactions on the blockchain. This is essential for the client to have a complete and accurate view of the blockchain state, enabling it to validate new transactions and blocks accurately. The traditional synchronization process can be time-consuming, especially for blockchains with a substantial history of transactions and blocks.

### Step 1: Download the Offline Package

Before initiating this step, ensure the EpicChain-CLI is completely shut down to prevent any conflicts.

#### Navigating the Blockchain Landscape

- Visit the **offline package downloading page** on the official EpicChain website. This page hosts snapshots of the blockchain data, typically zipped, for easier download and usage.
  
#### Choosing the Right Package

- **Full Offline Package (`chain.0.acc.zip`)**: Contains the entire blockchain data up to a recent but not the latest block. It’s designed for new clients or instances where the blockchain data needs to be rebuilt from scratch.
  
- **Increment Offline Package (`chain.xxx.acc.zip`)**: Contains blockchain data from a specific starting block height (`xxx`) to an ending block height. It's suitable for clients that have been offline for a short period and need to catch up with the blockchain network without re-syncing the entire chain.

**Compatibility Note**: Maintain the original filename of the downloaded package to ensure seamless integration with the EpicChain-CLI during the synchronization process.

### Step 2: Placement of the Offline Package

Upon downloading the appropriate offline package, you must place it in the root directory of your EpicChain-CLI installation. This placement is critical for the EpicChain-CLI to recognize and utilize the offline data during the synchronization process.

### Step 3: Verification of Synchronization Status

After positioning the offline package accordingly:

1. **Restart** the EpicChain-CLI client to kickstart the synchronization process utilizing the offline package.
2. **Open** your wallet with `open wallet <path>` command to ensure your client is operational.
3. **Assess** the progress by issuing the `show state` command. This displays the synchronization status, including the number of blocks processed and the number of connected nodes.

**Indicator of Offline Synchronization**: A significant acceleration in block processing without any active node connections (`connected nodes = 0`) signals that the offline package is correctly deployed. Upon completion of the offline package processing, the client will automatically seek and sync the remaining blocks from the network, as indicated by the re-appearance of connected nodes.

### Terminology and Final Thoughts

- **Block Height**: The number of blocks in the chain leading up to (and including) the given block. It serves as a straightforward way to reference specific points in the blockchain history.
- **Blockchain State**: Represents the cumulative state of all transactions and consequent data up to the current block, including account balances, smart contract states, and more.
  
Utilizing an offline package for blockchain synchronization is a strategic approach to streamline the initial setup or catch-up process for the EpicChain-CLI client. It not only saves time but also reduces the burden on network resources, ensuring a smoother entry or re-entry into the EpicChain ecosystem for users.








<br/>