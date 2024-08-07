---
title: Block
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';



# **EpicChain Block Structure Deep Dive**

**EpicChain**, a fortress of digital transactions, stands tall on the pillars of blockchain technology. At its very core, the **block structure** acts as the bedrock upon which the integrity and reliability of the network are built. Let's unravel the intricacies of its design, focusing on the block's composition, the block header's anatomy, and the vibrant tapestry of transactions that form the block body.

---

## **Block Composition**

The **blockchain** is a meticulously woven tapestry of data, where each **block** is a patchwork of two primary components: the **block header** and the **block body**. Anchored by a cryptographic hash of its predecessor (`PrevHash`), it crafts a chain of unbreakable bonds, threading together the fabric of transactions through time.

### **Block Header**

Functioning as the nucleus of a block, the block header encapsulates the essence of blockchain data, offering both verification and essential information about the block. It is delineated as follows:

| Size | Field          | Name                     | Type        | Description                                                                                     |
|------|----------------|--------------------------|-------------|-------------------------------------------------------------------------------------------------|
| 4    | Version        | Block Version            | uint        | Marks the block version, currently at `0`.                                                      |
| 32   | PrevHash       | Previous Hash            | UInt256     | Cryptographic hash of the preceding block.                                                      |
| 32   | MerkleRoot     | Merkle Tree Root         | Uint256     | Root hash of the block's transactions, calculated via the Merkle tree.                          |
| 8    | Timestamp      | Block Timestamp          | ulong       | Timestamp marking the block's genesis.                                                          |
| 8    | Nonce          | Random number            | ulong       | A nonce fostering the randomness of the block.                                                  |
| 4    | Index          | Block Index              | uint        | Denotes block height with the Genesis Block at `0`.                                             |
| 1    | PrimaryIndex   | Speaker index            | byte        | Index of the proposing validator in the current consensus round.                                |
| 20   | NextConsensus  | Next round validator     | UInt160     | The script hash representing two-thirds of validator's signatures for the subsequent round.     |
| ?    | Witness        | Witness                  | Witness     | Comprises executable verification scripts ensuring the block's authenticity.                    |

**Identification**: A block can be pinpointed using its hash and index, with the hash emerging from a SHA256 operation performed twice on the concatenation of the block header's first seven attributes.

**Consensus and Timestamp**: Each block's height is unique, established by the consensus of over two-thirds of the nodes and incremented by `1` from the previous block. The timestamp must exceed that of the preceding block, aligning with a preset block generation interval.

**NextConsensus**: This field solidifies the nodes partaking in the ensuing round of consensus through a multi-signature contract, necessitating over two-thirds of the consensus nodes' signatures.

**Witness**: Constituting the block's verification scripts, this section ensures the block's verification through `InvocationScript` and `VerificationScript`.

---

## **Block Body**

Diving deeper into the structure, the block body is the repository of transactions — the heartbeat of the blockchain. Detailed structure below:

| Size | Field          | Name             | Type            | Description                                         |
|------|----------------|------------------|-----------------|-----------------------------------------------------|
| ?    | Header         | block header     | Header          | An outline at the commencement of the block body.    |
| ?*?  | Transactions   | Transaction List | Transaction[]   | The payload, encapsulating a suite of transactions. |

**Transaction Crafting**: Within a consensus round, the **Speaker** garners a series of vetted transactions from the memory pool, embedding their hashes into a consensus message (`PrePareRequest`) for network-wide broadcast.

**Constraints**: Presently, each block may envelop up to 512 transactions, optimizing both performance and integrity.

**Persistence**: Upon block finalization, a hash list of the transactions is stored, while the transaction data itself is archived separately to streamline queries.

---

EpicChain, through its innovative block structure, champions the principles of decentralization, security, and efficiency. It stands as a testament to the power of blockchain technology, weaving a secure ledger of digital existence.





<br/>