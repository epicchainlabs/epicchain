---
title: Get Application Log
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




The `getapplicationlog` method is a crucial tool in the EpicChain blockchain ecosystem, designed to retrieve contract event information using a specific transaction ID (`txid`). This functionality is indispensable for developers and users who need to verify the outcomes of transactions and smart contract executions.

## Prerequisites
Before you can use the `getapplicationlog` method, you must install the `ApplicationLogs` and `LevelDBStore` plugins on your EpicChain node. These plugins enable the node to log application events and store data, respectively.

## Parameters
- **txid**: The transaction ID for which you want to retrieve the contract event information.
- **trigger type (Optional)**: Allows you to filter the response based on the type of trigger. Available options include:
  - `OnPersist`
  - `PostPersist`
  - `Application`
  - `Verification`
  - `System`: Combines `OnPersist` and `PostPersist`
  - `All`: Includes `OnPersist`, `PostPersist`, `Verification`, and `Application`. This is the default setting if no trigger type is specified.

## Example Usage

### Request Body

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getapplicationlog",
  "params": [
    "0x7da6ae7ff9d0b7af3d32f3a2feb2aa96c2a27ef8b651f9a132cfaad6ef20724c"
  ]
}
```

This request is querying the application log for a transaction that involves a GAS transfer.

### Response Body

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "txid": "0x7da6ae7ff9d0b7af3d32f3a2feb2aa96c2a27ef8b651f9a132cfaad6ef20724c",
    "executions": [
      {
        "trigger": "Application",
        "vmstate": "HALT",
        "exception": null,
        "gasconsumed": "9999540",
        "stack": [],
        "notifications": [
          {
            "contract": "0x70e2301955bf1e74cbb31d18c2f96972abadb328",
            "eventname": "Transfer",
            "state": {
              "type": "Array",
              "value": [
                {
                  "type": "ByteString",
                  "value": "4rZTInKT6ZxPKQbVNVOrtKZy34Y="
                },
                {
                  "type": "ByteString",
                  "value": "+on7LBTfD1nd3wT25WUX8rNKrus="
                },
                {
                  "type": "Integer",
                  "value": "10000000000"
                }
              ]
            }
          }
        ]
      }
    ]
  }
}
```

### Response Description
- **txid**: Echoes the transaction ID queried.
- **trigger**: Specifies the type of trigger that invoked the event.
- **vmstate**: Indicates the virtual machine's state post-execution; `HALT` signifies success, `FAULT` indicates failure.
- **gasconsumed**: Represents the GAS consumed to execute the transaction.
- **notifications**: Lists the notifications emitted by the smart contracts during the transaction's execution.
  - The `contract` field identifies the contract that sent the notification (e.g., GasToken).
  - `eventname` denotes the event's identifier.
  - `state` contains the event data, with `ByteString` values typically representing Base64-encoded wallet addresses.

## Key Insights
- The `getapplicationlog` method is vital for tracking and understanding the outcomes of transactions and smart contract executions on the blockchain.
- Notifications within the application log provide detailed insights into the actions performed by smart contracts, including asset transfers.
- By examining the `vmstate` and the presence of `Transfer` notification events, users can discern the success or failure of transactions.

Remember, the transparency and auditability facilitated by methods like `getapplicationlog` are fundamental characteristics that enhance the reliability and trustworthiness of blockchain platforms like EpicChain.


<br/>