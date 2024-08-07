---
title: Get Block Header
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




The `getblockheader` method retrieves the header information of a specific block in the blockchain using either the block's hash or its index (also known as the block height).

## Prerequisites

To utilize the `getblockheader` method, the `RpcServer` plugin must be installed on your EpicChain node. This setup enables your node to process JSON-RPC requests.

## Parameters

- **hash | index**: Accepts either the block hash (as a string) or the block index (as an integer, where block height = number of blocks - 1).
- **verbose** (Optional): A boolean flag that determines the format of the returned information.
  - `false` (or `0`): Returns serialized information of the block header as a hexadecimal string. Deserialization is needed to view the detailed information.
  - `true` (or `1`): Returns detailed information of the block header in JSON format. This is the default if not specified.

## Example Usage

### Example 1 - Retrieve Serialized Block Header Information

#### Request Body:

For block index:
```json
{
  "jsonrpc": "2.0",
  "method": "getblockheader",
  "params": [140],
  "id": 1
}
```
Or for block hash:
```json
{
  "jsonrpc": "2.0",
  "method": "getblockheader",
  "params": ["0x3d87f53c51c93fc08e5ccc09dbd9e21fcfad4dbea66af454bed334824a90262c"],
  "id": 1
}
```

#### Response Body:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "Serialized Hexadecimal Block Header Data"
}
```

### Example 2 - Retrieve Block Header Information in JSON Format

#### Request Body:

For block index:
```json
{
  "jsonrpc": "2.0",
  "method": "getblockheader",
  "params": [140, true],
  "id": 1
}
```
Or for block hash:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getblockheader",
  "params": ["0x3d87f53c51c93fc08e5ccc09dbd9e21fcfad4dbea66af454bed334824a90262c", true]
}
```

#### Response Body:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "hash": "Block Hash",
    "size": 213,
    "version": 0,
    "previousblockhash": "Previous Block Hash",
    "merkleroot": "Merkle Root of the Block",
    "time": 1612687482881,
    "index": 140,
    "primary": 0,
    "nextconsensus": "Next Consensus Address",
    "witnesses": [
      {
        "invocation": "Invocation Script Data",
        "verification": "Verification Script Data"
      }
    ],
    "confirmations": 41,
    "nextblockhash": "Hash of the Next Block"
  }
}
```

## Response Description

- **hash**: The unique identifier of the block.
- **size**: The size of the block in bytes.
- **version**: The version number of the block.
- **previousblockhash**: The hash of the previous block in the chain.
- **merkleroot**: The root hash of the Merkle tree of the block's transactions.
- **time**: The timestamp when the block was mined.
- **index**: The height of the block in the blockchain.
- **primary**: The primary node index that proposed the block.
- **nextconsensus**: The address of the next consensus node.
- **witnesses**: Contains the scripts used in the block's verification process.
- **confirmations**: The number of confirmations the block has received.

The `getblockheader` method provides essential information for understanding and verifying the structural integrity of individual blocks within the blockchain, serving both developers and participants in the blockchain ecosystem.

<br/>