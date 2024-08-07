---
title: Get Next Block Validators
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';






The `getnextblockvalidators` method is a part of the JSON-RPC interface that allows users to retrieve the list of validator candidates for the subsequent block in the blockchain. Validators play a critical role in the consensus mechanism of many blockchains, validating transactions, and blocks to ensure the integrity and security of the network.

## Prerequisites

Before being able to invoke the `getnextblockvalidators` method, the blockchain node must have the `RpcServer` plugin installed. This plugin enables the node to handle JSON-RPC requests, facilitating interactions between the client and the blockchain.

## Example Usage

### Request for Getting Next Block Validators

To request the list of validators for the next block, the following JSON-RPC call format is used:

```json
{
  "jsonrpc": "2.0",
  "method": "getnextblockvalidators",
  "params": [],
  "id": 1
}
```

In this request:
- `method`: Specifies the action to be performed, here `"getnextblockvalidators"`.
- `params`: This method does not require any parameters, so an empty array is passed.
- `id`: A unique identifier for the request, useful for correlating the response with the request.

### Response with Validators List

The response to a successful call will contain an array of validator objects within the `result` field:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    {
      "publickey": "03aa052fbcb8e5b33a4eefd662536f8684641f04109f1d5e69cdda6f084890286a",
      "votes": "0",
      "active": true
    }
  ]
}
```

Each object in the `result` array represents a validator with the following fields:
- **publickey**: The public key of the validator candidate. This key helps in identifying and verifying the validator.
- **votes**: The number of votes this validator has received. The default value is "0", indicating that voting has not started or no votes have been received yet.
- **active**: A boolean value indicating whether the validator is currently active or not.

## Considerations and Use Cases

- This method is particularly useful in Proof of Stake (PoS) or Delegated Proof of Stake (DPoS) blockchains, where validators are elected or nominated based on various criteria, including the number of votes from token holders.
- Understanding the validator selection for the next block can be crucial for developers and stakeholders interested in the governance and consensus processes of the blockchain.
- It also assists in monitoring the health and decentralization of the network by keeping track of active validators and their distribution.

## Note

- The list and status of validators can change based on the blockchain’s consensus rules, voting outcomes, and specific blockchain implementations.
- Validators typically have a significant responsibility in maintaining network integrity, and their roles may include proposing blocks, validating transactions, and participating in consensus decisions.

Integrating and monitoring validator status using the `getnextblockvalidators` method offers insights into the consensus layer of the blockchain, enabling deeper engagement with the network's governance and operational dynamics.










<br/>