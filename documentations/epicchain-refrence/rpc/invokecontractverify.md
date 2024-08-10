---
title: Invoke Contract Verify
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';






The `invokecontractverify` method provides developers with a sophisticated mechanism for testing the verification capabilities of smart contracts deployed on the blockchain. This method executes the contract's `Verify` method via the Verification trigger, which allows for testing without actual transactions being processed on the blockchain. This RPC call is crucial for verifying contract permissions, signatures, and roles in a controlled environment.

## Prerequisites

Before invoking this method:

- **RpcServer Plugin**: Ensure the RpcServer plugin is installed on your blockchain node. This plugin is essential for processing JSON-RPC requests.

## Parameters

- **scripthash**: The smart contract's script hash that you wish to invoke.
- **params**: Parameters intended to be passed to the smart contract's operation. The specific type and number of parameters depend upon the contract.
- **signers** (Optional): 
  - **account**: The account that will sign the contract.
  - **scopes**: Defines the valid scopes for the signature (e.g., `FeeOnly`, `CalledByEntry`, `CustomContracts`, `CustomGroups`, `Global`).
  - **allowedcontracts** (Optional): Specifies contracts that the signature can affect when `scopes` is set to `CustomContracts`.
  - **allowedgroups** (Optional): Specifies public keys that the signature can affect when `scopes` is set to `CustomGroups`.

### Byte Order

The address's proper byte order must be observed according to its data type:
- Use big endian for `Hash160`.
- Use little endian for `ByteArray`.

## Example Usage

### Request Body

The following exemplifies a JSON-RPC request invoking the `Verify` operation of a smart contract, specifying an account with a `CalledByEntry` scope:

```json
{
  "jsonrpc": "2.0",
  "method": "invokecontractverify",
  "params": [
    "0x92f5c79b88560584a900cfec15b0e00dc4d58b54",
    [],
    [
      {
        "account": "NTpqYncLsNNsMco71d9qrd5AWXdCq8YLAA",
        "scopes": "CalledByEntry"
      }
    ]
  ],
  "id": 1
}
```

### Response Body

The response provides detailed information about the contract invocation, including the execution script, state, gas consumption, and execution result:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "script": "VgEMFFbIjRQK0swPKQN90Qp/AGCitShcYEBXAANAQZv2Z84MBWhlbGxvDAV3b3JsZFNB5j8YhEBXAQAMFFbIjRQK0swPKQN90Qp/AGCitShcQfgn7IxwaEA=",
    "state": "HALT",
    "epicpulseconsumed": "1017810",
    "exception": null,
    "stack": [
      {
        "type": "Boolean",
        "value": true
      }
    ]
  }
}
```

### Response Description

- **script**: The script that was invoked, encoded in Base64. This script represents the operations performed by the contract during the execution.
- **state**: Indicates the Virtual Machine (VM) execution state. `HALT` signifies successful execution, while `FAULT` indicates an execution error.
- **epicpulseconsumed**: Displays the amount of system fee consumed for the invocation.
- **stack**: Shows the execution result of the contract. Data types such as `String` or `ByteArray` are Base64-encoded.

## Considerations

- This method does not impact the blockchain state; it merely simulates the contract execution to verify its behavior.
- Proper synchronization with the blockchain network ensures the accuracy and relevance of the contract execution result.
- Understanding the contract's expected parameters and operation logic is essential for constructing valid test invocations.

By leveraging the `invokecontractverify` method, developers can rigorously test and ensure their smart contracts behave as expected in various transaction and signature scenarios without committing to actual transactions. This contributes significantly to the development and debugging process of secure and efficient smart contracts on the blockchain.










<br/>