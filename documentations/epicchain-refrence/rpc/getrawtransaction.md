---
title: Get Raw Transaction
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';






The `getrawtransaction` method retrieves information about a specific transaction based on its hash. It can return the transaction details either as a serialized string (Base64-encoded) or as a JSON object, depending on the verbosity level specified in the request.

## Prerequisites

To use this method, the `RpcServer` plugin must be installed on your blockchain node. This plugin enables the JSON-RPC interface, allowing interaction with the node through JSON-RPC calls.

## Parameters

- **txid**: The transaction ID (hash) for which you're querying the information.
- **verbose** (optional): A boolean flag that dictates the format of the response. By default, it's `false`.
  - When `false`, only the serialized transaction information (Base64-encoded) is returned.
  - When `true`, detailed transaction information is returned in JSON format.

## Example Usage

### Example 1: Get Serialized Transaction Information

#### Request Body

```json
{
  "jsonrpc": "2.0",
  "method": "getrawtransaction",
  "params": ["0x7da6ae7ff9d0b7af3d32f3a2feb2aa96c2a27ef8b651f9a132cfaad6ef20724c"],
  "id": 1
}
```

#### Response Body

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "AIsJtw60lJgAAAAAAAjoIwAAAAAACxcAA..."
}
```

In this example, the response contains the serialized transaction data (Base64-encoded).

### Example 2: Get Verbose Transaction Information

#### Request Body

For verbose transaction details, set the `verbose` parameter to `true`.

```json
{
  "jsonrpc": "2.0",
  "method": "getrawtransaction",
  "params": ["0x7da6ae7ff9d0b7af3d32f3a2feb2aa96c2a27ef8b651f9a132cfaad6ef20724c", true],
  "id": 1
}
```

#### Response Body

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "hash": "0x7da6ae7ff9d0b7af3d32f3a2feb2aa96c2a27ef8b651f9a132cfaad6ef20724c",
    "size": 386,
    "version": 0,
    "nonce": 246876555,
    ...
    "confirmations": 26,
    "blocktime": 1612687482881
  }
}
```

The verbose response includes detailed transaction data such as the transaction hash, size, version, nonce, sender, system fee, network fee, signers, script, witnesses, block hash, confirmations, and block time.

## Considerations

- Serialization may be useful for applications that only need to verify the existence of a transaction or to relay the transaction data elsewhere without needing to parse it.
  
- Verbose information is crucial for analyzing transaction details directly, understanding transaction flows, and debugging.

- Ensure your request matches the transaction ID format expected by the node, which typically involves prefixing the hexadecimal representation of the transaction hash with `0x`.

By utilizing the `getrawtransaction` method, developers and participants can retrieve comprehensive details about transactions on the blockchain, enhancing monitoring, analysis, and integration capabilities.










<br/>