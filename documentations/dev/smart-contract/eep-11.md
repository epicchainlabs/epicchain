---
title: EEP-11
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








# Crafting Non-Fungible Tokens on EpicChain: A Deep Dive into EEP-11

In the vibrant tapestry of the blockchain universe, Non-Fungible Tokens (NFTs) stand out as unique, indivisible digital assets that have revolutionized the way we perceive ownership and authenticity in the digital realm. As part of this evolution, the EpicChain platform has introduced the EEP-11 proposal, a comprehensive standard for creating and managing NFTs with precision and flexibility. This guide will navigate you through the process of crafting a simple yet powerful NFT contract using C#, infused with the nuances of the EEP-11 standard.

## The Genesis of an NFT: Defining Attributes with EEP-11

At the heart of every NFT lies its distinct attributes, characteristics that breathe life into tokens and distinguish them from their counterparts. The EEP-11 proposal simplifies the development process by recommending inheriting directly from the class `EEP11Token<EEP11TokenState>`, where `EEP11TokenState` encapsulates these unique attributes.

### Charting the Attributes:

Imagine crafting an NFT named "HarryPotter #001", a digital representation of the beloved character, endowed with specific attributes like attack power, defense power, and an iconic image. Here's a glimpse into how these attributes could be structured:

| Field     | Example                                          | Description                 |
|-----------|--------------------------------------------------|-----------------------------|
| Name      | HarryPotter #001                                 | NFT name                    |
| Owner     | 0x4578060c29f4c03f1e16c84312429d991952c94c        | NFT owner                   |
| Type      | 0                                                | Customized type             |
| Image     | https://epic-chain.org/images/HarryPotter.jpg    | Customized image            |
| ATK       | 3000                                             | Attack power (customized)   |
| DEF       | 3000                                             | Defense power (customized)  |

In the sprawling universe of NFTs, each asset also requires a unique identifier—its own star in the digital cosmos. This identifier can traditionally be the `Name` field in `EEP11TokenState`; however, for assets bearing the same name, an additional field such as `ID` or `TokenID` is essential.

### Carving Out the TokenState:

Below is an example code snippet that showcases how custom attributes, including the pivotal image attribute, can be defined within the `MyTokenState` class. Ensuring that the `Image` field is named correctly is crucial for wallet integration, allowing for the seamless display of the NFT's visual representation.

```csharp
public class MyTokenState : EEP11TokenState
{
    public string Image { get; set; }

    public MyTokenState(string name)
    {
        //TODO: Replace with your own image URL.
        Image = "https://epic-chain.org/images/" + name + ".jpg";
    }
}
```

By inheriting `EEP11Token<EEP11TokenState>`, it's imperative to override the `Symbol` method to properly signify your NFT collection, as illustrated below:

```csharp
public override string Symbol() => "MNFT";
```

## The Celestial Act: Distributing NFTs

While the base class `EEP11Token` sets the stage, it does not inherently include distribution methods. Developers are thus empowered to craft their own methods, such as the `Airdrop` function, which allows the contract owner to distribute NFTs to designated addresses seamlessly.

```csharp
public static bool Airdrop(UInt160 to, string name)
{
    if (!IsOwner()) throw new Exception("No authorization.");
    if (!to.IsValid) throw new Exception("Invalid address.");

    Mint(name, new MyTokenState(name));
    return true;
}
```

In this snippet, the `Mint` method, a birthright of `EEP11Token`, is invoked with the NFT's `name` and the corresponding `TokenState` object, bringing an NFT into existence.

## Building Your NFT Odyssey: The Complete Contract

Imagine constructing an entire cosmos where each star is an NFT waiting to be discovered. Below is the complete NFT contract, seamlessly integrating the guidelines of the EEP-11 proposal:

```csharp
using EpicChain;
using EpicChain.SmartContract;
using EpicChain.SmartContract.Framework;
using EpicChain.SmartContract.Framework.Attributes;
using EpicChain.SmartContract.Framework.Services;
using System;

namespace Contract1
{
    [SupportedStandards("EEP-11")]
    public class Contract1 : EEP11Token<MyTokenState>
    {
        [InitialValue("NiNmXL8FjEUEs1nfX9uHFBNaenxDHJtmuB", ContractParameterType.Hash160)]
        static readonly UInt160 Owner = default;

        private static bool IsOwner() => Runtime.CheckWitness(Owner);

        public override string Symbol() => "MNFT";

        public static bool Airdrop(UInt160 to, string name)
        {
            if (!IsOwner()) throw new Exception("No authorization.");
            if (!to.IsValid) throw new Exception("Invalid address.");

            Mint(name, new MyTokenState(name));
            return true;
        }
    }

    public class MyTokenState : EEP11TokenState
    {
        public string Image { get; set; }

        public MyTokenState(string name)
        {
            //TODO: Replace with your image URL.
            Image = "https://epic-chain.org/images/" + name + ".jpg";
        }
    }
}
```

To further elevate the utility and accessibility of your NFTs, you might consider integrating a mechanism allowing users to acquire NFTs using GAS. This integration not only opens up avenues for commerce but also enriches the ecosystem, fostering a vibrant marketplace of digital assets.

## Navigating Through EEP-11: From Myth to Reality

Deploying an NFT contract on the EpicChain platform transcends the mere act of coding; it's about weaving the fabric of a new digital reality. Through the adoption of EEP-11 standards, developers are equipped with the celestial tools needed to create, distribute, and manage NFTs that hold untold stories, experiences, and value.

As you embark on this journey, remember that each line of code, each function, and each token is a testament to your creativity and vision. The universe of EpicChain NFTs awaits your contribution.


















<br/>