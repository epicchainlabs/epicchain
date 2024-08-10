---
title: Open Wallet
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







The `openwallet` method is an advanced feature used to open a specified wallet file through the JSON-RPC interface. Given the sensitive nature of this operation, it is disabled by default to protect user security. Should you require the use of this method, it necessitates manual activation within the RpcServer configuration settings.

## Prerequisites

- **RpcServer Plugin**: Installation of the RpcServer plugin on your blockchain node is crucial for processing JSON-RPC requests, including wallet management operations like `openwallet`.

## Parameters Description

- **path**: This is the file path to the wallet that you intend to open. It should be accessible by the node.
  
- **password**: The password for the wallet, provided in plain text. Extreme caution should be exercised with this parameter to prevent unauthorized access.

## Enabling `openwallet`

To activate the `openwallet` method, you must edit the RpcServer’s `config.json` file, specifically adjusting the `DisabledMethods` field to allow `openwallet`. Removing `openwallet` from the list of disabled methods will enable this capability on your node.

## Example Usage

### Request Body

Here’s how to format a JSON-RPC request that attempts to open a wallet named `11.db3` with the password `1`:

```json
{
  "jsonrpc": "2.0",
  "method": "openwallet",
  "params": ["11.db3", "1"],
  "id": 1
}
```

### Response Body

A successful execution returns a simple confirmation that the wallet has been opened:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": true
}
```

### Response Description

- **true**: Indicates successful opening of the specified wallet.
- **Others**: Any other response signals that the wallet could not be opened, potentially due to incorrect path, password, or if the method is disabled.

## Considerations

- **Security**: Given the operation's sensitivity (exposing passwords in plain text), ensure your JSON-RPC endpoint is secured. Use HTTPS for communications and restrict access to trusted clients only.
  
- **Configuration Management**: Directly modifying the `config.json` to enable the `openwallet` method increases operational risk. Ensure proper backups and validation steps are in place before making such changes.

- **Access Control**: If the `openwallet` method is enabled and exposed, monitor and control access diligently to prevent unauthorized use.

The `openwallet` method provides powerful functionality for wallet management through RPC calls, supporting scenarios where automated or remote wallet operations are necessary. However, the enablement and use of this feature should be managed with utmost care to maintain the security of blockchain assets.









<br/>