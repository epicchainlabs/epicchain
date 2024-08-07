---
title: About
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';





# RpcClient Overview

The EpicChain ecosystem thrives on innovation, enabling developers to craft applications ranging from wallet clients to interactive games. A cornerstone of this development environment is the `RpcClient` library, a C# dependency designed to facilitate seamless interaction with EpicChain's RPC interfaces. Whether you're orchestrating transactions or invoking contracts, the `RpcClient` library streamlines the development process for applications on EpicChain N3.

This guide assumes you are working with EpicChain SDK within Visual Studio 2019 or later. Here's how to harness the main features of `RpcClient` for your project.

## **Main Features**

- **RPC Invocation**: Simplifies calling built-in RPC methods provided by the EpicChain network.
- **Transaction Management**: Offers tools for the construction, serialization, and deserialization of transactions.
- **Smart Contract Scripts**: Enables easy script generation for smart contract execution.
- **Wallet Functions**: Facilitates wallet operations including transfers, balance verifications, and GAS claim methods.

## **Adding `RpcClient` to Your Project**

To integrate `RpcClient` into your .NET project in Visual Studio 2019, follow these steps:

1. **Project Compatibility**: Ensure your .NET project version aligns with or surpasses the .NET version dependencies of the EpicChain SDK.

2. **Manage NuGet Packages**: Right-click on your project in the Solution Explorer and choose "Manage NuGet Packages".

3. **Search and Install**: Type `EpicChain.Network.RPC.RpcClient` in the search box and install the corresponding plugin that appears in the results.

Once installed, include the `RpcClient` library in your project files as necessary:

```csharp
using EpicChain.Network.RPC;
```

## **Constructing `RpcClient` with Configuration**

When dealing with transactions that require signatures, it's crucial that the `RpcClient` instance aligns with the network settings. To ensure this compatibility, load the EpicChain-CLI `config.json` during the `RpcClient` initialization:

```csharp
RpcClient client = new RpcClient(new Uri("http://localhost:20332"), null, null, ProtocolSettings.Load("config.json"));
```

## **Exception Handling**

Interaction with EpicChain nodes via `RpcClient` is conducted through RPC requests. If an RPC response contains an error, an exception is thrown, typically an `RpcException`. Here are some common error codes you may encounter:

- `-100`: Unknown transaction/blockhash
- `-300`: Insufficient funds
- `-301`: Necessary fee exceeds the Maxfee
- `-400`: Access denied
- `-500`: Relay not successful for various reasons (e.g., AlreadyExists, OutOfMemory, UnableToVerify, etc.)
- `-32600`: Invalid Request
- `-32601`: Method not found
- `-32602`: Invalid params
- `-32700`: Parse error

In addition to `RpcException`, anticipate .NET standard exceptions such as `ArgumentNullException` or `FormatException` based on input validation.

## **Project Repository and Community Engagement**

The `RpcClient` library is a feature subset of the broader `epicchain-modules` project. For those interested in delving into the source code or contributing to the project, visit:

- **Project Source**: [epicchain-modules on GitHub](https://github.com/epic-chain/epicchain-modules)

Engage with the EpicChain developer community by discussing issues, proposing enhancements, or asking questions related to `RpcClient` and other modules:

- **Issue Tracking**: [epicchain-modules Issues on GitHub](https://github.com/epic-chain/epicchain-modules/issues)

Embrace the power of EpicChain's `RpcClient` to propel your blockchain development ventures, leveraging its robust features for efficient application building on the EpicChain network.





















<br/>