---
title: API Refrence
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




The EpicChain blockchain offers an extensive API interface through JSON-RPC, facilitating easy development of blockchain applications by providing access to blockchain data. This guide covers the essential aspects of interacting with EpicChain's JSON-RPC API, including setup instructions, command lists, and request examples.

## Setting Up EpicChain JSON-RPC Service

### Install RpcServer Plugin

To enable the RPC service on your EpicChain-CLI node, you must first install the `RpcServer` plugin. Follow the plugin installation instructions provided in the official documentation. No additional arguments are needed when starting the node with `epicchain-CLI`.

### Compile RpcServer Plugin

If the required version of the `RpcServer` plugin is unavailable, you may need to compile the `epicchain-modules` project manually:

1. Create a `Plugins` folder in the directory where `epicchain-cli.dll` is located.
2. Place the compiled `RpcServer` file in the `Plugins` folder.
3. Restart `epicchain-CLI`.

### Listening Ports

By default, the JSON-RPC server listens on local address port 10112 (`http://127.0.0.1:10112/`). This can be adjusted in the `config.json` file within the `RpcServer` folder.

## EpicChain JSON-RPC Command Lists

EpicChain's JSON-RPC API is divided into several categories, including blockchain data, node information, smart contracts, tools, wallet operations, and specific plugin functions. Below is an overview of some critical RPC methods available:

### Blockchain
- `getbestblockhash`: Fetches the hash of the most recent block.
- `getblock`: Returns block information by hash or index.
- `getblockcount`: Retrieves the total number of blocks in the blockchain.
- `getcontractstate`: Provides details of a contract with a given script hash.

### Node
- `getconnectioncount`: Shows the number of connections to the node.
- `getversion`: Retrieves node version information.

### Smart Contract
- `invokefunction`: Invokes a contract method with parameters.
- `invokescript`: Executes a script on the virtual machine.

### Wallet
- `getwalletbalance`: Shows the balance of a specific asset in the wallet.
- `sendtoaddress`: Sends assets to a specified address.
- `openwallet`: Opens a specified wallet file.

### ApplicationLogs Plugin
- `getapplicationlog`: Returns contract event information based on a transaction ID.

### StateService Plugin
- `getproof`, `verifyproof`: Functions related to validating state proofs.

## Getting Started with JSON-RPC Requests

### GET Request Example
```plaintext
http://127.0.0.1:10112?jsonrpc=2.0&method=getblockcount&params=[]&id=1
```
This GET request fetches the current block count from the blockchain.

### POST Request Example
```json
{
  "jsonrpc": "2.0",
  "method": "getblockcount",
  "params": [],
  "id": 1
}
```
Send this POST request to `http://127.0.0.1:10112` to obtain the number of blocks in the blockchain.

## Note on Amounts and Decimals
All amount-related return values (e.g., fees, asset balances) are unsigned integers and must be manually converted to account for asset decimals. For instance, if the return value is `1234560` and the asset decimal is `8`, the actual amount is `0.0123456`.

## Testing Tools
For testing purposes, tools like Postman or similar REST client applications can facilitate interaction with the JSON-RPC API. These tools make it easier to construct, send, and analyze API requests and responses.

By utilizing EpicChain's JSON-RPC API, developers can harness the full potential of the EpicChain blockchain for application development, ranging from querying blockchain data to initiating smart contract interactions and managing wallet operations.


<br/>