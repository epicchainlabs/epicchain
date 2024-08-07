---
title: Get Native Contracts
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




The `getnativecontracts` method is essential for retrieving a comprehensive list of native contracts deployed on the blockchain. These are the contracts that are built into the core of the blockchain software, providing foundational functionality such as token management, governance, or inter-contract operations. This method gives users and developers insight into the basic information of these integral contracts and their descriptive `manifest.json`, which outlines their capabilities, permissions, and interfaces.

## Prerequisites

Execution of the `getnativecontracts` method requires the `RpcServer` plugin to be installed on your blockchain node. This essential setup allows your node to handle JSON-RPC requests, facilitating interactions with the blockchain's deep functionalities.

## Example Usage

### Request Body

To query the list of all native contracts, you construct your JSON-RPC request as follows:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getnativecontracts",
  "params": []
}
```

In this request:
- `method`: Signifies the action you desire to perform, here being `"getnativecontracts"`.
- `params`: An empty array indicating that no specific parameters are required for this method.
- `id`: A unique identifier for the request, aiding in matching the response.

### Partial Response Body

Assuming the blockchain you are querying has multiple native contracts, the response will include an array under the `result` field, enumerating these contracts. Below is a shortened example showing the structure of just one such contract in the response:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    {
      "id": -1,
      "hash": "0xfffdc93764dbaddd97c48f252a53ea4643faa3fd",
      "nef": {...},
      "manifest": {
        "name": "GasToken",
        "groups": [],
        "supportedstandards": ["NEP-17"],
        "abi": {...},
        "permissions": [...],
        "trusts": [],
        "extra": null
      }
    },
    ...
  ]
}
```

The `result` key wraps an array, each element representing a native contract with:
- **id**: A unique identifier for the contract.
- **hash**: The contract's script hash, serving as its unique address on the blockchain.
- **nef**: Contains compiled contract code information (not fully shown).
- **manifest**: A comprehensive description of the contract, detailing:
  - `name`: The contract's name.
  - `groups`: Container for group data (often empty for native contracts).
  - `supportedstandards`: Any standards the contract adheres to (e.g., NEP-17 for tokens).
  - `abi`: The application binary interface, defining how to interact with the contract.
  - `permissions`, `trusts`, and `extra`: Additional configuration and metadata.

## Key Insights

- **Native Contracts**: They perform essential blockchain operations and are integral to its functionality. Accessing this list allows developers to understand and leverage the core capabilities of the blockchain.
- **Interoperability**: Understanding native contracts and their ABI is crucial for developing applications that interact seamlessly with blockchain features, such as transferring tokens or invoking governance actions.
- **Security**: The permissions outlined in the `manifest` help understand the scope of operations allowed for each contract, facilitating secure interactions with these contracts.

Retrieving the list of native contracts gives users and developers invaluable insights into the blockchain's base layer operations. It serves as a foundational step for developing applications, auditing contract interactions, or simply exploring a blockchain's capabilities.


<br/>