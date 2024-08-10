---
title: Get Contract State
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';



The `getcontractstate` method is a powerful tool designed for obtaining detailed information about a particular smart contract deployed on the blockchain. This includes both contracts identified by their script hash and native contracts recognized by their unique names. 

## Prerequisites

To successfully invoke this method, it's essential to have the `RpcServer` plugin installed on your node. This plugin facilitates communication between your node and the external environment through JSON-RPC requests, allowing for a broad array of blockchain interactions.

## Parameters

- **script_hash / name**: This is the primary parameter for the `getcontractstate` method. You can provide either the script hash, which is a unique identifier for the smart contract on the blockchain, or the native name of the contract if it's a predefined contract by the network.

## Example Usage

### Request by Native Contract Name

This example illustrates how to query the state of a contract using its native name.

```json
{
  "jsonrpc": "2.0",
  "method": "getcontractstate",
  "params": ["epicchaintoken"],
  "id": 1
}
```

### Request by Script Hash

This example demonstrates how to request contract information using its script hash.

```json
{
  "jsonrpc": "2.0",
  "method": "getcontractstate",
  "params": ["0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"],
  "id": 1
}
```

### Detailed Response Analysis

The response to a `getcontractstate` request is extensive and includes several crucial parts:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "id": 383,
    "updatecounter": 0,
    "hash": "Contract Script Hash",
    "nef": {
      "magic": "Magic Number",
      "compiler": "Compiler Used",
      "tokens": [
        {
          "method": "Method Name",
          "paramcount": "Number of Parameters",
          "hasreturnvalue": "Boolean indicating if returns a value",
          "callflags": "Call Flags"
        }
      ],
      "script": "Compiled Contract Script"
    },
    "manifest": {
      "name": "Contract Name",
      "groups": [],
      "supportedstandards": ["NEP-17"],
      "abi": {
        "methods": [
          {
            "name": "Method Name",
            "parameters": [],
            "returntype": "Return Type",
            "offset": "Byte Offset in Script"
          }
        ],
        "events": [
          {
            "name": "Event Name",
            "parameters": [
              {
                "name": "Parameter Name",
                "type": "Parameter Type"
              }
            ]
          }
        ]
      },
      "permissions": [
        {
          "contract": "Other Contract's Script Hash",
          "methods": ["Allowed Methods"]
        }
      ],
      "extra": {
        "Author": "Contract Author",
        "Email": "Author's Email",
        "Description": "Contract Description"
      }
    }
  }
}
```

**Key Insights from the Response:**

- **id & updatecounter**: These numbers provide the contract's unique identifier and the total number of updates it has gone through, respectively.
- **hash**: The unique script hash of the contract.
- **nef**: Contains information about the compiled contract, such as the compiler used and any associated tokens.
- **manifest**: This is the heart of the contract's definition, detailing its name, associated groups, supported standards, ABI (Application Binary Interface), permissions, and any additional information provided by the developer.

The `manifest` section is critical for understanding how to interact with the contract. It outlines:

- **methods**: The functions available within the contract, their parameters, return types, and their security measures.
- **events**: Notifications that the contract can emit, useful for external applications to react to changes within the contract.
- **permissions**: Specifies which contracts and methods this contract is allowed to interact with, adding a layer of security.

## Conclusion

Utilizing the `getcontractstate` method offers deep insights into the structure and capabilities of blockchain contracts. Whether you're a developer looking to integrate with, audit or simply explore the functionalities of a smart contract, this method provides the comprehensive data needed to understand contract behaviors on the blockchain fully.


<br/>