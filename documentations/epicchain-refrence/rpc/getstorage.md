---
title: Get Storage
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';






The `getstorage` method is crucial for interacting with smart contracts on a blockchain. It allows you to retrieve the value associated with a specified key from a contract's storage, with both the key and the returned value being Base64-encoded. This feature is particularly useful for developers and users who need to query contract states or data directly from the blockchain.

## Prerequisites

Before executing the `getstorage` method, ensure:

1. **RpcServer Plugin**: This plugin must be installed on your blockchain node. It allows the node to process JSON-RPC requests, including storage queries.

## Parameters

- **script_hash**: The hash of the smart contract script or the contract's ID. This identifier is used to locate the contract on the blockchain.
- **key**: The key for the data you want to retrieve from the contract's storage. The key must be Base64-encoded.

## Example Usage

### Scenario

You want to retrieve a value stored in a smart contract. The value you're interested in is associated with the key "hello". First, you'll need to convert "hello" into its Base64 representation, which is "aGVsbG8=".

### Request Body

Using the contract's script hash and the Base64-encoded key, structure your request as follows:

```json
{
  "jsonrpc": "2.0",
  "method": "getstorage",
  "params": ["0x99042d380f2b754175717bb932a911bc0bb0ad7d", "aGVsbG8="],
  "id": 1
}
```

### Response Body

The response contains the stored value associated with the given key, also Base64-encoded. In this case, the result "d29ybGQ=" is the Base64-encoded form of the word "world".

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "d29ybGQ="
}
```

### Decoding the Result

To interpret the result, you need to decode it from Base64. Converting "d29ybGQ=" back to ASCII yields "world".

## Considerations

- **Key Encoding**: Always remember to Base64-encode your keys before sending the request. Failure to encode correctly could result in an unsuccessful query.

- **Value Decoding**: The values retrieved are also Base64-encoded. You'll need to decode them to gain meaningful insights.

- **Script Hash**: Ensure the contract's script hash is accurate. Incorrect script hashes will direct the query to the wrong contract, leading to errors or null results.

- **Plugin Installation**: The `RpcServer` plugin must be installed and configured correctly. Without this, the node cannot handle `getstorage` queries.

The `getstorage` method is an essential tool for blockchain developers and users, offering direct access to the data stored within smart contracts. This enables a wide range of applications, from simple data queries to complex interaction scripts, enhancing the blockchain's utility and accessibility.










<br/>