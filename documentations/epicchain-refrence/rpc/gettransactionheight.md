---
title: Get Transaction Height
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';





The `gettransactionheight` method is utilized to find out the block height at which a particular transaction is included on the blockchain. This is especially useful for tracking the confirmation status of transactions and understanding their placement within the blockchain's ledger.

## Prerequisites

To use this method:

1. **RpcServer Plugin**: Must be installed on your blockchain node. This plugin facilitates the handling of JSON-RPC requests, providing an interface for querying blockchain data, including transaction heights.

## Parameters

- **txid**: The transaction ID (hash) for which you are querying the height. This unique identifier represents the transaction across the blockchain.

## Example Usage

### Scenario

You want to determine the block height for a transaction with a specific hash.

### Request Body

Construct your request by specifying the `txid` as follows:

```json
{
  "jsonrpc": "2.0",
  "method": "gettransactionheight",
  "params": ["0x57280b29c2f9051af6e28a8662b160c216d57c498ee529e0cf271833f90e1a53"],
  "id": 1
}
```

This request includes:
- `jsonrpc` version `2.0`, indicating the JSON-RPC protocol version being used.
- The `method` name, `gettransactionheight`, directed at retrieving the transaction's block height.
- `params`, containing the transaction ID (`txid`) of interest.
- An `id` to uniquely identify the request.

### Response Body

The response will include the block height (`result`) at which the specified transaction has been included:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": 14
}
```

In this example, the transaction identified by the provided hash was included in block height `14`.

## Considerations

- **Confirmation Status**: The transaction height can help evaluate its confirmation status. Generally, the greater the number of subsequent blocks since the transaction's block, the more confirmations it has.

- **Blockchain Synchronization**: Ensure your node is fully synchronized with the blockchain. Querying a recent transaction from an out-of-sync node might lead to inaccurate results or indicate that the transaction is not found.

- **Transaction Existence**: If a transaction ID (`txid`) does not correspond to any transaction on the blockchain, the method may return an error or indicate that the transaction does not exist.

- **RpcServer Plugin Requirement**: Without the `RpcServer` plugin installed and properly configured on your node, this and other JSON-RPC methods cannot be executed.

The `gettransactionheight` method serves a key role in transaction tracking and blockchain navigation, offering clear insights into the lifecycle and confirmation status of transactions.











<br/>