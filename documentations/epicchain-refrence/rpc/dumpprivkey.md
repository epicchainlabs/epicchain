---
title: Dump Privkey
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




The `dumpprivkey` method is a crucial security tool within the EpicChain ecosystem, offering users the capability to export the private key associated with a specific address. This functionality is key for backing up or transferring wallet data securely.

## Prerequisites

To successfully invoke the `dumpprivkey` method, ensure that:

- **RpcServer Plugin**: The `RpcServer` plugin must be installed on your EpicChain node. This plugin facilitates the handling of JSON-RPC requests, including sensitive operations such as exporting private keys.

- **Open Wallet**: You must first use the `openwallet` RPC method to open the wallet. The `dumpprivkey` method requires an opened wallet context to access the private keys securely stored within.

## Parameter Description

- **address**: This parameter requires the input of a standard EpicChain address from which you intend to export the private key. 

## Example

Here's a step-by-step guide to crafting a request for exporting a private key and interpreting the response.

### Request Body:

```json
{
  "jsonrpc": "2.0",
  "method": "dumpprivkey",
  "params": ["NepVckSSgHJf1szQ6LEibd5NU7Ap67yJrJ"],
  "id": 1
}
```

In this request:
- The `method` field specifies the action to be performed (`dumpprivkey`).
- The `params` field holds an array containing the address whose private key you wish to export.
- The `id` field assists in associating responses with their corresponding requests, particularly useful in asynchronous environments.

### Response Body:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "L5LEfSAAbVAk5vxmkBpWQqJ2e1hyh3nEqgWaosB35XpBAkZdizj4"
}
```

The response includes:
- The `result` field, which contains the exported private key in WIF (Wallet Import Format). This key must be handled with extreme caution, as anyone with access to it can control the associated address and its funds.

## Response Description:

- The method returns the private key associated with the provided standard address, allowing for secure backup or transfer of wallet holdings.

## Security Considerations

- **Sensitive Information**: Private keys are highly sensitive. Leakage or unauthorized access can result in the loss of assets. Therefore, it's crucial to handle and store private keys securely.

- **Secure Channels**: When exporting and transmitting private keys, always use secure, encrypted channels to prevent interception by malicious actors.

- **Backup Strategies**: Regularly backing up your private keys is a good practice. However, these backups must be stored securely, such as in encrypted storage or offline mediums.

The `dumpprivkey` method is an essential tool for managing wallet security and portability within the EpicChain ecosystem. Proper use and handling of exported private keys are vital to maintaining control over your blockchain assets.


<br/>