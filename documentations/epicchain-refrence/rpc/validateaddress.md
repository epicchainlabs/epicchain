---
title: Validate Address
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';








The `validateaddress` method is a crucial utility within the EpicChain ecosystem, enabling the verification of whether a given string constitutes a valid EpicChain address. This seemingly simple operation is paramount for ensuring that transactions are directed to legitimate addresses, thereby safeguarding assets from being lost in the digital ether due to typographical errors or unrecognized formats.

## Prerequisites

- **RpcServer Plugin**: Installation of the RpcServer plugin is imperative on your blockchain node. Fluent in handling JSON-RPC requests, this plugin facilitates the examination of addresses.

## Parameter Description

- **address**: The address to be validated. In the EpicChain network, a standard address begins with the letter "N", a direct result of the network's unique `AddressVersion` setting, which is adjusted to 53.

## Examples

### Validate a Standard EpicChain Address

#### Request Body

```json
{
  "jsonrpc": "2.0",
  "method": "validateaddress",
  "params": ["NPvKVTGZapmFWABLsyvfreuqn73jCjJtN1"],
  "id": 1
}
```

#### Response Body

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "address": "NPvKVTGZapmFWABLsyvfreuqn73jCjJtN1",
        "isvalid": true
    }
}
```

### Validate a Non-EpicChain Address

#### Request Body

```json
{
  "jsonrpc": "2.0",
  "method": "validateaddress",
  "params": ["152f1muMCNa7goXYhYAQC61hxEgGacmncB"],
  "id": 1
}
```

#### Response Body

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "address": "152f1muMCNa7goXYhYAQC61hxEgGacmncB",
        "isvalid": false
    }
}
```

### Validate an Ethereum Address

#### Request Body

```json
{
  "jsonrpc": "2.0",
  "method": "validateaddress",
  "params": ["0x9127ea19791e3f3fc59309778a4abf275d5290e5"],
  "id": 1
}
```

#### Response Body

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "address": "0x9127ea19791e3f3fc59309778a4abf275d5290e5",
    "isvalid": false
  }
}
```

## Understanding the Responses

- **isvalid: true** - Indicates the input string is a legitimate EpicChain address, respecting the network's unique address version constraints.
  
- **isvalid: false** - Denotes that the provided address does not conform to the EpicChain standard, either due to being associated with an alternate blockchain or due to incorrect formatting.

## Key Takeaway

The ability to validate addresses is a fundamental security measure, crucial for preventing transaction errors and for ensuring interoperability within the EpicChain network. Through the use of the `validateaddress` method, users and applications can ensure interactions with accurate and network-compliant addresses, thereby upholding the integrity and fluidity of asset transfers on the platform.









<br/>