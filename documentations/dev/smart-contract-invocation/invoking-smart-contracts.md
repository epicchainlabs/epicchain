---
title: Invoking Smart Contracts
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Invoking Smart Contracts on EpicChain

After deploying a smart contract, the next step is to invoke it. This can be for various purposes such as executing its functions or querying data. The unique identifier known as the script hash is crucial for this process.

## Querying Contract Details

Before invoking a contract, it's often necessary to query its details:

### Using Neo-CLI

- **Method**: Use the `getcontractstate` RPC API method.
- **Command**: Input the contract's script hash.
- **Results**: Displays general information, methods, notifications, etc.

### Using Neo-GUI

- **Navigation**: Click `Contract` -> `Search`.
- **Input**: Enter the contract script hash.
- **Outcome**: Displays the contract's details, including manifest and `.nef` files.


## Invoking a Contract

### Using Neo-CLI

There are two main approaches:

1. **The `invoke` Command**:

    ```shell
    invoke <scriptHash> <operation> [contractParameters=null] [sender=null] [signerAccounts=null] [maxEpicPulse=20]
    ```

2. **RPC API Methods**:

    - Recommended: `invokefunction`
    - Alternative: `invokescript`

### Using Neo-GUI

- **Navigation**: Click `Contract` -> `Invoke Contract`.
- **Input**: Enter the script hash and click `Search`.
- **Execute**: From the displayed methods and parameters, select and fill in the desired operation, then click `Invoke`.

## Cosignature

Certain contracts necessitate multiple signatures for invocation:

- The transaction initiator's signature is often required to cover the fee.
- Runtime.CheckWitness(owner) is commonly used to authenticate the caller's address.
  
  When invoking a `Runtime.CheckWitness(owner)` included contract, ensure to include the owner's signature as a cosignature.

### Adding a Cosignature in Neo-CLI

Use the `invoke` command with complete parameters, including `[signerAccounts]`.

### Adding a Cosignature in Neo-GUI

- **Option**: Click `Cosignature` and choose `Public key`.
- **Execution**: Click `Sign` to add the signature.

## Inter-Contract Invocation

Dynamic invocation between contracts is simpler in Neo N3. Here's an example for a basic contract call:

```csharp
public class Contract1 : SmartContract
{
    [InitialValue("0xbed7d6494ceb31c82630de657cbb1e7fc1254469", ContractParameterType.Hash160)]
    public static UInt160 ScriptHash;
    
    public static object Main(string operation, object[] args)
    {
        if (operation == "name")
        {
            return Contract.Call(ScriptHash, "name", CallFlags.ReadOnly, new object[0]);
        }
        // Additional operations...
    }
}
```

**Key Aspects**:

- **scriptHash**: The target contract's script hash.
- **method**: The method name to invoke.
- **flags**: Special behaviors, see CallFlags Enumerator.
- **params**: Parameters of the invoked contract method.

## Invocation Permissions

The contract manifest defines permissions and trusts to manage invoking behavior and wallet prompts:

- **Groups**: Trusted contracts group.
- **Permissions**: Contracts and methods allowed to be called. 
- **Trusts**: Contracts trusted by this contract, affecting wallet prompts.

Based on these settings, the wallet will guide the user on setting appropriate signature scopes and whether additional permissions are required for the contract invocation.

Adhering to these guidelines ensures smooth and secure smart contract invocation on the EpicChain blockchain, leveraging both CLI and GUI tools effectively.




















<br/>