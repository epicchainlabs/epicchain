---
title: Invoke Script
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';







The `invokescript` method lets you test how a script performs on the blockchain without making any actual changes. It simulates running a VM script - including those that would be produced by `invokefunction` calls - and provides a detailed outcome of this execution. This method is invaluable for developers aiming to debug or test smart contract behavior in a non-destructive manner.

## Prerequisites

- **RpcServer Plugin**: Ensure the RpcServer plugin is installed on your node. This plugin enables the node to process JSON-RPC requests.

## Parameters Description

- **script**: The script you want to run on the VM. It's typically the output of a smart contract function that you desire to test.
- **signers** (Optional): 
  - **account**: The account providing a signature.
  - **scopes**: Defines the scope or level of access the signature possesses.
  - **allowedcontracts**: Specifies which contracts the signature is effective on, applicable when `scopes` is set to `CustomContracts`.
  - **allowedgroups**: Specifies which public keys the signature is effective on, applicable when `scopes` is `CustomGroups`.
- **use diagnostic** (Optional): Whether to return detailed simulation information and storage change data, defaulting to false.

### Signature Scopes
Scopes determine the signature's authorization level:

- `None`: Only signs the transaction without authorizing contract use.
- `CalledByEntry`: Limits use to the initial contract call in a chain of calls.
- `CustomContracts`: Allows use within specified contracts.
- `CustomGroups`: Allows use within specified contract groups.
- `Global`: Unrestricted use across any contract on the blockchain, requiring extreme trust.

## Example Usage

### Request Body

Here's an example of a JSON-RPC request using the `invokescript` method:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "invokescript",
  "params": [
    "DAABECcMFPqJ+ywU3w9Z3d8E9uVlF/KzSq7rDBRmmdXxSKUGNq3wQcppLAdAe8sD+hTAHwwIdHJhbnNmZXIMFMTiKascQ8IoLIUKuF3Y3n1ndaOhQWJ9W1I=",
    [
      {
        "account": "0xfa03cb7b40072c69ca41f0ad3606a548f1d59966",
        "scopes": "CalledByEntry"
      }
    ],
    true
  ]
}
```

### Response Body

The response includes the script that was tested, the outcome of the VM execution, gas consumption, and detailed diagnostic information:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "script": "...",
    "state": "HALT",
    "epicpulseconsumed": "1490312",
    "exception": null,
    "notifications": [],
    "diagnostics": {...},
    "stack": [...],
    "tx": "..."
  }
}
```

### Response Description

- **script**: The tested script.
- **state**: Execution state (`HALT` for success, `FAULT` for failure).
- **epicpulseconsumed**: The system fee used for the invocation.
- **exception**: Details of any exception that occurred.
- **notifications**: Events emitted during the script execution.
- **diagnostics**: Provides insights into contract invocations and simulated storage changes.
- **stack**: The result of the VM execution.
- **tx**: Hex string representation of the transaction, available when the wallet is open and signers added.

## Considerations

- **Security and Synchronization**: Ensure your client or node is fully synchronized and securely configured to obtain accurate and relevant results.
- **Diagnostic Details**: The `use diagnostic` flag being set to true provides invaluable insights for debugging but should be used judiciously to not overwhelm with data.
- **Session Handling**: If session handling is enabled (`SessionEnabled` in the RpcServer's `config.json`), sessions are returned for further detail retrieval on iterators, aiding in comprehensive contract testing.

The `invokescript` method is a powerful tool that aids developers in optimizing and debugging smart contracts within the blockchain environment, providing a sandbox-like simulation to ensure correctness and efficiency before making any commit to the chain.









<br/>