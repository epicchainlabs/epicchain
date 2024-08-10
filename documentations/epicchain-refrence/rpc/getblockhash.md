---
title: Get Block Hash
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




The `getblockhash` method is employed to obtain the hash of a specific block on the blockchain based on its index (also referred to as block height). This method is fundamental for querying and interacting with the blockchain, allowing for direct access to block data through its unique identifier.

## Prerequisites

Before you're able to make use of the `getblockhash` method, ensure the `RpcServer` plugin is actively installed on your EpicChain node. This installation is essential for your node to process and respond to JSON-RPC requests.

## Parameters

- **index**: The index of the block for which you desire to retrieve the hash. The block index refers to the block's height within the blockchain, starting from 0 for the genesis block.

## Example Usage

### Request Body

To request the hash for a block at a specific index, your JSON-RPC request should look like the following:

```json
{
  "jsonrpc": "2.0",
  "method": "getblockhash",
  "params": [10000],
  "id": 1
}
```

In this request:
- The `method` field is set to `"getblockhash"`, indicating the desired operation.
- The `params` array contains a single element, the index of the block whose hash you're querying (in this case, `10000`).

### Response Body

Assuming the block at the specified index (`10000`) has a hash of `"0xdf17b40c5627a45e83d61b286a6d6d14859136621760d0a5b58dd59d18fd53d4"`, the response would be structured as:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "0xdf17b40c5627a45e83d61b286a6d6d14859136621760d0a5b58dd59d18fd53d4"
}
```

## Response Description

- **jsonrpc**: Indicates the version of the JSON-RPC protocol being used.
- **id**: The unique identifier that matches the request to the response, facilitating asynchronous processing.
- **result**: Contains the hash of the block at the specified index. This hash serves as a unique identifier for the block within the blockchain.

## Key Insights

- The `getblockhash` method is invaluable for developers and applications requiring access to specific blocks within the blockchain. 
- Knowing the block hash, further detailed information about the block can be obtained using methods like `getblock`.
- This functionality underscores the blockchain's transparent and accessible nature, where information about any block is readily available to participants.
  
Ensure that your EpicChain node's `RpcServer` plugin is correctly configured and operational to utilize the `getblockhash` method and other JSON-RPC functionalities effectively.


<br/>