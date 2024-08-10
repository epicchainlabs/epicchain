---
title: Import Privkey
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








The `importprivkey` method is a critical operation within numerous blockchain ecosystems, allowing users to import a private key in Wallet Import Format (WIF) into their wallet. This method is integral for recovering, migrating, or consolidating blockchain assets across wallets.

## Prerequisites

To successfully employ the `importprivkey` approach, ensure you have:

1. **RpcServer Plugin**: This essential plugin must be installed on your blockchain node. It facilitates the node's ability to handle JSON-RPC requests, including sensitive operations like importing private keys.

2. **Open Wallet**: It's necessary to open the wallet using the `openwallet` RPC method before attempting to import a private key. This operation unlocks the wallet, making it ready to receive the new private key.

## Parameters

- **key**: The private key you intend to import into the wallet, formatted in WIF (Wallet Import Format). The WIF is a standard encoding for a private key, ensuring it's compact and ready for use.

## Example Usage

### Request Body

To import a WIF-formatted private key into your wallet, structure your JSON-RPC request as follows:

```json
{
  "jsonrpc": "2.0",
  "method": "importprivkey",
  "params": ["KwYRSjqmEhK4nPuUZZz1LEUSxvSzSRCv3SVePoe67hjcdPGLRJY5"],
  "id": 1
}
```

### Response Body

Upon successful import, the blockchain node responds with details concerning the imported key:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "address": "NPvKVTGZapmFWABLsyvfreuqn73jCjJtN1",
    "haskey": true,
    "label": null,
    "watchonly": false
  }
}
```

### Response Description

- **address**: The blockchain address corresponding to the imported private key. This address is now authorized to initiate transactions from the wallet.
- **haskey**: A boolean indicating that the imported key is recognized and stored in the wallet.
- **label**: A label associated with the imported key, if any. This field might return null if no label was assigned.
- **watchonly**: Specifies whether the address is treated as "watch only." An address set to `false` (as in the example) means the wallet not only observes but can also control assets associated with that address.

## Considerations

- **Security**: Importing private keys is a sensitive operation. Ensure the RPC interface is securely configured, and sensitive information is not exposed.

- **Client Synchronization**: Make sure your client or node is fully synchronized with the blockchain to ensure that the newly imported key operates seamlessly with the current blockchain state.

- **Backup**: Always maintain a secure backup of your wallet before performing operations like importing private keys to mitigate risks of data loss.

- **Finality**: Once a private key is imported into a wallet, any assets controlled by that key can now be accessed through the wallet. Exercise caution to ensure that private keys are only imported when necessary and under secure conditions.

The `importprivkey` method facilitates the seamless addition of private keys to your blockchain wallet, enabling access and control over associated assets. Proper understanding and careful execution of this method are paramount to ensure the security and integrity of your digital assets.








<br/>