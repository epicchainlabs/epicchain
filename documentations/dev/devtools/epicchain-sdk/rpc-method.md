---
title: RPC Method
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';









# Comprehensive Guide to Leveraging RpcClient for EpicChain Development

The `RpcClient` is a fundamental library for EpicChain development, enabling efficient interaction with the blockchain's Remote Procedure Call (RPC) interfaces. Whether your project involves creating wallet clients, integrating blockchain data into games, or any other application, this guide provides a walkthrough of utilizing `RpcClient` to its full potential.

## **Initial Setup**

### **Setting Up RpcClient**

Firstly, initialize `RpcClient` to connect it to an EpicChain node. You have the flexibility to connect to the TestNet, your local node, or another network configuration depending on your project's requirements.

```csharp
// Connecting to a TestNet Node
RpcClient client = new RpcClient(new Uri("http://seed1.neepic-chain.org:20112"), null, null, ProtocolSettings.Load("config.json"));

// Connecting to a Local Node
RpcClient localClient = new RpcClient(new Uri("http://localhost:20332"), null, null, ProtocolSettings.Load("config.json"));
```

A single instance of `RpcClient` is generally sufficient for an application, avoiding the need for reinstantiation for each method call.

## **Fetching Blockchain Data**

### **Block Information**

#### **Latest Block Hash and Height**

Retrieve the highest block's hash and the total block count:

```csharp
string bestBlockHash = await client.GetBestBlockHashAsync().ConfigureAwait(false);
uint blockCount = await client.GetBlockCountAsync().ConfigureAwait(false);
```

#### **Detailed Block Data**

Acquire detailed information or the serialized form of a specific block using its height or hash:

```csharp
RpcBlock blockByHash = await client.GetBlockAsync("YourBlockHash").ConfigureAwait(false);
string blockHex = await client.GetBlockHexAsync(1024).ConfigureAwait(false); // Example block height
```

#### **Block Headers**

Fetch specific block header information or its serialized form:

```csharp
RpcBlockHeader headerByIndex = await client.GetBlockHeaderAsync("10000").ConfigureAwait(false);
string headerHex = await client.GetBlockHeaderHexAsync("YourBlockHash").ConfigureAwait(false);
```

### **Transactions and Contracts**

#### **Transaction Details**

Retrieve transaction data either in specific form or as a raw hex string:

```csharp
RpcTransaction rpcTx = await client.GetRawTransactionAsync("YourTransactionID").ConfigureAwait(false);
string serializedTx = await client.GetRawTransactionHexAsync("YourTransactionID").ConfigureAwait(false);
```

#### **Contract Information**

Access contract states, operations, and stored values through:

```csharp
ContractState contractInfo = await client.GetContractStateAsync("YourContractHash").ConfigureAwait(false);
string storageValue = await client.GetStorageAsync("YourContractHash", "YourStoredKey").ConfigureAwait(false);
```

### **Network Information and Fees**

Calculate network fees, understand policy settings, and explore node connectivity:

```csharp
long networkFee = await client.CalculateNetworkFeeAsync(yourTransaction).ConfigureAwait(false);
PolicyAPI policy = new PolicyAPI(client);
long feePerByte = await policy.GetFeePerByteAsync().ConfigureAwait(false);
```

Interacting with the local node, discovering connected peers, and versioning:

```csharp
int connectedNodes = await client.GetConnectionCountAsync().ConfigureAwait(false);
RpcVersion nodeVersion = await client.GetVersionAsync().ConfigureAwait(false);
```

### **Smart Contracts and Tokens**

Invoke contract methods or run scripts in the VM to interact with smart contracts:

```csharp
RpcInvokeResult invokeResult = await client.InvokeFunctionAsync("ContractHash", "MethodName", new RpcStack[] { param1, param2 }, signer).ConfigureAwait(false);
```

Manage NEP-17 tokens, retrieving balance, transfer records, or invoking token methods:

```csharp
RpcNep17Balances nep17Balances = await client.GetNep17BalancesAsync("YourAddress").ConfigureAwait(false);
```

### **Wallet Functions and Plugin Usage**

Operate on the local wallet, from address management to asset transfers:

```csharp
bool isOpen = await client.OpenWalletAsync("wallet.json", "password").ConfigureAwait(false);
string newAddress = await client.GetNewAddressAsync().ConfigureAwait(false);
```

Utilize plugins for additional functionalities, such as obtaining app logs or tracking NEP-17 transfers:

```csharp
RpcApplicationLog appLog = await client.GetApplicationLogAsync("YourTransactionID").ConfigureAwait(false);
```

## **Utilizing `RpcClient` Efficiently**

This guide offers a detailed exploration of using `RpcClient` for interacting with the EpicChain blockchain. By harnessing these capabilities, developers can significantly enrich their applications with blockchain functionalities, from simple transactions to complex smart contract interactions.

Remember, efficient blockchain development is not just about executing transactions; it's about creating seamless, secure, and user-centric applications that leverage the immutable and distributed nature of blockchain technology. Happy coding!

















<br/>