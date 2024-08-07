---
title: Transaction
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








# EpicChain Transaction Construction Guide

## Overview
This guide covers the necessary steps to construct transactions for the EpicChain blockchain, leveraging the `RpcClient` module.

## Initializing RpcClient
Ensure the `RpcClient` and the network are configured correctly to validate transactions on the blockchain.

```csharp
RpcClient client = new RpcClient(new Uri("http://localhost:20332"), null, null, ProtocolSettings.Load("config.json"));


## Constructing a Transaction Script
The script determines transaction functions, such as transfers.

```csharp
UInt160 scriptHash = NativeContract.EpicChain.Hash;
byte[] script = scriptHash.MakeScript("transfer", sender, receiver, 1, "data");
```

## Managing Transaction with TransactionManager
Initialize `TransactionManagerFactory` with `RpcClient`, and `TransactionManager` with `script` and `signers`.

```csharp
TransactionManager txManager = await new TransactionManagerFactory(client)
        .MakeTransactionAsync(script, signers).ConfigureAwait(false);
```

### Adding Signatures
- **Single Signature**:
  `txManager.AddSignature(sendKey);`

- **Multiple Signatures**:
  ```csharp
  txManager.AddMultiSig(key1, 2, receiverKey.PublicKey, key2.PublicKey, key3.PublicKey);
  ```

- **Multi-Signature Contract**:
  Create a contract requiring 2 out of 3 KeyPairs for signing.

```csharp
Contract multiContract = Contract.CreateMultiSigContract(2, sendKey.PublicKey, key2.PublicKey, key3.PublicKey);
UInt160 multiAccount = multiContract.Script.ToScriptHash();
```

### Finalizing the Transaction
Sign the transaction with added signatures. Missing signatures or insufficient fees will raise an exception.

```csharp
Transaction tx = await txManager.SignAsync().ConfigureAwait(false);
```

## Transaction Construction Examples

### EPP17 Transfer Transaction
Transfers 1024 EpicChain from sender to receiver account, showcasing script versus signature distinctions.

#### Setup and Transfer
1. Initialize `RpcClient` and retrieve account `KeyPair`.
2. Define `Signers` and recipient `ScriptHash`.
3. Construct the transfer `script`.
4. Initialize `TransactionManager` with the script and signers.
5. Sign the transaction and broadcast over the EpicChain network.

```csharp
RpcClient client = new RpcClient("http://127.0.0.1:10332");
KeyPair sendKey = Utility.GetKeyPair("YourWIF");
UInt160 sender = Contract.CreateSignatureContract(sendKey.PublicKey).ScriptHash;
UInt160 receiver = Utility.GetScriptHash("ReceiverAddress");
...
await client.SendRawTransactionAsync(tx).ConfigureAwait(false);
```

### Multi-Signature Transaction to and from a Multi-Signature Account
Illustrates transactions involving multi-signature accounts requiring multiple KeyPairs for authorization.

#### Creating a Multi-Signature Contract
1. Define the KeyPairs involved and the contract requirements.
2. Generate the script for transferring EpicPulse or another asset.
3. Use `TransactionManager` to apply multiple signatures as per the contract's requirements.
4. Broadcast the transaction and monitor the blockchain for confirmation.

```csharp
KeyPair key2 = Utility.GetKeyPair("SecondWIF");
KeyPair key3 = Utility.GetKeyPair("ThirdWIF");
Contract multiContract = Contract.CreateMultiSigContract(2, sendKey.PublicKey, key2.PublicKey, key3.PublicKey);
...
await client.SendRawTransactionAsync(tx).ConfigureAwait(false);
```




















<br/>