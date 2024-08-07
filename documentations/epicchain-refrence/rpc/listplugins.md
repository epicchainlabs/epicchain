---
title: List Plugins
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';










The `listplugins` method serves a critical function in blockchain node management by providing an outline of all plugins currently loaded by the node. This method is invaluable for developers and node operators to ensure their node is properly configured and to verify the availability and version conformity of essential plugins.

## Prerequisites

- **RpcServer Plugin**: Before invoking the `listplugins` method, the RpcServer plugin must be installed on your blockchain node. This plugin is crucial for handling JSON-RPC requests, enabling your node to interact seamlessly with diverse operations within the blockchain environment.

## Example Usage

### Request Body

Here’s how to construct a JSON-RPC request to retrieve the list of all plugins loaded by the node:

```json
{
  "jsonrpc": "2.0",
  "method": "listplugins",
  "params": [],
  "id": 1
}
```

### Response Body

The response provides a comprehensive view of each plugin, including its name, version, and the interfaces it implements:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    {
      "name": "DBFTPlugin",
      "version": "3.1.0.0",
      "interfaces": [
        "IP2PPlugin"
      ]
    },
    {
      "name": "LevelDBStore",
      "version": "3.1.0.0",
      "interfaces": []
    },
    {
      "name": "TokensTracker",
      "version": "3.1.0.0",
      "interfaces": [
        "IPersistencePlugin"
      ]
    },
    {
      "name": "RpcServer",
      "version": "3.1.0.0",
      "interfaces": []
    },
    {
      "name": "SystemLog",
      "version": "3.1.0.0",
      "interfaces": [
        "ILogPlugin"
      ]
    }
  ]
}
```

### Response Description

- **name**: The plugin's name, identifying the plugin being used.
- **version**: The version of the plugin, providing insight into the plugin's development cycle and compatibility.
- **interfaces**: A list of interfaces implemented by the plugin, shedding light on the functionality and potential integrations offered by the plugin.

## Considerations

- **Plugin Compatibility**: Ensure that the versions of the installed plugins are compatible with your node’s version and with each other to avoid conflicts or runtime errors.
  
- **Security and Performance**: Regularly reviewing the list of installed plugins is crucial for maintaining both the security and performance of your blockchain node. Unnecessary or outdated plugins should be assessed and updated or removed as needed.

The `listplugins` method is an essential tool for blockchain node maintenance, allowing for effective monitoring and management of the node's capabilities and extensions. This method empowers developers and node operators to optimize their blockchain infrastructure for both performance and functionality.







<br/>