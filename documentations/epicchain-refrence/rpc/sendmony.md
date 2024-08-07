---
title: Send Money
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








The `sendmoney` method facilitates bulk transfers, allowing for multiple transfers within a single transaction. This method is particularly useful for distributing assets to multiple addresses efficiently. It also supports specifying a change address if the exact amount of the transfer does not deplete the source account.

## Prerequisites

Before you can use the `sendmoney` method:

- **RpcServer Plugin**: Must be installed on your blockchain node to handle the JSON-RPC request.
  
- **Open Wallet**: The wallet needs to be opened using the `openwallet` RPC method, ensuring that the necessary assets and permissions are available for initiating the transfers.

## Parameters Description

- **from** (Optional): The address initiating the transfer. If not specified, the wallet's default address is assumed.

- **outputs_array**: A structured array, where each element contains the following properties:
  - **asset**: The EEP-17 contract script hash identifying the asset to transfer.
  - **value**: The amount of the asset to transfer.
  - **address**: The destination address for the transfer.
  - **signers** (Optional): Signature accounts authorizing the transaction, primarily the `from` address.

## Examples

### Bulk Transfer without Specifying `from` Address

A JSON-RPC request to send multiple assets to various addresses:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "sendmoney",
  "params": [
    "",
    [
        {"asset": "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5", "value": 100, "address": "NbtvbHpwv6nswDtVFpKEyooHhDHwZh2LHf"},
        {"asset": "0xd2a4cff31913016155e38e474a2c06d08be276cf", "value": 100, "address": "NbtvbHpwv6nswDtVFpKEyooHhDHwZh2LHf"}
    ]
  ]
}
```

### Bulk Transfer with Specified `from` Address

Here’s a request specifying the `from` address for a more explicit transaction control:

```json
{
  "jsonrpc": "2.0",
  "method": "sendmoney",
  "params": [
    "NY9nnDv7cAJ44C2xeRScrXfzkrCJfFWYku",
    [
        {"asset": "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5", "value": 100, "address": "NbtvbHpwv6nswDtVFpKEyooHhDHwZh2LHf"},
        {"asset": "0xd2a4cff31913016155e38e474a2c06d08be276cf", "value": 100, "address": "NPTvd2T1zi7ioj3LmvpeBd45pPvAJU3gvr"}
    ]
  ],
  "id": 1
}
```

### Response

The server will respond with transaction details upon success:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "hash": "0xe8742fc5a69f3180ab59f3f21695ce5459891429682a7f1df38219bc05cce39e",
    ...
    "signers": [...],
    "script": "...",
    "witnesses": [...]
  }
}
```

### Response Description

- **hash**: The unique identifier of the transaction.
- **signers**: Lists the accounts that signed the transaction, reflecting the provided `signers` parameter.
- **script**: The script executed by the VM to facilitate the transfers.
- **witnesses**: Contains the invocation and verification scripts facilitating transaction execution.

## Considerations

- Ensure the asset ID(s) are correct as per the EEP-17 contract script hash.
- Verify the balances in `from` address(es) are sufficient to cover the transfer amounts and the transaction fees.
- Utilize secure measures for `openwallet` and transaction signing to safeguard assets.

The `sendmoney` method streamlines asset distribution, making it an essential tool for bulk transfers within blockchain applications, enabling efficient asset management and distribution.









<br/>