---
title: Get State Height
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







The `getstateheight` method allows querying the current state root height in the blockchain. This method is particularly useful for obtaining a snapshot of the blockchain's state at a specific height, aiding in tasks such as synchronization checks, audit, and historical data retrieval.

## Prerequisites

To utilize this method, the following requirements must be met:

1. **StateService Plugin**: It's essential for handling queries related to the blockchain state, such as retrieving the state root height.
2. **RpcServer Plugin**: This plugin enables the JSON-RPC interface on the node, allowing for the execution of methods like `getstateheight`.

Ensure both plugins are installed and appropriately configured on your blockchain node.

## Example Usage

### Request Body

The request to fetch the state root height is straightforward, requiring no parameters:

```json
{
  "jsonrpc": "2.0",
  "method": "getstateheight",
  "params": [],
  "id": 1
}
```

### Response Body

The response contains two crucial pieces of information:

- **localrootindex**: The height of the state root that the local node has processed or stored. It indicates up to which block height the state has been calculated and is available on the local node.

- **validatedrootindex**: The height of the state root that has been validated. This often matches the `localrootindex` but is specifically significant in ensuring that the state at this height is confirmed and agreed upon by the network.

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": {
    "localrootindex": 602,
    "validatedrootindex": 602
  }
}
```

## Considerations

- Knowing the state root height is crucial for applications and services that operate with historical data or require checkpoints for data integrity.

- A discrepancy between `localrootindex` and `validatedrootindex` could imply a delay or issue in state validation or synchronization with the network. It's critical to monitor these values, especially in nodes crucial for operational tasks.

- When setting up a new node or performing maintenance, the `getstateheight` method can be used to confirm successful synchronisation with the network state.

The `getstateheight` method is a valuable tool for developers, node operators, and anyone involved in blockchain data analysis or infrastructure maintenance, providing insights into the blockchain's state integrity and synchronization status.









<br/>