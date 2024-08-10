---
title: Close Wallet
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';



The `closewallet` method is a straightforward yet critical function within the EpicChain ecosystem, enabling users and applications to securely close the currently opened wallet through the JSON-RPC interface. This action is essential for maintaining security and integrity of wallet operations, especially in automated or application-driven environments.

## Prerequisites

To utilize the `closewallet` method effectively, the following conditions must be met:

- **RpcServer Plugin**: Ensure the `RpcServer` plugin is installed on your EpicChain node. This plugin is a prerequisite for handling JSON-RPC requests, including wallet management functions.
  
- **Open Wallet**: The `openwallet` RPC method must have been previously called to open a wallet. The `closewallet` method is designed to close a wallet that's currently open in the node's context.

## Example

This example demonstrates the protocol to construct a JSON-RPC request to close an open wallet.

### Request Body:

```json
{
  "jsonrpc": "2.0",
  "method": "closewallet",
  "params": [],
  "id": 1
}
```

In this request, the method name `closewallet` indicates the action to be performed, and no parameters are required. The `id` field helps in matching requests with their responses, aiding in request-response tracking in asynchronous environments or batch processing. 

### Response Body:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": true
}
```

The response body returns a Boolean value in the `result` field, which indicates the outcome of the close wallet operation.

## Response Description:

- **true**: Indicates that the wallet has been closed successfully.
  
- **Others**: Any response other than `true` can be considered a failure to close the wallet, which may require further investigation or corrective actions.

## Key Takeaways

- **Security**: Regularly closing the wallet when not in active use enhances security, especially in automated systems or applications.
  
- **Prerequisites**: Before calling `closewallet`, ensure the wallet is opened using `openwallet`, and the `RpcServer` plugin is active.
  
- **RPC Interaction**: This method, along with others, forms the basis for secure and flexible wallet management through EpicChain's JSON-RPC interface.

The `closewallet` RPC method is part of a comprehensive set of tools provided by EpicChain for efficient and secure blockchain and wallet management. Proper utilization of these tools can significantly enhance the user experience and automate tasks within the EpicChain ecosystem.


<br/>