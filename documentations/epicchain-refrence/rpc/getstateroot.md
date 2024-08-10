---
title: Get State Root
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';





The `getstateroot` method enables querying the state root associated with a specific block height in the blockchain. The state root is a cryptographic hash that represents the entire state of the blockchain at a particular block, ensuring the integrity and consistency of the blockchain's state at that point in time.

## Prerequisites

Before using `getstateroot`, ensure the following plugins are installed on your blockchain node:

1. **StateService Plugin**: For handling state-related queries, including fetching the state root for a given block height.
2. **RpcServer Plugin**: This plugin activates the JSON-RPC interface, allowing your node to process requests such as `getstateroot`.

## Parameters

- **index**: The block height (index) for which you want to query the state root. The blockchain height refers to the number of blocks preceding a particular block, including the genesis block.

## Example Usage

### Request Body

To retrieve the state root for block 160, structure your JSON-RPC request as follows:

```json
{
  "jsonrpc": "2.0",
  "method": "getstateroot",
  "params": [160],
  "id": 1
}
```

This request specifies:
- The JSON-RPC version (`2.0`),
- The method name (`getstateroot`),
- Parameters for the method (`[160]` represents the block index),
- A unique identifier for the request (`id = 1`).

### Response Body

The response to a successful query includes:

- **version**: The version of the state root. It could indicate the format or structure used to compute the state root.
- **index**: The block height for which the state root was requested, confirming the target of the query.
- **roothash**: The actual state root hash for the requested block. This hash is a cryptographic digest representing the entire state of the blockchain at block 160.
- **witnesses**: Witnesses are usually signatures proving the authenticity of the state root. This field might be empty depending on the blockchain's design or if the information is not required for the query.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "version": 0,
    "index": 160,
    "roothash": "0x3d3f099e05cf92c018703ab309d8643c30a0ab6b2b008cc6fe80869b1a350c31",
    "witnesses": []
  }
}
```

## Considerations

- The state root provides a snapshot of the blockchain’s entire state at a given block height. It's a crucial tool for verifying the integrity of the blockchain’s state and for auditing purposes.

- The `witnesses` array might be critical in environments where additional verification is required to prove the state root's authenticity.

- When querying the state root, ensure that your node is fully synchronized up to the block of interest to avoid querying non-existent or incomplete data.

Using the `getstateroot` method is essential for developers, auditors, and users looking to verify blockchain transactions and states up to a specific block, enhancing transparency and trust in blockchain ecosystems.











<br/>