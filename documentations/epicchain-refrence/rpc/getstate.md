---
title: Get State
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








The `getstate` method is an essential tool within a blockchain ecosystem, particularly for developers and applications that need to query the state of a specific contract at a particular point in time based on the state root, making it possible to access historical data.

## Prerequisites

Before utilizing the `getstate` method:

1. **StateService Plugin**: This is crucial for handling state-related queries, including accessing historical states.
2. **RpcServer Plugin**: Necessary to accept and process JSON-RPC requests.

Ensure these plugins are installed and properly configured on your blockchain node to utilize this functionality.

## Parameters

- **roothash**: The hash of the state root that you want to query. This value identifies a specific point in the blockchain's history.
- **scripthash**: The hash of the smart contract whose state you're inquiring about. This identifier is unique to each contract.
- **key**: The key for the specific piece of data you're querying within the contract's storage. This value must be Base64-encoded.

## Example Usage

### Request Body

In the given request, the user seeks the state associated with a particular key in the contract's storage, specified by the contract hash, at the blockchain state defined by the root hash.

```json
{
  "jsonrpc": "2.0",
  "method": "getstate",
  "params": ["0xec31cdb14da4143e2ab471a8b5812d895b88fc1c12d54e112791491feca9b5f4","0xb1fbb6b0096919071769906bb23b2ca2ec51eea7","AQFM8QSIkBuHVYOd2kiRmQXXOI833w=="],
  "id": 1
}
```

### Response Body

The response returns the requested state information related to the provided key, encoded in Base64. In this instance, understanding the data returned requires decoding from Base64 and then interpreting based on the contract's storage structure.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "nMJ4AQ=="
}
```

## Considerations

- **Base64 Encoding**: When querying states, keys must be Base64-encoded. Ensure the encoding is correctly implemented to access the desired data.
  
- **Decoding the Result**: The `result` field returns Base64-encoded data. Depending on your application's requirements, you may need to decode this data to interpret it correctly.
  
- **Historical Data Access**: The ability to query state by root hash allows access to historical states, not just the current state. This feature is especially useful for audit purposes or for applications that rely on historical data analysis.

- **Plugin and Node Configuration**: Ensure both the `StateService` and `RpcServer` plugins are properly installed and configured on your node. Incorrect setup could lead to inability to process `getstate` queries effectively.

The `getstate` method is a powerful feature for dApp developers and others within the blockchain ecosystem, allowing for detailed insights and interactions with smart contract states across different points in blockchain history.








<br/>