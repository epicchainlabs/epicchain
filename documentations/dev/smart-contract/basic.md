---
title: Basic
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




# Journey into Smart Contract Engineering on EpicChain

Embark on an enlightening expedition into the realm of smart contract development on the EpicChain platform. This comprehensive guide serves as your astrolabe, navigating through the intricate constellations of smart contract code, starting with the celestial body known as the "Hello World" contract.

## Unraveling the "Hello World" Contract

The cornerstone of any smart contract developer's odyssey, our "Hello World" contract, lays the foundation of smart contract creation on the EpicChain. This example leverages EpicChain's distinct namespaces and base classes, paving your path to becoming a smart contract architect.

```csharp
using EpicChain;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Framework;
using EpicChain.SmartContract.Framework.Native;
using EpicChain.SmartContract.Framework.Services;
using System;

namespace Helloworld
{
    [ManifestExtra("Author", "EpicChain")]
    [ManifestExtra("Email", "dev@EpicChain.org")]
    [ManifestExtra("Description", "A beacon in the world of contracts")]
    public class Contract1 : SmartContract
    {
        // The contract's guardian, immutable and eternal.
        [InitialValue("NiNmXL8FjEUEs1nfX9uHFBNaenxDHJtmuB", ContractParameterType.Hash160)]
        static readonly UInt160 Owner = default;

        // A lantern guiding the verification process.
        private static bool IsOwner() => Runtime.CheckWitness(Owner);

        public static bool Verify() => IsOwner();

        public static string MyMethod() {
            return Storage.Get(Storage.CurrentContext, "Hello");
        }

        // The dawn of deployment, where "World" meets "Hello".
        public static void _deploy(object data, bool update) {
            if (update) return;
            Storage.Put(Storage.CurrentContext, "Hello", "World");
        }

        // Evolution and adaptability, key to survival.
        public static void Update(ByteString nefFile, string manifest) {
            if (!IsOwner()) throw new Exception("Unauthorized cosmic entity.");
            ContractManagement.Update(nefFile, manifest, null);
        }

        // The contract's inevitable return to stardust.
        public static void Destroy() {
            if (!IsOwner()) throw new Exception("Unauthorized cosmic entity.");
            ContractManagement.Destroy();
        }
    }
}
```

### Stellar Constants: Guideposts in the Cosmos of Contracts

In the vast expanse of smart contract development, constants are the North Star, guiding our code's voyage. These immutable elements signify the foundational aspects of our contract, ensuring reliability and integrity.

**Contract Properties: Bedrock of Trust**

- **Immutable Owner**: Like the fixed stars in the night sky, the owner's address is an unchangeable constant, ensuring that only the designated guardian can alter the contract's course.

```csharp
[InitialValue("YourAddressHere", ContractParameterType.Hash160)]
static readonly UInt160 Owner = default;
```

- **Constants as Cosmic Laws**: Constants like 'factor' serve as the laws of physics for our contract's universe, providing predictability and stability amidst the quantum fluctuations of blockchain transactions.

```csharp
private const ulong factor = 100000000;
```

### The Storage Nebula: Memory of the Universe

In the ether of the blockchain, the storage of a smart contract is akin to the memory of a civilization. This persistent archive records the eons of interactions, standing as a testament to the contract's operations.

**Key-Value Cosmos: The Foundation of Memory**

- **Primitive Elements**: The fundamental operations - retrieving (`Get`) and inscribing (`Put`) data - act as the primal forces shaping the storage landscape.

```csharp
Storage.Put(Storage.CurrentContext, "totalSupply", 100000000);
```

- **Structured Memory**: For complex data structures, `StorageMap` unfolds as a dimension within our contract's universe, grouping related data under a single key.

```csharp
StorageMap contract = new(Storage.CurrentContext, "galaxy");  // Crafting a new cosmos for our contract's data.
var value = contract.Get("totalSupply");  // Unveiling the mysteries of our universe's supply.
```

### Data Types: Tools to Sculpt the Cosmos

Venturing into smart contract development with C# on EpicChain requires an understanding of the dialect spoken by the EpicChainVM. This lean language focuses on the essential types, refining the expansive toolkit of C# into a specialized lexicon for blockchain wizardry.

**EpicChainVM Types**: The elemental particles forming the building blocks of our smart contract universe.

**C# Compatibility**: A selective adoption of C# types ensures your spells cast in the blockchain realm are potent yet efficient, avoiding the pitfalls of excess and frivolity.

### Crafting the First Chronicle: The EpicChain DNS Contract

With the cosmic insights gained from our "Hello World" exploration, the journey advances towards creating a DNS system, a registry of domain names, akin to charting the stars in the blockchain galaxy. This system employs the principles discussed, serving as your rite of passage into the realm of smart contract development.

**Envisioning the Contract's Galaxy**

The structure of our DNS smart contract is an odyssey in itself, brimming with opportunities to apply our understanding of properties, storage mechanisms, and data types. As you embark on this journey, remember that each line of code is a step towards charting unexplored territories in the EpicChain cosmos.

**Navigating Through Code**: A Step-by-Step Cosmic Voyage

In this voyage across coding constellations, you will learn to:
- **Illuminate the Blockchain with "Hello World"**: Understand the basics and set the stage for grander adventures.
- **Guideposts and Constants**: Learn to use constants as navigational stars in the smart contract universe.
- **The Storage Nebula**: Delve into the memory of the blockchain, learning to store and retrieve the lore of your contract.
- **The Language of the Cosmos**: Master the types and structures that resonate with the EpicChainVM, sculpting your code to perfection.

With these celestial tools and insights, your journey into the heart of smart contract development on EpicChain begins. May your code be strong, your logic clear, and your contracts resilient as you navigate the endless possibilities of the blockchain universe.






















<br/>