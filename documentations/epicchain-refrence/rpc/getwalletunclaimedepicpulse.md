---
title: Get Wallet Unclaimed EpicPulse
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';






# JSON-RPC Method: getwalletunclaimedepicpulse

The `getwalletunclaimedepicpulse` method provides a mechanism to query the amount of unclaimed EpicPulse tokens within a wallet. This functionality is particularly useful for participants of the EpicChain ecosystem looking to manage their EpicPulse asset holdings effectively.

## Prerequisites

In order to successfully invoke this method:

1. **RpcServer Plugin**: The RpcServer plugin needs to be installed on your blockchain node. This plugin is essential for serving JSON-RPC requests, including those related to wallet queries such as the unclaimed EpicPulse tokens.

2. **Open Wallet**: Prior to querying for unclaimed EpicPulse, the wallet in question must be opened using the `openwallet` RPC method. This step ensures that the method has access to the necessary wallet data to perform the query.

## Parameters

- **address**: The specified wallet address from which to query the unclaimed EpicPulse tokens. This address should be involved in the operations or transactions that accrue EpicPulse tokens that are yet to be claimed.

## Example Usage

### Request Body

The following request structure demonstrates how to check for unclaimed EpicPulse tokens for a given wallet address:

```json
{
  "jsonrpc": "2.0",
  "method": "getwalletunclaimedepicpulse",
  "params": ["NgaiKFjurmNmiRzDRQGs44yzByXuSkdGPF"],
  "id": 1
}
```

### Response Body

The response from the node will include the amount of unclaimed EpicPulse tokens associated with the specified wallet address:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "750000000"
}
```

### Response Description

- **result**: Indicates the total amount of unclaimed EpicPulse tokens in the specified wallet. The value is returned as a string representation of the number of tokens. 

## Considerations

- **Accuracy**: It is important to ensure that your client or node is fully synchronized with the blockchain to obtain an accurate count of unclaimed EpicPulse tokens. A node that is not fully synced may return outdated or incorrect information.

- **Security**: As with any operation that involves accessing wallet data, security considerations should be paramount. Ensure that the `openwallet` operation is conducted in a secure manner to mitigate the risk of unauthorized access.

- **Compatibility**: This method is specific to the EpicChain ecosystem and its associated tokens, EpicPulse. Ensure that your blockchain and wallet are compatible with the EEP-17 standard and specifically configured for EpicChain operations.

The `getwalletunclaimedepicpulse` method is a powerful tool for users interacting with the EpicChain ecosystem, enabling effective management of the EpicPulse tokens. By understanding the prerequisites and following the proper querying process, users can efficiently access and manage their unclaimed tokens.








<br/>