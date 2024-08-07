---
title: Get Wallet Balance
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








The `getwalletbalance` method provides a way to check the balance of specific assets within a wallet. This method is particularly useful for assets that comply with the EEP-17 standard, which is a common token standard on many blockchain platforms, including EpicChain.

## Prerequisites

Before invoking this method:

1. **RpcServer Plugin**: Ensure that the RpcServer plugin is installed on your blockchain node. This plugin enables the handling of JSON-RPC requests.

2. **Open Wallet**: Use the `openwallet` RPC method to unlock the wallet. This step is necessary because the balance information is derived from the wallet, which must be accessible.

3. **Client Synchronization**: Make sure your client is fully synchronized with the blockchain up to the latest block. Out-of-date information can lead to inaccurate balance reporting.

## Parameters

- **Asset_id**: This is the asset identifier, represented by the script hash of the contract for EEP-17 compliant contract assets. Each asset on the blockchain has a unique `Asset_id`.

For instance:
- For EpicChain's native token, the hypothetical `Asset_id` might be `0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5`.
- For Gas tokens, a common utility token, the `Asset_id` might be `0xd2a4cff31913016155e38e474a2c06d08be276cf`.

## Example Usage

### Querying the Balance of EEP-17 Assets

#### Request Body

The following request checks the balance of a specific asset (identified by its asset ID) within the wallet:

```json
{
  "jsonrpc": "2.0",
  "method": "getwalletbalance",
  "params": ["0xd2a4cff31913016155e38e474a2c06d08be276cf"],
  "id": 1
}
```

#### Response Body

The response includes the current balance of the queried asset in the wallet:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
      "balance": "3000014661474560"
  }
}
```

### Response Description

- **balance**: Indicates the balance of the queried asset in the wallet. Given EEP-17 assets use a balance system (as opposed to the UTXO system used by some cryptocurrencies), the balance reported is directly usable without the need for confirmations.

## Considerations

- **Accurate Synchronization**: Execution of the `getwalletbalance` API prior to the client synchronizing to the block where the contract was deployed can result in errors. Accurate and up-to-date client synchronization is crucial.

- **EEP-17 Compliance**: Attempting to query the balance for assets that are not EEP-17 compliant by using their script hash will lead to an error. This method is specifically designed for EEP-17 assets.

- **Privacy and Security**: As with any operation that involves accessing wallet information, ensure the security and privacy of your operations. Opening a wallet potentially exposes it to risk, so be cautious about where and how you use this method.

`getwalletbalance` is a powerful tool for managing and querying asset balances within wallets on blockchain networks that adhere to EEP-17 (or similar) standards. Proper use requires attention to security, synchronization status, and alignment with token standards.








<br/>