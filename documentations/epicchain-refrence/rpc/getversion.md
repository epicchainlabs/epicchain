---
title: Get Version
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








The `getversion` method is used to obtain comprehensive version information about a blockchain node. This includes network configuration parameters, software version, and other crucial details that define the node's capabilities and compatibility within the network.

## Prerequisites

- **RpcServer Plugin**: To invoke this method, the RpcServer plugin must be installed on your blockchain node. This plugin provides the necessary interface for processing JSON-RPC requests, enabling external queries about the node's configuration and status.

## Example Usage

### Request Body

To request the version information of the node, your JSON-RPC call should look as follows:

```json
{
  "jsonrpc": "2.0",
  "method": "getversion",
  "params": [],
  "id": 1
}
```

This request specifies:
- The JSON-RPC version (`2.0`),
- The method name (`getversion`),
- An empty `params` array, as this method does not require any parameters,
- A unique identifier for the request (`id = 1`).

### Response Body

The response contains detailed information about the node, including network ports, the user agent (which typically includes the software version), and various protocol parameters significant to the node's operation on the network:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "tcpport": 10333,
    "wsport": 10334,
    "nonce": 1930156121,
    "useragent": "/EpicChain:1.0.1/",
    "protocol": {
      "addressversion": 53,
      "network": 860833102,
      "validatorscount": 7,
      "msperblock": 15000,
      "maxtraceableblocks": 2102400,
      "maxvaliduntilblockincrement": 5760,
      "maxtransactionsperblock": 512,
      "memorypoolmaxtransactions": 50000,
      "initialgasdistribution": 5200000000000000
    }
  }
}
```

### Key Information Provided

- **tcpport & wsport**: The TCP and WebSocket ports on which the node listens for incoming connections.
- **nonce**: A unique identifier for the node, useful in network communications.
- **useragent**: Describes the node's software version and often contains the name of the blockchain or the specific software.
- **protocol**: Describes various protocol-related configurations, including:
  - `addressversion`: The version of the address protocol.
  - `network`: Identifies the specific blockchain network the node is part of, useful in distinguishing between mainnet, testnet, and others.
  - `validatorscount`: The number of validators in the network (for blockchains using consensus mechanisms involving validators).
  - `msperblock`: The target time (in milliseconds) for generating a new block.
  - `maxtraceableblocks`, `maxvaliduntilblockincrement`, `maxtransactionsperblock`, `memorypoolmaxtransactions`, `initialgasdistribution`: Detailed configuration values defining the transaction processing capabilities and limitations of the node.

## Considerations

- Understanding the version and capabilities of a node is crucial for developers and administrators when deploying applications, performing node maintenance, or troubleshooting network issues.
- The `getversion` method can be especially useful in networks with multiple software versions or forks, where compatibility may vary.
- Always ensure your node's software is up-to-date to maintain compatibility with network protocols and maximize security.

The `getversion` method provides an easy way to query significant details about a blockchain node, helping maintain and monitor its performance and compatibility within the broader network ecosystem.








<br/>