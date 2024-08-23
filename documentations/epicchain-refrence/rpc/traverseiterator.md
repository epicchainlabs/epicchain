---
title: Traverse Iterator
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








The `traverseiterator` method is utilized for querying Iterator type data within the EpicChain network, specifically for retrieving values iteratively from smart contract invocations. This method is essential for handling complex data structures returned by smart contracts without altering the blockchain state.

## Prerequisites

- **RpcServer Plugin**: Must be installed on your blockchain node. This plugin enables the node to process JSON-RPC requests, including Iterator queries.

- **Session and Iterator ID**: Obtain these by first invoking either `invokefunction` or `invokescript` methods. These IDs are essential for identifying the specific Iterator session for traversal.

- **SessionEnabled Configuration**: Ensure the `SessionEnabled` setting in `config.json` of the RpcServer plugin is set to `true`, allowing for session-based operations.

## Parameters Description

- **session**: A unique identifier for the session, returned previously by either `invokefunction` or `invokescript`.

- **iterator id**: Identifies the specific Iterator to be traversed, derived from a stack returned by `invokefunction` or `invokescript`.

- **count**: Specifies the number of values to return in the traversal. This number cannot exceed the `MaxIteratorResultItems` limit as defined in the RpcServer plugin’s `config.json`.

## Example Usage

### Request Body

To traverse an Iterator, structure your JSON-RPC request as below:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "traverseiterator",
  "params": [
    "c5b628b6-10d9-4cc5-b850-3cfc0b659fcf",
    "593b02c6-138d-4945-846d-1e5974091daa",
    10
  ]
}
```

### Response Body

A successful traversal will return a subset of the data contained within the Iterator:

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": [
        {
            "type": "ByteString",
            "value": "Encoded_value_1"
        },
        ...,
        {
            "type": "ByteString",
            "value": "Encoded_value_n"
        }
    ]
}
```

### Response Description

- **Result**: An array of the first `count` items in the Iterator. Subsequent requests continue from where the last traversal ended, effectively allowing complete iteration through the Iterator dataset.

## Considerations

- **Session Expiration**: The validity of both the session and Iterator ID is contingent on the `SessionExpirationTime` setting within the RpcServer plugin’s `config.json`. Monitor session validity to ensure successful traversal.

- **Data Handling**: The returned data is encoded; thus, decoding may be required depending on your use case.

- **Resource Management**: Be mindful of the `MaxIteratorResultItems` setting to avoid overloading response bodies and ensuring efficient data retrieval.

The `traverseiterator` method enhances data accessibility for applications and developers by enabling efficient iteration through complex data sets returned by smart contracts within the EpicChain network.









<br/>