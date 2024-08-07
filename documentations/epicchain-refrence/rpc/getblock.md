---
title: Get block
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




The `getblock` method is essential for querying detailed information about a specific block within the blockchain, based on either its hash or index (block height).

## Prerequisites

Before executing the `getblock` method, ensure the `RpcServer` plugin is installed on your EpicChain node. This setup is required for the node to process JSON-RPC requests.

## Parameters

- **hash | index**: This can either be the hash of the block in string form or the block's index as an integer (where block height = block index + 1).
- **verbose** (Optional): Determines the format of the returned data. A boolean where:
  - `false` (or `0`): Returns a Base64-encoded string of the serialized block.
  - `true` (or `1`): Returns detailed information of the block in JSON format. This is the default behavior if the verbose parameter is not specified.

## Example Requests

### Example 1 - Serialized Block Information

#### Request Body:

```json
{
  "jsonrpc": "2.0",
  "method": "getblock",
  "params": [26536],
  "id": 1
}
```
or
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getblock",
  "params": ["0xd373a9afdbe57d79ad788196aa4ef37dbfb28c7d8f22ffa1ccbc236d56268bca"]
}
```
#### Response Body:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "[Base64-encoded block data]"
}
```
### Example 2 - Detailed Block Information in JSON

#### Request Body:

```json
{
  "jsonrpc": "2.0",
  "method": "getblock",
  "params": [26536, true],
  "id": 1
}
```
or
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getblock",
  "params": ["0xd373a9afdbe57d79ad788196aa4ef37dbfb28c7d8f22ffa1ccbc236d56268bca", true]
}
```
#### Response Body:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "hash": "Block Hash",
    "size": 5317,
    "version": 0,
    "previousblockhash": "Previous Block Hash",
    "merkleroot": "Merkle Root",
    "time": 1615348136186,
    "index": 26536,
    "primary": 6,
    "nextconsensus": "Next Consensus Node",
    "witnesses": [...],
    "tx": [...],
    "confirmations": 50,
    "nextblockhash": "Next Block Hash"
  }
}
```
The detailed response includes information such as the block's hash, the size, the version, the hash of the previous block, the block's Merkle root, the time of block creation, block index (height), list of transactions within the block, and more.

## Response Description

- **hash**: The unique identifier of the block.
- **size**: The size of the block in bytes.
- **version**: The version number of the block.
- **previousblockhash**: The hash of the previous block in the blockchain.
- **merkleroot**: The Merkle root of the block's transactions.
- **time**: The timestamp for when the block was mined.
- **index** (block height): The index or height of the block within the blockchain.
- **primary**: The index of the primary validator that proposed the block.
- **nextconsensus**: The consensus node expected to propose the next block.
- **witnesses**: An array containing the witness data for the block.
- **tx**: An array of transactions included in the block.
- **confirmations**: The number of confirmations the block has received.
- **nextblockhash**: The hash of the subsequent block in the blockchain.

Using the `getblock` method, users and applications can access both high-level and granular details about blocks on the EpicChain blockchain, supporting a wide range of blockchain analysis and monitoring tasks.


<br/>