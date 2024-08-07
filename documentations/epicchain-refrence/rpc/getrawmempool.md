---
title: Get Raw mempool
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';






The `getrawmempool` JSON-RPC method fetches the list of transactions currently in the memory pool (mempool) of a blockchain node. The mempool consists of transactions that have been broadcast to the network but are not yet included in a block — essentially, transactions waiting to be confirmed. 

## Prerequisites

Before you can use the `getrawmempool` method, ensure the `RpcServer` plugin is installed on your blockchain node. This plugin enables JSON-RPC communication, allowing external services to request data from the node.

## Example Usage

### Example 1: Retrieve Verified Transactions

To fetch a list of transactions that have been verified but not yet confirmed in a block:

#### Request Body

```json
{
  "jsonrpc": "2.0",
  "method": "getrawmempool",
  "params": [],
  "id": 1
}
```

#### Response Body

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    "0x9786cce0dddb524c40ddbdd5e31a41ed1f6b5c8a683c122f627ca4a007a7cf4e",
    "0xb488ad25eb474f89d5ca3f985cc047ca96bc7373a6d3da8c0f192722896c1cd7",
    "0xf86f6f2c08fbf766ebe59dc84bc3b8829f1053f0a01deb26bf7960d99fa86cd6"
  ]
}
```

The `result` contains an array of transaction hashes representing the verified transactions in the mempool. 

### Example 2: Get Both Verified and Unverified Transactions

To obtain a detailed view including both verified and unverified transactions:

#### Request Body

```json
{
  "jsonrpc": "2.0",
  "method": "getrawmempool",
  "params": [true],
  "id": 1
}
```

By passing `true` as a parameter, this request asks for a categorized list of transactions.

#### Response Body

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "height": 5882071,
    "verified": [
      "0x0c65fbfd2598aee5f30cd18f1264b458f1db137c4a460f4a174facb3f2d59d06",
      "0xc8040c285aa495f5b5e5b3761fd9333899f4ed902951c46d86c3bbb1cb12f2c0"
    ],
    "unverified": []
  }
}
```

The response includes:

- **height**: The current block height.
- **verified**: Transactions that have been verified and are waiting for inclusion in a block.
- **unverified**: Transactions that are in the mempool but haven't been verified yet. In this example, the array is empty, indicating all mempool transactions at the moment of the query were verified.

## Considerations

- The mempool state can quickly change as new transactions arrive and others are confirmed in blocks. Therefore, the data fetched reflects a snapshot of the mempool at the time of the query.
- Understanding the mempool's state is crucial for estimating transaction confirmation times and for network analysis purposes.
- Nodes may have policy or resource-based constraints affecting the mempool's composition, so reported transactions might vary between nodes.

The `getrawmempool` method provides valuable insights into pending transactions and network dynamics, aiding developers, analysts, and participants in navigating and understanding blockchain operations.










<br/>