---
title: Get Peers
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







The `getpeers` method is a JSON-RPC call provided by blockchain nodes to fetch a list of peer nodes that the queried node is currently connected to, as well as those it is disconnected from or has marked as 'bad'. This method is crucial for understanding the network topology and the node's positioning within the blockchain network.

## Prerequisites

Before you can use the `getpeers` JSON-RPC method, ensure the `RpcServer` plugin is installed on your blockchain node. This plugin enables the node to respond to JSON-RPC requests, facilitating interactions with the network participants and tools that query node information.

## Example Usage

### Request

To obtain the list of peer nodes in relation to the querying node, you would send a request formatted as follows:

```json
{
  "jsonrpc": "2.0",
  "method": "getpeers",
  "params": [],
  "id": 1
}
```

- `method`: Specifies the action, in this case, "getpeers".
- `params`: There are no parameters required for the `getpeers` method, hence an empty array is sent.
- `id`: A unique identifier for the request to match it with its response.

### Response

Upon a successful request, the node will respond with a list of peers categorized into 'unconnected', 'bad', and 'connected':

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "unconnected": [],
    "bad": [],
    "connected": [
      {
        "address": "47.90.28.99",
        "port": 21113
      },
      {
        "address": "47.90.28.99",
        "port": 22113
      }
    ]
  }
}
```

- **Unconnected**: Nodes that are not currently connected but may have been discovered.
- **Bad**: Nodes that have been disconnected due to errors or misbehavior.
- **Connected**: Nodes that are currently actively connected to the querying node.

## Importance

- Understanding network dynamics: Helps in obtaining a visual representation of the blockchain's P2P network, aiding in troubleshooting and network optimization.
- Node’s Network Health: Checking the `connected` list can provide insights into the node's health and its effectiveness in participating in the network.
- Security Assessments: Identifying `bad` nodes can help in assessing potential security threats and in implementing countermeasures.

## Security Considerations

While fetching and analyzing peer information is crucial for network monitoring and management, it's also essential from a security standpoint to manage this information carefully. Bad actors could potentially use peer lists to target specific nodes in the network, necessitating a strategic approach to node security and management.

## Conclusion

The `getpeers` JSON-RPC method is a fundamental tool in the arsenal of blockchain node operators and developers, providing deep insights into the node's interactions within the broader network. Understanding the state and dynamics of peer connections can help ensure robust, secure, and efficient network operation.









<br/>