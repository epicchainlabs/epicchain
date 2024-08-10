---
title: Send From
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







The `sendfrom` method is critical for executing asset transfers between addresses within the blockchain environment. This method specifically allows for transferring assets from a given address to another destination address.

## Prerequisites

Before invoking the `sendfrom` method:

- **RpcServer Plugin**: The RpcServer plugin must be installed on your blockchain node. This plugin enables the node to process JSON-RPC requests.
  
- **Open Wallet**: You need to open the wallet through the `openwallet` RPC method to access the assets and permissions necessary for sending transactions.

## Parameters Description

- **asset_id**: The asset identifier, or the script hash of the NEP-17 contract representing the asset you intend to transfer.
  
- **from**: The address from which the assets are to be transferred. It is essential that this address holds sufficient balance of the specified asset and is accessible (unlocked) by the open wallet.
  
- **to**: The destination address to which the assets are being sent.
  
- **value**: The amount of the asset to transfer. Depending on the asset's divisibility, this could require conversion into its smallest unit.

- **signers**: A list of accounts authorizing the transaction. At a minimum, this includes the account initiating the transfer.

## Example Usage

### Request Body

A sample JSON-RPC request to transfer assets from one address to another looks like this:

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "sendfrom",
    "params": [
        "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
        "NgaiKFjurmNmiRzDRQGs44yzByXuSkdGPF",
        "NikhQp1aAD1YFCiwknhM5LQQebj4464bCJ",
        100000000
    ]
}
```

### Response Body

The response for a successful transaction will contain detailed information about the conducted transaction:

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "hash": "0xe01b16626dec583941c1053467100041ce868e3b35e5fe3a85e530792cc9149d",
        ...
        "script": "CwIA4fUF...",
        "witnesses": [
            {
                "invocation": "DEAUQ3hUPg/qi77rnSzXRgd2RYdZCsPDBa/n0a6M+sCsOpC/YyLPeeoqcVNAyh73qpocOqdX1tnGeizh+C8cXoK0",
                "verification": "EQwhAs7UMjl93ETtugMcC8O5M/KP3ZZ3eS17IObANt2qrPHiEQtBE43vrw=="
            }
        ]
    }
}
```

### Response Description

- **hash**: The unique identifier of the transaction.
  
- **sender**: The address that initiated the transaction.

- **sysfee** & **netfee**: System and network fees associated with the transaction.

- **signers**: Details the accounts that have signed the transaction, including their scopes.

- **script**: The executed script of the transaction, which follows the contract's requirements.

- **witnesses**: Contains the invocation and verification scripts necessary for the transaction's execution.

## Considerations

- **Asset ID**: Ensure the asset ID is correctly specified, as errors can lead to transaction failure.

- **Wallet Authentication**: The wallet must be unlocked and contain the private keys for the `from` address to initiate the transaction.

- **Sufficient Balance**: Verify the `from` address has enough balance of the specified asset to cover the transfer amount and transaction fees.

- **Security Practices**: Given the method involves transferring assets, ensure all security protocols are adhered to, including secure handling of private keys and wallet information.

`sendfrom` provides a streamlined approach to asset transfer within the blockchain, enabling efficient and secure transactions between addresses by leveraging the platform's underlying smart contract functionalities.




<br/>