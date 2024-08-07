---
title: Get EEP17 Transfer
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







The `geteep17transfers` method fetches all EEP17 (analogous to ERC-20 in Ethereum) transaction activities for a specified address, including both sent and received transactions. This method is crucial for tracking token movements and auditing purposes in blockchain applications that support EEP17 tokens.

## Prerequisites

To utilize the `geteep17transfers` method, ensure your blockchain node is equipped with the necessary plugins: `TokensTracker`, `LevelDBStore`, and `RpcServer`. These plugins enable token tracking, data storage, and handling JSON-RPC requests, respectively.

## Configuration

Before invoking this method, make adjustments in the `TokensTracker` config.json file:

- **MaxResults**: Limits the number of transaction records stored. Exceeding transactions will not be recorded.
- **Network**: This should match the `Network` value from the Neo-CLI `config.json` to ensure synchronization with the correct blockchain network.

## Parameters

- **address**: Target address for querying EEP17 transactions.
- **startTime | endTime**: Optionally define a time range (UTC timestamp) for filtering transactions. If both are specified, only transactions within the range are returned. If one is specified, transactions from that time until now are considered. Without these parameters, the most recent seven days of transactions are returned by default.

## Example Usage

### Request 1: Set the Start Time
To get all transactions since a specific beginning time:

```json
{
  "jsonrpc": "2.0",
  "method": "geteep17transfers",
  "params": ["NikhQp1aAD1YFCiwknhM5LQQebj4464bCJ", 0],
  "id": 1
}
```

### Request 2: Set Both Start and End Times
To filter transactions within a specific time frame:

```json
{
  "jsonrpc": "2.0",
  "method": "geteep17transfers",
  "params": ["NikhQp1aAD1YFCiwknhM5LQQebj4464bCJ", 1611716619654, 2011716619654],
  "id": 1
}
```

### Response

The response includes arrays of `sent` and `received` transactions with details such as timestamp, asset hash, transfer address, amount, block index, transfer notify index, and transaction hash:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "sent": [],
    "received": [
      {
        "timestamp": 1612690497725,
        "assethash": "0xf61eebf573ea36593fd43aa150c055ad7906ab83",
        "transferaddress": "NgaiKFjurmNmiRzDRQGs44yzByXuSkdGPF",
        "amount": "100",
        "blockindex": 2,
        "transfernotifyindex": 1,
        "txhash": "0x5f957960a782514d6587c445288ee1cca7d6b0f952edc204f14d9be83b8152ff"
      },
      ...
    ],
    "address": "NikhQp1aAD1YFCiwknhM5LQQebj4464bCJ"
  }
}
```

## Key Considerations

- Ensure your client is fully synchronized to the blockchain to avoid errors or missing information.
- Transactions concerning non-EEP17 smart contracts will result in errors.
- This method is vital for applications requiring historical transaction data or real-time monitoring of EEP17 token flows for auditing, tracking, and user interface functionalities.









<br/>