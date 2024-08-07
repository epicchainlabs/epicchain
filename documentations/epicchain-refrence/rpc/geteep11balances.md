---
title: Get EEP11 Balances
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







The `geteep11balances` method is designed to fetch the balance of all EEP11 (a token standard similar to ERC-721 for NFTs) assets owned by a specified address. This method is invaluable for applications and services that need to display a user's NFT holdings on a blockchain that adopts the EEP11 standard.

## Prerequisites

To use the `geteep11balances` method, you must have the `TokensTracker`, `LevelDBStore`, and `RpcServer` plugins installed on your node. These plugins work together to track token balances, provide database storage capabilities, and handle JSON-RPC requests, respectively.

## Configuration

Before invoking the method, specific configuration adjustments are required in the `TokensTracker` plugin's `config.json` file:

- **MaxResults**: Defines the maximum number of records to be stored. Records exceeding this limit will not be stored.
- **Network**: This should be set to match the network identifier value from the Neo-CLI `config.json`, ensuring that the plugin operates on the correct blockchain network.

## Example Usage

### Request Body:

```json
{
  "jsonrpc": "2.0",
  "method": "geteep11balances",
  "params": ["NdUL5oDPD159KeFpD5A9zw5xNF1xLX6nLT"],
  "id": 1
}
```

In this request:
- `method`: Specifies that you're calling the `"geteep11balances"` method.
- `params`: Contains an array with the single element being the address you're querying.
- `id`: A unique identifier for the request which is useful for matching responses with requests.

### Response Body:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "address": "NdUL5oDPD159KeFpD5A9zw5xNF1xLX6nLT",
    "balance": [
      {
        "assethash": "0xb3b65e5c0d2af3f98cac6e80083f6c2b90476f40",
        "tokens": [
          {
            "tokenid": "Blind Box 2",
            "amount": "1",
            "lastupdatedblock": 36653
          },
          {
            "tokenid": "Blind Box 3",
            "amount": "1",
            "lastupdatedblock": 37100
          },
          {
            "tokenid": "Blind Box 1265",
            "amount": "1",
            "lastupdatedblock": 501483
          }
        ]
      }
    ]
  }
}
```

## Important Considerations

- **Blockchain Synchronization**: Your client must be synchronized to the block in which the contract was deployed to retrieve accurate values. Otherwise, the API may return an error or outdated information.
- **Non-EEP11 Contracts**: Attempting to query the balance for a script hash corresponding to a non-EEP11 contract will result in an error.
- **Up-to-date Information**: Ensure your client is fully synchronized with the latest block height before making a query. Failing to do so may lead to receiving balances that are not current.

The `geteep11balances` method is an essential tool for developers building applications on top of blockchain platforms that need to interact with or display NFT assets conforming to the EEP11 standard. Proper synchronization with the blockchain and configuration of the necessary plugins and settings is crucial for making successful queries with this method.









<br/>