---
title: Get EEP11 Transfer
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';






The `geteep11transfers` method is designed to return all EEP-11 (a token standard for NFTs) transaction information that has occurred for a specified address. This could involve both sending and receiving transactions of EEP-11 tokens. It's instrumental for tracking NFT activities associated with a particular wallet address on a blockchain that supports the EEP-11 standard.

## Prerequisites

Before you can utilize the `geteep11transfers` method, it's essential that your blockchain node is equipped with the `TokensTracker`, `LevelDBStore`, and `RpcServer` plugins. These components work together to track token transactions, manage data storage efficiently, and handle incoming JSON-RPC requests.

## Configuration

To ensure the `TokensTracker` plugin operates effectively, certain settings within its `config.json` file need to be adjusted:

- **MaxResults**: Defines the cap on the number of records to be stored. Records exceeding this limit will not be captured.
- **Network**: This setting should match the Network value found in your Neo-CLI `config.json`, aligning the plugin's operation to the correct blockchain network.

## Parameters

- **address**: The wallet address for which you want to query EEP-11 transaction information.
- **startTime | endTime** (optional): UTC timestamps to filter the transactions based on their occurrence time. Specifying both defines a range, while using one sets a starting or ending point. If omitted, transactions from the most recent seven days are returned.

## Example Usage

### Request Body

```json
{
  "jsonrpc": "2.0",
  "method": "geteep11transfers",
  "params": ["NdUL5oDPD159KeFpD5A9zw5xNF1xLX6nLT", 1635146038919],
  "id": 1
}
```
This request:

- Invokes the `"geteep11transfers"` method to query transactions.
- Supplies an address and an optional timestamp as parameters.
- Utilizes a unique ID to correlate the request with its response.

### Response Body

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "address": "NdUL5oDPD159KeFpD5A9zw5xNF1xLX6nLT",
    "sent": [ ... ],
    "received": [ ... ]
  }
}
```
The `result` contains:

- **address**: Reiterates the queried address.
- **sent**: An array of objects, each representing a sent EEP-11 token transaction. Includes details such as timestamp, asset hash, transfer address, amount, block index, transfer notify index, transaction hash, and the token ID.
- **received**: Similar to `sent`, but for received transactions.

## Important Considerations

- Both `sent` and `received` transactions include a **timestamp**, indicating when the transaction occurred, and a **txhash**, uniquely identifying the transaction on the blockchain.
- **tokenid** denotes the unique identifier of the transferred EEP-11 token.
- **assethash** refers to the contract hash of the EEP-11 token.

This method is pivotal for applications and services needing detailed insights into EEP-11 token transactions for an address, empowering users with comprehensive historical data of their NFT interactions.










<br/>