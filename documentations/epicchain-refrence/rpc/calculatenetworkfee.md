---
title: Calculate Network Fee
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';



The `calculatenetworkfee` method is a vital tool within the EpicChain ecosystem, offering developers and users the ability to accurately calculate the network fee for a specific transaction. This calculation is essential for ensuring that transactions are submitted to the blockchain with an adequate fee to be processed in a timely manner.

## Prerequisites

Before you can utilize the `calculatenetworkfee` method, it's essential to have the `RpcServer` plugin installed on your EpicChain-CLI node. This plugin enables your node to handle JSON-RPC requests, including transaction fee calculations.

## Method Parameters

- **tx**: A string parameter representing the transaction information, encoded in Base64.

## Example

This example demonstrates how to construct a request to calculate the network fee for a given transaction.

### Request Body:

```json
{
  "jsonrpc": "2.0",
  "method": "calculatenetworkfee",
  "params": [
    "AAzUzgl2c4kAAAAAAMhjJAAAAAAAmRQgAAKDHlc9J/rM4KzhpixYX/fRkt2q8ACBubhEJKzaXrq9mt5PesW40qC01AEAXQMA6HZIFwAAAAwUgx5XPSf6zOCs4aYsWF/30ZLdqvAMFIG5uEQkrNpeur2a3k96xbjSoLTUE8AMCHRyYW5zZmVyDBS8r0HWhMfUrW7g2Z2pcHudHwyOZkFifVtSOAJCDED0lByRy1/NfBDdKCFLA3RKAY+LLVeXAvut42izfO6PPsKX0JeaL959L0aucqcxBJfWNF3b+93mt9ItCxRoDnChKQwhAuj/F8Vn1i8nT+JHzIhKKmzTuP0Nd5qMWFYomlYKzKy0C0GVRA14QgxAMbiEtF4zjCUjGAzanxLckFiCY3DeREMGIxyerx5GCG/Ki0LGvNzbvPUAWeVGvbL5TVGlK55VfZECmy8voO1LsisRDCEC6P8XxWfWLydP4kfMiEoqbNO4/Q13moxYViiaVgrMrLQRC0ETje+v"
  ],
  "id": 1
}
```

In this request body, the `params` field contains the Base64-encoded transaction data.

### Response Body:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "networkfee": "23848400"
  }
}
```

The response provides the network fee for the transaction in question, allowing the sender to adjust their transaction accordingly to ensure optimal processing time.

## Key Takeaways

- Utilizing `calculatenetworkfee` is essential for determining the adequate network fee for transactions on EpicChain.
- This method aids in preventing underpayment of transaction fees, which can lead to delays in processing.
- It's crucial to have the `RpcServer` plugin installed and properly configured to access this and other JSON-RPC methods.

By incorporating `calculatenetworkfee` into your transaction process, you can optimize your interactions with the EpicChain network, ensuring efficient and timely transaction confirmation.



<br/>