---
title: Invoke Function
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';






The `invokefunction` method performs a vital role in interacting with smart contracts in the blockchain environment. This method invokes a specified function within a smart contract, utilizing its script hash, along with specific operations and parameters, and returns the cryptographic simulation result. It's crucial for testing smart contract functions and observing their behaviors without making permanent changes to the blockchain.

## Prerequisites

Before invoking this method:

- **RpcServer Plugin**: Ensure the RpcServer plugin is installed on the blockchain node, enabling it to process JSON-RPC requests.

## Parameters Description

- **scripthash**: The hash of the smart contract you wish to operate on. The byte order should correspond with the data type: big endian for `Hash160`, little endian for `ByteArray`.
- **operation**: A string denoting the name of the function you want to invoke within the smart contract.
- **params**: (Optional) Arguments to be passed to the smart contract function. 
- **signers**: (Optional) A list detailing the contract signature accounts for executing the function, including:
  - **account**: The account providing the signature.
  - **scopes**: Defines the level of access the signature has.
  - **allowedcontracts**: (Optional) Specifies permissible contracts in `CustomContracts` scope.
  - **allowedgroups**: (Optional) Specifies permissible public keys in `CustomGroups` scope.

### Scopes
The **scopes** can have various access levels:

- `None`, `CalledByEntry`, `CustomContracts`, `CustomGroups`, `Global`, each defining the range and security of the operation.

### Use Diagnostic
- **use diagnostic**: Determines whether to return simulated invocation and storage change information, defaulting to false.

## Example Usage

### Request Body

Here's how you might structure a JSON-RPC request to invoke a `transfer` operation of a smart contract:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "invokefunction",
  "params": [
    "0xa1a375677dded85db80a852c28c2431cab29e2c4",
    "transfer",
    [
      {
        "type": "Hash160",
        "value": "0xfa03cb7b40072c69ca41f0ad3606a548f1d59966"
      },
      {
        "type": "Hash160",
        "value": "0xebae4ab3f21765e5f604dfdd590fdf142cfb89fa"
      },
      {
        "type": "Integer",
        "value": "10000"
      },
      {
        "type": "String",
        "value": ""
      }
    ],
    [
      {
        "account": "0xfa03cb7b40072c69ca41f0ad3606a548f1d59966",
        "scopes": "CalledByEntry"
      }
    ],
    true
  ]
}
```

### Response Body

The response returns detailed information about the invocation, including the script, execution state, consumed system fee, and more:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "script": "...",
    "state": "HALT",
    "epicpulseconsumed": "1490312",
    "exception": null,
    "notifications": [...],
    "diagnostics": {...},
    "stack": [
      {
        "type": "Boolean",
        "value": true
      }
    ],
    "tx": "..."
  }
}
```

### Response Description

- **script**: The executed invocation script.
- **state**: "HALT" indicates successful execution; "FAULT" denotes an exception occurred.
- **epicpulseconsumed**: System fee used for this invocation.
- **exception**: Details any exceptions encountered during execution (null if none).
- **notifications**: Events emitted during contract execution.
- **diagnostics**: Detailed info about invoked contracts & simulated storage changes.
- **stack**: The results of the contract execution, often indicating success or specific outputs from the function called.
- **tx**: The transaction hex string, is present when the wallet is open and signers correctly added.

## Considerations

- **Security & synchronization**: Ensure the blockchain node is securely configured and fully synchronized for accurate results.
- **Stack interpretation**: Understand the type and structure of data returned in the `stack` for appropriate interaction with contract outputs.

This `invokefunction` method furnishes developers with a powerful tool for dynamically interacting with and testing smart contracts, opening avenues for development, debugging, and integration within the blockchain ecosystem.










<br/>