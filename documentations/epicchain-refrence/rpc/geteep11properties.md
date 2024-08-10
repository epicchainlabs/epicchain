---
title: Get EEP11 Properties
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




The `geteep11properties` method provides a direct way to fetch the properties of a specific EEP-11 (an NFT standard similar to ERC-721) token. This includes retrievals such as the token's name, description, image URL, and token URI - all crucial pieces of information for applications interacting with or displaying NFTs. Uniquely, it essentially automatically decodes properties like name, description, and image URLs from UTF8, streamlining the process of displaying this information to end-users.

## Prerequisites

Before utilizing the `geteep11properties` method, your blockchain node must have the `TokensTracker`, `LevelDBStore`, and `RpcServer` plugins installed. These are necessary to track tokens on the network, provide a database for storing this information, and enable the node to process JSON-RPC requests, respectively.

## Parameters

- **contract**: The smart contract's hash which issued the EEP-11 token. This is the unique identifier for the contract on the blockchain.
- **tokenId**: A hex string representing the token ID within the specified contract.

## Example Usage

### Request Body

To query the properties of an EEP-11 token, the JSON-RPC request format is shown below:

```json
{
  "jsonrpc": "2.0",
  "method": "geteep11properties",
  "params": ["0xd9e2093de3dc2ef7cf5704ceec46ab7fadd48e7f", "452023313032204e6f697a"],
  "id": 1
}
```

In this example:
- `method` signifies the operation to be performed, namely `"geteep11properties"`.
- `params` is an array containing the smart contract hash and the hex string of the token ID.
- `id` is a unique identifier for the request for response tracking.

### Response Body

Assuming the token and contract exist and the query was successful, the response may look something like this:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "name": "E #102 Noiz",
    "owner": "wJjkrPCyCQ3Rbss9WN5CaocVhRs=",
    "number": "Zg==",
    "image": "https://neo.org/Noiz.png",
    "video": null
  }
}
```

The `result` key contains an object with the token's properties:
- **name**: The token's name, automatically decoded from UTF8.
- **owner**: The encoded owner's address.
- **number**: An additional property example, potentially signifying a sequence number or edition, also encoded.
- **image**: A URL to the token's associated image.
- **video**: This field can contain multimedia links related to the token, and if not applicable, it remains `null`.

## Important Considerations

- **Automatic Decoding**: The properties returned by this API are automatically decoded from UTF8, simplifying the integration process for developers who need to display this information directly.
- **Data Accuracy**: It's critical to ensure that your node is fully synchronized and the plugins are correctly configured to retrieve accurate information.
- **Error Handling**: In cases where the contract or token ID does not exist or does not follow the EEP-11 standard, the API might return an error. It's important for applications to gracefully handle these scenarios.

This method is indispensable for applications aiming to interact with, display, or manage EEP-11 NFTs, providing a straightforward means to access token-specific properties efficiently.












<br/>