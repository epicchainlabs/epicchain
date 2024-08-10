---
title: Find States
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';



The `findstates` method is an advanced functionality provided by the EpicChain blockchain, enabling users to query specific states based on the root hash, contract hash, and storage key prefix. This method necessitates the installation of both the `StateService` and `RpcServer` plugins to be utilized effectively.

## Parameters

- **roothash**: The state root's hash at a particular block height. This parameter is essential for defining the blockchain's state at a specific moment.
- **scripthash**: The smart contract's hash for which the state is being queried. This parameter specifies the contract whose storage keys and values are of interest.
- **prefix**: The prefix of the storage key, provided in Base64-encoded format. This allows for filtering states of interest.
- **key** (Optional): If specified, returns the result starting from the provided Base64-encoded key.
- **count** (Optional): Determines the number of items returned. Useful for limiting the results in queries expected to have numerous matches.

## Example Usage

### Request Body:

```json
{
  "jsonrpc": "2.0",
  "method": "findstates",
  "params": [
    "0xec31cdb14da4143e2ab471a8b5812d895b88fc1c12d54e112791491feca9b5f4",
    "0xb1fbb6b0096919071769906bb23b2ca2ec51eea7",
    "AQE="
  ],
  "id": 1
}
```

In this request, we query the states associated with a particular root hash (`roothash`), contract hash (`scripthash`), and a specific prefix (`prefix`), which is Base64-encoded.

### Response Body:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "firstProof": "...",
    "lastProof": "...",
    "truncated": false,
    "results": [
      {
        "key": "AQFM8QSIkBuHVYOd2kiRmQXXOI833w==",
        "value": "nMJ4AQ=="
      },
      {
        "key": "AQFmmdXxSKUGNq3wQcppLAdAe8sD+g==",
        "value": "yGJI4+cA"
      },
      {
        "key": "AQHu8BYlbUqc54Mtr+S7+i0Aj5J/Lg==",
        "value": "nOrj7wA="
      }
    ]
  }
}
```

The response includes:
- **firstProof** and **lastProof**: These are cryptographic proofs for the first and last state entries matching the query, useful for certain types of audit or verification operations.
- **truncated**: Indicates whether the response has been shortened due to exceeding a limit on the number of items that can be returned in a single response.
- **results**: An array of objects, each representing a key-value pair matching the query criteria. The keys and values are Base64-encoded.

## Key Insights

- The `findstates` method is a powerful tool for interacting with smart contracts on the EpicChain blockchain, providing deep insights into the contract's storage state at various block heights.
- The method's flexibility in querying based on key prefixes allows for efficient and targeted retrieval of state information.
- Proper understanding and usage of the `findstates` method can significantly enhance blockchain analytics, auditing, and smart contract management tasks.

**Note**: Handling of Base64 encoding/decoding and understanding the structure and purpose of the queried smart contract's storage are essential for effective use of this method.


<br/>