---
title: Get Unclaimed EpicPulse
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';





# JSON-RPC Method: getunclaimedEpicPulse

The `getunclaimedepicpulse` method is used to determine the amount of unclaimed EpicPulse associated with a given address on the blockchain. EpicPulse is often a network's utility token, used to pay for transaction fees, smart contract deployments, and other network operations. Over time, addresses accumulate EpicPulse that can be claimed based on their blockchain activities.

## Prerequisites

- **RpcServer Plugin**: This plugin must be installed on your blockchain node. It enables the node to accept and process JSON-RPC requests, crucial for querying blockchain data like unclaimed EpicPulse for an address.

## Parameters

- **address**: The blockchain address for which you're querying unclaimed EpicPulse. This is typically a public address associated with a wallet or contract.

## Example Usage

### Scenario

You want to find out how much unclaimed EpicPulse is associated with a specific address.

### Request Body

To query this information, structure your request as below, replacing `"NgaiKFjurmNmiRzDRQGs44yzByXuSkdGPF"` with the address of interest:

```json
{
  "jsonrpc": "2.0",
  "method": "getunclaimedepicpulse",
  "params": ["NgaiKFjurmNmiRzDRQGs44yzByXuSkdGPF"],
  "id": 1
}
```

This request specifies the JSON-RPC version (`2.0`), the method name (`getunclaimedepicpulse`), the address as a parameter in the `params` array, and a unique identifier for the request (`id`).

### Response Body

The response will include the amount of unclaimed EpicPulse at the specified address and echo back the address queried:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "unclaimed": "499999500",
    "address": "NgaiKFjurmNmiRzDRQGs44yzByXuSkdGPF"
  }
}
```

In this response:
- **unclaimed**: Represents the amount of unclaimed EpicPulse associated with the address. The unit and format of this value may vary between different blockchain networks.
- **address**: Confirms the address for which the unclaimed EpicPulse amount was queried.

## Considerations

- **Address Accuracy**: Ensure the address supplied is correct and conforms to the address format of the specific blockchain network being queried.
- **Synchronization**: Your node must be fully synchronized with the blockchain to ensure it returns the most current unclaimed EpicPulse amount. An out-of-sync node may provide outdated information.
- **RpcServer Plugin**: Proper installation and configuration of the RpcServer plugin are crucial for the successful execution of this and other JSON-RPC methods.

Querying unclaimed EpicPulse can be particularly useful for users managing their resources on a blockchain, enabling them to claim and utilize this EpicPulse for their transactions and activities within the network.









<br/>