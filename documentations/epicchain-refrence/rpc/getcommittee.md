---
title: Get Committee
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




The `getcommittee` method is designed to retrieve the list of public keys representing the current committee members in the Neo blockchain or similar platforms.

## Prerequisites

To utilize the `getcommittee` method, it's necessary to have the `RpcServer` plugin installed on your node. This plugin enables your node to handle JSON-RPC requests, allowing for interaction with various blockchain data and features.

## Example Usage

### Request Body

Below is an example of how to construct a JSON-RPC request to get the list of current committee members' public keys:

```json
{
  "jsonrpc": "2.0",
  "method": "getcommittee",
  "params": [],
  "id": 1
}
```

In this request:
- `method`: The name of the method being invoked, in this case, `getcommittee`.
- `params`: An empty array indicating that no parameters are required for this method.
- `id`: A unique identifier for the request.

### Response Body

The response to the above request provides an array of strings, each representing the public key of a committee member:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    "Public Key 1",
    "Public Key 2",
    ...
    "Public Key N"
  ]
}
```

A sample of what the actual response might look like, shortened for brevity:

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": [
        "020f2887f41474cfeb11fd262e982051c1541418137c02a0f4961af911045de639",
        "03204223f8c86b8cd5c89ef12e4f0dbb314172e9241e30c9ef2293790793537cf0",
        "0222038884bbd1d8ff109ed3bdef3542e768eef76c1247aea8bc8171f532928c30",
        ...
        "03d281b42002647f0113f36c7b8efb30db66078dfaaa9ab3ff76d043a98d512fde",
        "02df48f60e8f3e01c48ff40b9b7f1310d7a8b2a193188befe1c2e3df740e895093"
    ]
}
```

## Key Insights

- The `getcommittee` method provides transparency by making the list of current committee members publicly accessible. This is a crucial aspect of decentralized governance systems.
- The public keys of the committee members play a significant role in the blockchain's consensus mechanism and governance, as they may be responsible for validating transactions and blocks or making critical decisions about the network.
- Being able to retrieve and verify the list of committee member public keys helps maintain the integrity and trust in the blockchain system, ensuring that only authorized members are part of the decision-making process.

This method is integral to maintaining transparency and trust in blockchain governance, providing easy access to the identities of those in significant governance roles.

<br/>