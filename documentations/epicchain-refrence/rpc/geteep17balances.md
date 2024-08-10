---
title: Get EEP17 Balance
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';






The `geteep17balances` method is utilized to query the balance of all EEP17 (equivalent to ERC-20 in Ethereum) assets held by a specified address. This method proves invaluable for applications requiring real-time data on token balances for wallet addresses within a blockchain network supporting the EEP17 standard.

## Prerequisites

Before you can invoke the `geteep17balances` method, ensure that your blockchain node has the `TokensTracker`, `LevelDBStore`, and `RpcServer` plugins installed. These plugins facilitate the tracking of token balances, offer structured data storage solutions, and enable the processing of JSON-RPC requests.

## Configuration

Prior to employing the `geteep17balances` method, specific adjustments must be made in the `TokensTracker` configuration (`config.json`):

- **MaxResults**: This determines the maximum number of records the tracker will store. Records exceeding this cap will not be captured.
- **Network**: Ensure this is aligned with the `Network` setting in your EpicChain-CLI `config.json` to maintain consistency across your network operations.

## Parameters

- **address**: This parameter should be the wallet address for which you wish to query the EEP17 token balance.

## Example Usage

### Request Body

To perform a balance query for an EEP17 asset, your JSON-RPC request body should look something like the following:

```json
{
  "jsonrpc": "2.0",
  "method": "geteep17balances",
  "params": ["NgaiKFjurmNmiRzDRQGs44yzByXuSkdGPF"],
  "id": 1
}
```

This request specifies:
- `method`: The action to perform, here `"geteep17balances"`.
- `params`: Contains the address for which the balance inquiry is made.
- `id`: A unique identifier for matching the request with its response.

### Response Body

Assuming successful execution, the response might appear as follows:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "balance": [
      {
        "assethash": "0x70e2301955bf1e74cbb31d18c2f96972abadb328",
        "amount": "3000000100000000",
        "lastupdatedblock": 2
      },
      {
        "assethash": "0xf61eebf573ea36593fd43aa150c055ad7906ab83",
        "amount": "99999900",
        "lastupdatedblock": 2
      }
    ],
    "address": "NgaiKFjurmNmiRzDRQGs44yzByXuSkdGPF"
  }
}
```

## Key Points

- **Balances**: Displayed under the `balance` array, providing details like the `assethash` of each EEP17 token, the `amount` held, and the `lastupdatedblock`, indicating the block at which the balance was last updated.
- **Synchronization**: Ensure your client is fully synchronized with the blockchain to the point of the contract's deployment. Otherwise, the method may return an error or outdated information.
- **Non-EEP17 Contracts**: Attempting to query a non-EEP17 smart contract will result in an error.

This method is crucial for developers creating wallet interfaces, financial tracking applications, or any service requiring up-to-date information on EEP17 token balances. By ensuring proper node configuration and synchronization, developers can reliably integrate this functionality into their applications.










<br/>