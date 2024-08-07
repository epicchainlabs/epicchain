---
title: Chain Info
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';



# Utilizing RpcClient for Blockchain Data Access on EpicChain

The EpicChain's RPC (Remote Procedure Call) module is an indispensable tool for developers, offering a streamlined approach to retrieving essential blockchain data and performing various operations. With the `RpcClient` library, gaining insights into block heights, transaction details, and smart contracts becomes a more intuitive process. Here, we explore how to leverage `RpcClient` for fetching blockchain information and interacting with smart contracts and policy settings on EpicChain.

## **Accessing Blockchain Information**

### **Getting the Latest Block Information**
To start, create an instance of `RpcClient` targeting an EpicChain node with RPC enabled:

```csharp
RpcClient client = new RpcClient(new Uri("http://localhost:20332"), null, null, ProtocolSettings.Load("config.json"));
```

#### **Fetching the Block Height and Hash:**

```csharp
// Get the hash of the tallest block in the main chain
string hash = await client.GetBestBlockHashAsync().ConfigureAwait(false);

// Get the count of blocks in the main chain
uint count = await client.GetBlockCountAsync().ConfigureAwait(false);
```

### **Retrieving Block Data:**

```csharp
// Fetch a block using its height and obtain the Base64-encoded string
string blockHex = await client.GetBlockHexAsync("166396").ConfigureAwait(false);

// Alternatively, fetch using the block hash
RpcBlock block = await client.GetBlockAsync("0x4e61cd9d76e30e9147ee0f5b9c92f4447decbe52c6c8b412d0382a14d3a0b408").ConfigureAwait(false);
```

### **Contract Information:**

To fetch details about a specific contract, such as its script, hash, and manifest:

```csharp
// Retrieve EpicChain contract state
ContractState contractState = await client.GetContractStateAsync(NativeContract.EpicChain.Hash.ToString()).ConfigureAwait(false);
```

## **Policy Information:**

Interact with the native `PolicyContract` through `PolicyAPI` for information related to network policies:

```csharp
PolicyAPI policyAPI = new PolicyAPI(client);

// Retrieve the system fee per byte
long feePerByte = await policyAPI.GetFeePerByteAsync().ConfigureAwait(false);

// Get maximum block size
uint maxBlockSize = await policyAPI.GetMaxBlockSizeAsync().ConfigureAwait(false);

// Fetch the maximum transaction count per block
uint maxTransactionsPerBlock = await policyAPI.GetMaxTransactionsPerBlockAsync().ConfigureAwait(false);

// Determine if an account is blocked
bool isBlocked = await policyAPI.IsBlockedAsync(account).ConfigureAwait(false);
```

## **EEP-17 Contract Information:**

EEP-17, EpicChain's asset standard (similarly functioning to Ethereum's ERC-20), encompasses EpicChain and EpicPulse among other assets. Retrieve EEP-17 token details using `EEP17API`:

```csharp
EEP17API EEP17API = new EEP17API(client);
RpcEEP17TokenInfo tokenInfo = await EEP17API.GetTokenInfoAsync(NativeContract.EpicChain.Hash).ConfigureAwait(false);
```

This summary elucidates the seamless access and management of blockchain data via `RpcClient` on EpicChain, encompassing block details, smart contracts, policy information, and EEP-17 token data. Harnessing these capabilities, developers can craft sophisticated applications, enhancing the blockchain ecosystem with innovative solutions.























<br/>