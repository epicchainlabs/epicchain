---
title: Get Block Count
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




The `getblockcount` method provides a straightforward way to retrieve the total number of blocks currently in the blockchain. This count reflects the blockchain's length and can be crucial for various applications, including synchronization and monitoring blockchain growth.

## Prerequisites

Before utilizing the `getblockcount` method, make sure the `RpcServer` plugin is installed on your EpicChain node. This plugin facilitates handling JSON-RPC requests, enabling external communication with the blockchain node.

## Example Usage

### Request Body

To query the blockchain for its current block count, construct your JSON-RPC request as follows:

```json
{
  "jsonrpc": "2.0",
  "method": "getblockcount",
  "params": [],
  "id": 1
}
```
This request specifies the method `getblockcount` without any parameters, as the method does not require any.

### Response Body

Assuming the blockchain currently comprises 991,991 blocks, the response will be structured as follows:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": 991991
}
```
In the response:
- **jsonrpc** specifies the JSON-RPC version.
- **id** is the identifier matching the request and response.
- **result** contains the total number of blocks in the blockchain, which, in this example, is 991,991.

## Key Insights

- The `getblockcount` method is vital for understanding the size and progression of the blockchain, providing a numeric representation of the total blocks contained within.
- This method can aid in syncing operations, as it offers a clear target for applications or services looking to download or verify the blockchain up to its most recent block.
- Additionally, monitoring the `getblockcount` output over time can offer insights into the blockchain's growth rate, reflecting the activity level and adoption of the network.

Remember that the block count is continuously increasing as new blocks are mined or produced, so repeated queries are expected to yield gradually increasing results in a healthy and active blockchain ecosystem.


<br/>