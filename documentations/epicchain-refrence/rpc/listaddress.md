---
title: List Address
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';









The `listaddress` method is a useful tool within the blockchain ecosystem, enabling users to retrieve a comprehensive list of all addresses contained within the currently opened wallet. This functionality is particularly essential for managing and auditing wallet contents, ensuring users have immediate access to their addresses—both for sending transactions and for monitoring purposes.

## Prerequisites

- **RpcServer Plugin**: This method requires the RpcServer plugin to be installed on your blockchain node. The plugin facilitates the node's ability to handle JSON-RPC requests.
  
- **Open Wallet**: Prior to invoking `listaddress`, the `openwallet` RPC method must be utilized to unlock the wallet. This step is mandatory to access and list the wallet's addresses.

## Parameters Description

- **No parameters required**: The `listaddress` method does not necessitate any parameters, simplifying the process of querying wallet addresses.

## Example Usage

### Request Body

Here's how to structure a JSON-RPC request for listing all addresses in the currently open wallet:

```json
{
  "jsonrpc": "2.0",
  "method": "listaddress",
  "params": [],
  "id": 1
}
```

### Response Body

The response provides a detailed account of each address within the wallet, along with associated metadata:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    {
      "address": "NikhQp1aAD1YFCiwknhM5LQQebj4464bCJ",
      "haskey": true,
      "label": null,
      "watchonly": false
    },
    {
      "address": "NgaiKFjurmNmiRzDRQGs44yzByXuSkdGPF",
      "haskey": true,
      "label": null,
      "watchonly": false
    }
  ]
}
```

### Response Description

- **address**: Indicates the unique identifier or location code for the wallet address in question.
- **haskey**: A boolean value stating whether a private key exists for this address within the wallet; critical for transactions and ownership.
- **label**: An optional label that may be assigned to the address for easier identification or organization purposes (null if not assigned).
- **watchonly**: Specifies whether the address is a watch-only address, denoting it's being monitored without access to the private key (useful for tracking purposes without ownership).

## Considerations

- **Security**: As the `listaddress` method involves accessing sensitive wallet information, ensure that your node and wallet are secured against unauthorized access.
  
- **Wallet Management**: This method serves as a pivotal tool in wallet management workflows, enabling you to audit addresses, verify ownership, and organize your assets securely.

The `listaddress` method offers a streamlined, effective way for users and wallet managers to maintain oversight over their wallet addresses, bolstering both organizational and security protocols within their blockchain operations.







<br/>