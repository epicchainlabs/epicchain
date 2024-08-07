---
title: Migration Destruction
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';






# Contract Update and Destroy

Smart contracts often need to be updated or destroyed after deployment. This guide explains how to implement these features in your NEO smart contracts.

## Contract Update

Updating a smart contract allows you to upgrade the contract's functionality or migrate its storage to a new contract without changing the contract hash or storage. To enable contract updates, you must implement the Update interface in your contract.

### Implementing the Update Interface

The Update method should be implemented in your contract. In the contract template, this method is already implemented as follows:

```csharp
public static bool Verify() => IsOwner();

public static void Update(ByteString nefFile, string manifest)
{
    if (!IsOwner()) throw new Exception("No authorization.");
    ContractManagement.Update(nefFile, manifest, null);
}
```

To update the contract later, you must implement the Update method before deployment. For detailed instructions on deploying and invoking smart contracts, refer to the NEO documentation.

#### Updating the Contract

To update a contract, follow these steps:

1. Prepare the compiled NEF file and Manifest file of the new contract.
2. Base64-encode the NEF file.
3. Compress and escape the Manifest file.
4. Use the invoke command to call the Update method of the contract.

After the Update method is executed, the contract is upgraded, and neither the contract hash nor the storage area is changed.

### Contract Destruction

To destroy a contract, you must implement the Destroy method in your contract.

#### Implementing the Destroy Method

The Destroy method should be implemented in your contract. In the contract template, this method is already implemented as follows:

```csharp
public static bool Verify() => IsOwner();

public static void Destroy()
{
    if (!IsOwner()) throw new Exception("No authorization.");
    ContractManagement.Destroy();
}
```

When the Destroy method is invoked, the contract and its storage (if any) are deleted, making the contract no longer available.





















<br/>