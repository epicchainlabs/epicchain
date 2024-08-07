---
title: Interface
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








# Mastering Wallet Interactions on EpicChain

Diving into EpicChain development, understanding wallet interactions is pivotal. This document explores the essential components of accounts and wallets, highlighting how to utilize the `WalletAPI` for operations such as balance inquiries, claiming EpicPulse, and asset transfers.

## Introduction to Accounts and Wallets

### Account

An account represents a user's identity within the EpicChain network, essentially a pair of private and public keys.

- **Creating a KeyPair**: Generate a new private and public key pair.
  ```csharp
  byte[] privateKey = new byte[32];
  using (RandomNumberGenerator rng = RandomNumberGenerator.Create())
  {
      rng.GetBytes(privateKey);
  }
  KeyPair keyPair = new KeyPair(privateKey);
  ```

### Private Key

The private key, a critical authorization tool, is used to sign transactions, embodying account ownership.

- **WIF Format**: The Wallet Import Format (WIF) provides another representation of the private key.
  ```csharp
  // Export KeyPair as WIF
  string wif = keyPair.Export();

  // Get KeyPair from WIF
  KeyPair keyPairFromWif = Utility.GetKeyPair(wif);
  ```

### Public Key

The public key serves to verify signatures from the corresponding private key, computable from the private key itself.

### Account Script Hash and Address

- The **script hash** (`ScriptHash`), a unique identifier generated from the public key, enables account identification and is reversible to an **address** format, the more commonly used representation in transactions.

## Wallet

A wallet encompasses one or multiple accounts, serving as a key management tool. EPP6 wallets, which can be serialized into JSON files containing encrypted keys, are commonly adopted in EpicChain.

- **Creating and Saving an EPP6 Wallet**:
  ```csharp
  string path = "wallet_new.json";
  string password = "YourPassword";
  EPP6Wallet wallet = new EPP6Wallet(path);
  using (wallet.Unlock(password))
  {
      wallet.CreateAccount(keyPair.PrivateKey);
  }
  wallet.Save();
  ```

## Utilizing WalletAPI for EpicChain Interactions

### Initializing WalletAPI

Create an instance of `WalletAPI` to manage wallet-related operations.

```csharp
RpcClient client = new RpcClient(new Uri("http://localhost:20332"), null, null, ProtocolSettings.Load("config.json"));
WalletAPI walletAPI = new WalletAPI(client);
```

### Balance Inquiry

Query the balance of EPP-17 assets using `WalletAPI`.

```csharp
// Get EpicChain balance
BigInteger epicChainBalance = await walletAPI.GetEpicChainBalanceAsync("Address").ConfigureAwait(false);

// Get EpicPulse balance
decimal epicPulseBalance = await walletAPI.GetEpicPulseBalanceAsync("Address").ConfigureAwait(false);
```

### Claiming EpicPulse

EpicPulse is automatically claimed upon a EpicChain transaction. Construct a self-transaction to claim pending EpicPulse.

```csharp
Transaction transaction = await walletAPI.ClaimEpicPulseAsync("WIF").ConfigureAwait(false);
```

### Asset Transfer

`WalletAPI` facilitates transferring EPP-17 assets, supporting single or multi-signature accounts.

- **Simple Transfer Example**:
  ```csharp
  await walletAPI.TransferAsync("EpicChainHash", "WifOfSender", "ReceiverAddress", 10).ConfigureAwait(false);
  ```

- **Multi-signature Transfer**:
  Make a multi-signature transaction specifying a list of public keys and required signatures.

## Conclusion

Understanding and effectively managing wallet operations are essential for developing robust EpicChain applications. From account creation, encryption practices, balance management, to sophisticated asset transfer mechanisms including multi-signature configurations, the `WalletAPI` provides rich functionalities tailored for various application scenarios. Whether you're building wallet clients, decentralized applications, or exploring the vast possibilities of EpicChain, mastering these interactions opens the door to innovative blockchain solutions.


















<br/>