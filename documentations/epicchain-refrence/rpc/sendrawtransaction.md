---
title: Send Raw Transaction
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







The `sendrawtransaction` method plays a pivotal role in the operation of the EpicChain network, enabling the broadcasting of signed transactions. This method sends transactions that have already been signed and serialized into the network for execution and confirmation.

## Prerequisites

- **RpcServer Plugin**: Must be installed on your blockchain node. This plugin is necessary for handling JSON-RPC requests and enabling the broadcasting of raw transactions.

## Parameters Description

- **transaction**: The signed transaction, serialized and encoded as a Base64 string. This parameter is critical, as it contains all the required information for the transaction to be validated and executed by the network.

## Example Usage

### Request Body

To broadcast a signed transaction, structure your JSON-RPC request as follows:

```json
{
  "jsonrpc": "2.0",
  "method": "sendrawtransaction",
  "params": ["ALmNfAb4lqIAAAAAAAZREgAAAAAA8S8AAAEKo4e1Ppa3mJpjFDGgVt0fQKBC9gEAKQwFd29ybGQRwAwDcHV0DBR9rbALvBGpMrl7cXVBdSsPOC0EmUFifVtSAUIMQACXF48H1VRmI50ievPfC042rJgj7ZQ3Y4ff27abOpeclh+6KpsL6gWfZTAUyFOwdjkA7CWLM3HsovQeDQlI0oopDCEDzqPi+B8a+TUi0p7eTySh8L7erXKTOR0ziA9Uddl4eMkLQZVEDXg="],
  "id": 1
}
```

### Successful Response Body

Upon successful transmission, the response will confirm the broadcast with the transaction hash:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "hash": "0x13ccdb9f7eda95a24aa5a4841b24fed957fe7f1b944996cbc2e92a4fa4f1fa73"
  }
}
```

### Unsuccessful Response Body

If the broadcast is unsuccessful, an error response is returned detailing the issue:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -500,
    "message": "AlreadyExists"
  }
}
```

## Response Description

- A successful response indicates that the transaction has been successfully broadcasted to the network, with the `hash` value signifying the transaction's unique identifier.
  
- An unsuccessful response signifies a failure in broadcasting the transaction, with specific error codes and messages provided for troubleshooting. Common issues include double spending (`AlreadyExists`), insufficient funds, or invalid transaction formatting.

## Common Error Codes

- **500 (AlreadyExists)**: Indicates the transaction or block already exists within the network.
  
- **OutOfMemory**: The memory pool is full, preventing new transactions from being sent.
  
- **UnableToVerify**: The transaction or block cannot be validated.
  
- **Invalid**: The transaction format or parameter is incorrect.
  
- **Expired**: The transaction refers to expired block information.
  
- **InsufficientFunds**: The sender doesn't have enough funds to cover the transaction.
  
- **PolicyFail**: The transaction violates network policies.

## Considerations

- Ensure the transaction is correctly signed and serialized before encoding to Base64 for successful broadcasting.
  
- Confirm the network's policy and the sufficiency of account funds to avoid common errors such as `PolicyFail` or `InsufficientFunds`.

The `sendrawtransaction` method is essential for interacting with the EpicChain network, allowing users and applications to perform transactions such as transfers, contract executions, and other blockchain operations.










<br/>