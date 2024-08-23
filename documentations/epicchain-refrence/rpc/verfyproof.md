---
title: Verfy Proof
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








The `verifyproof` method is designed to validate the integrity and authenticity of a specific piece of data within the EpicChain state, utilizing a cryptographic proof. This method is vital for light clients and applications requiring verification of data without the need to sync the entire blockchain.

## Prerequisites

- **StateService and RpcServer Plugins**: Both plugins must be installed on your blockchain node. The StateService plugin generates and manages state roots and proofs, whereas the RpcServer plugin allows the node to handle JSON-RPC requests.

## Parameters Description

- **roothash**: The root hash of the state root. This hash represents the entire state at a certain blockchain height, against which the proof is verified.

- **proof**: The proof data of the state root. This is a Base64-encoded string that cryptographically proves the existence (or absence) of a specific piece of state data in reference to the roothash.

## Example Usage

### Request Body

To verify a piece of data within the EpicChain state, structure your JSON-RPC request as shown below:

```json
{
  "jsonrpc": "2.0",
  "method": "verifyproof",
  "params": [
    "0x7bf925dbd33af0e00d392b92313da59369ed86c82494d0e02040b24faac0a3ca",
    "Base64_encoded_proof_data"
  ],
  "id": 1
}
```

Replace `"Base64_encoded_proof_data"` with your actual proof data encoded in Base64 format.

### Response Body

The response provides the result of the verification process. If successful, it returns the value of the storage item corresponding to the key used in generating the proof:

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": "Base64_encoded_value"
}
```

The `"Base64_encoded_value"` is also Base64-encoded and needs decoding to obtain the original value.

## Understanding the Response

- **Success**: A successful response indicates the proof is valid, and the state contains the data as claimed. The `result` contains the verified data corresponding to the key involved in the proof.

- **Failure**: If the proof is invalid or does not correspond to the given root hash, the method might return an error indicating the verification failed. This ensures that only authentic and accurate data is acknowledged.

## Key Takeaway

The `verifyproof` method is indispensable for applications and services requiring confirmation of the presence or contents of specific data within the EpicChain state without maintaining a full node. This functionality enhances the scalability and accessibility of blockchain data verification, empowering light clients and decentralized applications with the means to ensure data integrity efficiently.









<br/>