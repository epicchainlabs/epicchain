---
title: Submit Block
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








The `submitblock` method is integral to the EpicChain network's operations, enabling nodes to broadcast new blocks. This feature plays a crucial role in maintaining the network's integrity and ensuring its continuous, decentralized function.

## Prerequisites

- **RpcServer Plugin**: Installation of the RpcServer plugin is required on the blockchain node. This plugin facilitates the node's capability to process JSON-RPC requests, including the submission of new blocks.

## Parameter Description

- **hex**: A Base64-encoded string that represents a fully serialized block. This encapsulation ensures that the block, with all its transactions and header information, is correctly understood by the network upon submission.

## Example Usage

### Request Body

To submit a new block to the EpicChain network, format your JSON-RPC request as follows:

```json
{
  "jsonrpc": "2.0",
  "method": "submitblock",
  "params": ["Base64_encoded_serialized_block_data"],
  "id": 1
}
```

Replace `Base64_encoded_serialized_block_data` with the actual Base64-encoded string of your serialized block.

### Successful Response Body

Upon successful submission, the response will indicate acceptance with the new block's hash:

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "hash": "0xbe153a2ef9e9160906f7054ed8f676aa223a826c4ae662ce0fb3f09d38b093c1"
    }
}
```

### Unsuccessful Response Body

If the block fails to be accepted by the network, the response will include an error code and message detailing the rejection reason:

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

- **Success**: A successful submission is acknowledged by returning the hash of the block added to the blockchain.
  
- **Failure**: Failures to broadcast are characterized by an error code and message. Common reasons include:
  - **AlreadyExists**: The block or its transactions already exist on the chain.
  - **OutOfMemory**: The node's memory pool is full, preventing new submissions.
  - **UnableToVerify**: The block fails validation checks.
  - **Invalid**: Errors in the block's format or parameters.
  - **Expired**: The block is based on outdated blockchain state information.
  - **InsufficientFunds**: Transactions within the block cannot be completed due to insufficient funds.
  - **PolicyFail**: The block includes elements that violate network policies.

## Considerations

- **Block Validation**: Prior to submission, ensure the block is valid according to the EpicChain network's consensus rules.
  
- **Security Measures**: Adhere to security best practices when handling block submissions to prevent malicious activities and protect network integrity.

- **Network Policies**: Be aware of current EpicChain network policies to prevent submission rejections based on policy violations.

Submitting a block to the EpicChain network is a critical action, often restricted to nodes participating directly in the blockchain's consensus mechanism. This process supports the decentralized, secure, and trustless nature of blockchain technology.









<br/>