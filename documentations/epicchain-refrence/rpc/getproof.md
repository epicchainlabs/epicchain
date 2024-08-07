---
title: Get Proof
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







The `getproof` method serves an important function in blockchain networks by fetching proof associated with a given state root, contract hash, and a specific key within the contract's storage. This type of proof, often utilized in light client protocols and cross-chain verification, demonstrates the existence or absence of particular data without requiring the full blockchain data.

## Prerequisites

To invoke `getproof`, ensure the following setup:

1. **RpcServer Plugin**: This plugin enables the blockchain node to handle JSON-RPC requests, facilitating communication between clients and the blockchain network.

2. **StateService Plugin**: Essential for managing and retrieving state data. This service provides the functionality needed for `getproof` operations.

Additionally, several configurations in the `StateService` plugin's `config.json` file are essential:

- **FullState**: Must be set to `true`. This configuration ensures that the node maintains the full state history required to generate proofs.

- **Network**: Must match the network setting in the `EpicChain-cli` `config.json` to ensure consistent network identification.

- **AutoVerify**: Optional, determines whether to enable automatic verification of proofs. If enabled, the default active wallet is used for verification purposes.

## Example Usage

### Request Body

The request to fetch a proof requires three parameters: the root hash (`roothash`), the contract's script hash (`scripthash`), and the storage key of interest (`key`). Here is an example:

```json
{
  "jsonrpc": "2.0",
  "method": "getproof",
  "params": ["0x7bf925dbd33af0e00d392b92313da59369ed86c82494d0e02040b24faac0a3ca","0x79bcd398505eb779df6e67e4be6c14cded08e2f2","Fw=="],
  "id": 1
}
```

These parameters are detailed as follows:

- `roothash`: Root hash of the state root.
- `scripthash`: Contract script hash.
- `key`: Storage key; must be Base64-encoded.

### Response Body

The response includes the proof as a string within the `result` field:

```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": "Bfv///8XBiQBAQ8D..."
}
```

The `result` is a string representation of the proof generated from the specified root hash, contract hash, and storage key.

## Considerations

Use of the `getproof` method is particularly beneficial in applications involving:

- **Light clients**: Clients that do not maintain the full blockchain state can verify certain information using the proof.
  
- **Cross-chain interactions**: Proofs allow a contract or client on one chain to verify data from another chain without needing the entire chain data.

## Error Handling

- If `FullState` is not set to `true`, an error message "Old state not supported" will be returned.
  
- It's critical to ensure that the node is fully synchronized with the blockchain and that the `StateService` and `RpcServer` plugins are correctly installed and configured.

The `getproof` method is a powerful tool in blockchain technology, enabling efficient and secure verification of data for decentralized applications, especially in environments with limited resources or cross-chain contexts.









<br/>