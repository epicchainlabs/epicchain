---
title: EEP-17
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







# Implementing EEP-17: The Evolution of Token Standards on EpicChain

The blockchain ecosystem thrives on innovation and standardization, and the EpicChain blockchain is no exception. The introduction of the EEP-17 standard marks a significant evolution from its predecessor, EEP-5, setting a new benchmark for tokenized smart contracts' interaction mechanisms. EEP-17 not only enriches the functionality of token contracts but also simplifies their integration within the ecosystem, offering a unified framework for fungible tokens. This detailed guide will walk you through creating a compliant EEP-17 token contract using C#.

## EEP-17: The Core Components

EEP-17 enhances token contracts with a set of specific functionalities, ensuring a standardized approach to creating, transferring, and managing tokens on the EpicChain blockchain. Below are the primary methods and events an EEP-17 compliant contract must implement:

### Fundamental Methods

1. **totalSupply**
   - **Purpose:** Provides the total token supply within the system.
   - **Return Type:** Integer

2. **symbol**
   - **Purpose:** Returns a short, valid ASCII string symbol of the token managed in the contract, devoid of whitespace or control characters.
   - **Return Type:** String

3. **decimals**
   - **Purpose:** Specifies the number of decimals used by the token for user representation.
   - **Return Type:** Integer

4. **balanceOf**
   - **Purpose:** Fetches the token balance of a specific account.
   - **Parameters:** account (Hash160)
   - **Return Type:** Integer

5. **transfer**
   - **Purpose:** Facilitates the transfer of tokens from one account to another, implementing essential checks and invoking the `onEEP17Payment` method on the receiving contract if applicable.
   - **Parameters:** from (Hash160), to (Hash160), amount (Integer), data (Any)
   - **Return Type:** Boolean

### Primary Event

- **Transfer**
  - **Purpose:** Must be triggered when tokens are transferred, including self-transfers, token creation, and token burning events.
  - **Parameters:** from (Hash160), to (Hash160), amount (Integer)

## Crafting an EEP-17 Token Contract

The EEP-17 standard furthers the integration of smart contracts with the broader EpicChain ecosystem, making it crucial for developers to adhere to its requirements. Here's a simplified example illustrating how to implement an EEP-17 compliant token contract in C#:

```csharp
using EpicChain;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Framework;
using EpicChain.SmartContract.Framework.Native;
using EpicChain.SmartContract.Framework.Services;
using System;
using System.Numerics;

namespace Template.EEP17.CSharp
{
    public partial class EEP17 : SmartContract
    {
        // Triggered when tokens are transferred, including token creation and burning
        [DisplayName("Transfer")]
        public static event Action<UInt160, UInt160, BigInteger> OnTransfer;

        // Returns the total token supply deployed in the system
        public static BigInteger TotalSupply() => TotalSupplyStorage.Get();

        // Returns the token balance of the specified account
        public static BigInteger BalanceOf(UInt160 account)
        {
            if (!ValidateAddress(account)) throw new Exception("Invalid account address.");
            return AssetStorage.Get(account);
        }

        // Facilitates the transfer of tokens with comprehensive checks and logic
        public static bool Transfer(UInt160 from, UInt160 to, BigInteger amount, object data)
        {
            if (!ValidateAddress(from) || !ValidateAddress(to)) throw new Exception("Invalid from/to address.");
            if (amount <= 0) throw new Exception("Amount must be greater than zero.");
            if (!Runtime.CheckWitness(from) && !from.Equals(ExecutionEngine.CallingScriptHash)) throw new Exception("No authorization.");
            if (AssetStorage.Get(from) < amount) throw new Exception("Insufficient balance.");

            if (from == to) return true; // Self-transfer check

            AssetStorage.Reduce(from, amount);
            AssetStorage.Increase(to, amount);

            OnTransfer(from, to, amount); // Triggering the Transfer event

            // Invoking onEEP17Payment on the receiver contract if applicable
            if (IsDeployed(to)) Contract.Call(to, "onEEP17Payment", new object[] { from, amount, data });

            return true;
        }
    }
}
```

### Key Enhancements of EEP-17

- **onEEP17Payment Method:** EEP-17 mandates the invocation of the `onEEP17Payment` method when the receiving account is a contract, ensuring contracts can react to incoming token transfers.
- **Dynamic Manifest Entry:** The `name` method has migrated to the manifest file, requiring `[DisplayName("Token Name")]` annotation in contract code, enhancing the token contract's semantics.
- **Transformed Transfer Event:** The standard specifies the "Transfer" event with capital T, establishing consistency across token contracts.
- **Revised Asset Reception Logic:** EEP-17 removes the "IsPayable" flag, incorporating logic within the `onEEP17Payment` method to decide whether a contract can receive assets, providing greater flexibility and control.

As the EpicChain blockchain evolves, standards like EEP-17 are pivotal in harmonizing the functionalities of token contracts, fostering a vibrant and interoperable ecosystem. Developers embarking on the journey of creating EEP-17 compliant tokens are not only complying with a standard but are also contributing to the broader narrative of blockchain evolution.



















<br/>