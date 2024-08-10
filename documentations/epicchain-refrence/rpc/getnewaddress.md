---
title: Get New Address
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';






The `getnewaddress` method is part of a JSON-RPC interface that enables the creation of a new blockchain address within a wallet. This method is commonly used in scenarios requiring the generation of new wallet addresses for transactional purposes or wallet organization.

## Prerequisites

- **RpcServer Plugin**: Before invoking the `getnewaddress` method, your blockchain node must have the `RpcServer` plugin installed. This plugin allows your node to process JSON-RPC requests.
- **Open Wallet**: The wallet containing the address must be opened using the `openwallet` RPC method prior to calling `getnewaddress`. This step ensures the node has access to the wallet where the new address will be created.

## Example Usage

### Opening a Wallet

Before creating a new address, you must ensure the wallet is open. Here's how you might call the `openwallet` method:

```json
{
  "jsonrpc": "2.0",
  "method": "openwallet",
  "params": ["path/to/wallet/file", "walletPassword"],
  "id": 0
}
```
After successfully opening the wallet, you can proceed to create a new address.

### Request to Create a New Address

```json
{
  "jsonrpc": "2.0",
  "method": "getnewaddress",
  "params": [],
  "id": 1
}
```

In this request:
- `method` specifies the action to be performed, here `"getnewaddress"`.
- `params` is empty because creating a new address does not require any parameters.
- `id` is a unique identifier for the request. It helps in matching the response with the request.

### Response

The response to a successful `getnewaddress` request will include the newly created address in the `result` field:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": "NeMDdPDC29BfayYF7xNvnCSfCq9Drh1xKX"
}
```

Response elements:
- `result` contains the newly generated address as a string. This address can now be used for receiving transactions.

## Scenario Considerations

The `getnewaddress` method is particularly useful in applications or services that require fresh addresses for each transaction or user to enhance privacy and security. Automated systems, such as cryptocurrency exchanges or payment gateways, often utilize such functionality to generate deposit addresses for their users.

## Important Notes

- Ensure that the wallet is securely opened and available before attempting to create a new address.
- Managing newly created addresses and maintaining their security is crucial. Always follow best practices for key management and security.
- Depending on the blockchain platform, the wallet may need to be synchronized with the network for the newly created address to be fully operational.

By following the above steps, developers can seamlessly integrate the functionality to generate new addresses within their blockchain-based applications, facilitating a wide range of transactional operations.










<br/>