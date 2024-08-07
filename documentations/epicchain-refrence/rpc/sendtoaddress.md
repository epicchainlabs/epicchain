---
title: Send to Address
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







The `sendtoaddress` method enables the transfer of assets to a specified address on the EpicChain network. This procedure is essential for executing transfers and requires the wallet to be opened first for access to assets.

## Prerequisites

- **RpcServer Plugin**: Must be installed on your blockchain node. This plugin allows the node to handle JSON-RPC requests including transactions.
  
- **Open Wallet**: Utilize the `openwallet` method to unlock the wallet. This action grants access to its assets for executing the transfer.

## Parameters Description

- **asset_id**: The asset identifier or the script hash of the NEP-17 contract associated with the asset you wish to transfer.

- **address**: The destination address receiving the assets.

- **value**: The amount to be transferred. Note that the value should be provided in the smallest unit of the asset, depending on its divisibility. For example, to transfer 0.000001 of an asset with 8 decimal places, use a value of 100. For transferring 100 of the same asset, the value would be 10000000000.

## Example Usage

### Request Body

Here is an example of a JSON-RPC request that transfers assets to a specific address:

```json
{
  "jsonrpc": "2.0",
  "method": "sendtoaddress",
  "params": ["0xd2a4cff31913016155e38e474a2c06d08be276cf", "NUuPz4k387bHuySx2e2RWhZj5SpF8V4Csy", 100],
  "id": 1
}
```

### Response Body

Upon successful execution, the response will include detailed information about the transaction:

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "hash": "0x4c072646cf9515dd2a5df9fc4df0563a4f2e468910d24a9b2196743bcea8b8f0",
        ...
        "script": "...",
        "witnesses": [...]
    }
}
```

### Response Description

If the response returns transaction details such as the `hash`, the operation was successful. The `hash` represents the unique identifier of the transaction on the blockchain.

## Handling Errors and Signatures

- **Incomplete Signatures**: If the transaction lacks complete signatures, the response may include information required for signing.
  
- **Insufficient Funds**: A failure to send the transaction due to inadequate balance will trigger an error message detailing the issue.

## Common Considerations

- **Asset Precision**: Ensure the value transferred considers the asset's precision. Incorrect amounts due to misunderstanding precision can lead to failure in executing the desired transaction.

- **Secure Transactions**: Always practice safe security measures when handling wallet operations to avoid unauthorized access to assets.

- **Wallet Compatibility**: Confirm that the wallet you're using supports the NEP-17 assets intended for transfer.

The `sendtoaddress` method is a straightforward yet crucial tool for asset management within the EpicChain ecosystem, facilitating efficient transactions across the network.







<br/>