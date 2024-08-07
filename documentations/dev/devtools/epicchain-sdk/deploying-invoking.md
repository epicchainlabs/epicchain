---
title: Deploying and Invoking Contracts
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';





# EpicChain Smart Contract Operations

## Contract Deployment and Invocation

EpicChain's robust ecosystem supports the deployment and invocation of smart contracts through its `RpcClient` module, providing seamless interaction with contracts on the blockchain.

### Preparing for Deployment

To deploy a contract, you need:

- **Contract Scripts and Manifests**: Obtained post-compilation, defining the contract's structure and permissions.
- **Sufficient EpicPulse**: The deploying account must hold enough EpicPulse for system and network fees.

#### Deployment Steps:

1. **Read Contract Files**: Load `.nef` and `.manifest.json` files.
    ```csharp
    using (var nefStream = new FileStream("path/to/your.nef", FileMode.Open, FileAccess.Read))
    {
        NefFile nefFile = nefStream.DeserializeNefFile();
    }

    ContractManifest manifest = ContractManifest.Parse(File.ReadAllBytes("path/to/your.manifest.json"));
    ```

2. **Create Deployment Transaction**:
    ```csharp
    Transaction tx = await contractClient.CreateDeployContractTxAsync(nefFile.ToArray(), manifest, senderKeyPair);
    ```

3. **Broadcast Transaction**: Send the deployment transaction to the blockchain.
    ```csharp
    await client.SendRawTransactionAsync(tx);
    ```

4. **Verify Deployment**: Check the transaction's execution status to ensure successful deployment.
    ```csharp
    // Utilize WalletAPI to monitor the transaction status
    await EpicChainAPI.WaitTransactionAsync(tx)
        .ContinueWith(task => Console.WriteLine($"Transaction status: {task.Result.VMState}"));
    ```

### Contract Invocation

Contracts can be invoked in read-only mode or through on-chain transactions, modifying the blockchain state.

#### Read-Only Invocation:

1. **Simulation**: Use `ContractClient.TestInvokeAsync` for simulating contract methods that don’t alter the blockchain state.
    ```csharp
    var result = await contractClient.TestInvokeAsync(contractHash, "methodName");
    ```

2. **Direct Invocation**: Directly call the `InvokeScriptAsync` method on `RpcClient` for execution results.
    ```csharp
    byte[] script = contractHash.MakeScript("methodName");
    var invokeResult = await client.InvokeScriptAsync(script);
    ```

#### On-Chain Transactions:

For on-chain interactions (e.g., token transfers):

1. **Construct Invocation Script**: Define the operation, like transferring tokens.
    ```csharp
    byte[] script = scriptHash.MakeScript("transfer", sender, receiver, amount);
    ```

2. **Create Transaction**: Utilize `TransactionManager` for transaction scaffolding, signing, and broadcasting.
    ```csharp
    TransactionManager txManager = await new TransactionManagerFactory(client)
        .MakeTransactionAsync(script, signers).SignAsync(senderKey);
    ```

3. **Broadcast Transaction**: Ensure it's dispatched to the network.
    ```csharp
    await client.SendRawTransactionAsync(txManager.Transaction);
    ```

4. **Confirmation**: Await the transaction to be mined and verify its execution status.
    ```csharp
    // Monitoring for transaction confirmation and status
    await EpicChainAPI.WaitTransactionAsync(txManager.Transaction)
        .ContinueWith(task => Console.WriteLine($"Execution: {task.Result.VMState}"));
    ```

### EEP-17 Tokens:

`Eep17API` class simplifies operations related to EEP-17 tokens, encapsulating common functionalities:

- **Creating Transfer Transactions**:
    ```csharp
    Transaction tx = await Eep17API.CreateTransferTxAsync(scriptHash, senderKeyPair, receiver, amount);
    ```

- **Retrieving Token Info**: Name, symbol, decimals, total supply, and account balance.
    ```csharp
    string tokenName = await Eep17API.NameAsync(tokenScriptHash);
    ```

Following these structured approaches to deploying and invoking contracts on EpicChain ensures that developers can effectively utilize blockchain capabilities, whether for deploying new smart contracts, conducting token transfers, or engaging with deployed contracts for comprehensive blockchain solutions.





















<br/>