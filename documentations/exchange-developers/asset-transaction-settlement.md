---
title: Asset Transaction Settlement
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';



# Transaction Processing on EpicChain

EpicChain revolutionizes the digital asset landscape through its adherence to the NEP-17 token standard. This standard facilitates the creation, management, and transfer of fungible digital assets across the network. Assets on EpicChain are meticulously managed through BALANCE operations, enabling seamless interaction within the blockchain ecosystem for various transactions such as balance inquiries, asset deposits, withdrawals, and more intricate asset operations.

## NEP-17 Asset Management

### Core Operations:
- **Balance Inquiries**: Users can query their current asset balances, ensuring transparency and control over their digital holdings.
- **Deposits**: The inflow of assets into a user's account, enhancing their digital asset portfolio.
- **Withdrawals**: Users can transfer assets out of their accounts for external usage or exchanges, facilitating liquidity within the blockchain ecosystem.

These operations are backbone components of the digital asset management framework on EpicChain, designed to provide users and exchanges with a robust suite of tools for asset interaction.

## Fee Mechanisms on EpicChain

### Network Fees:
The vitality of EpicChain's blockchain infrastructure is maintained through the implementation of network fees. These fees serve as incentives for consensus nodes — the principal architects behind block generation. This intricate mechanism ensures the sustainability and robust operation of the blockchain network.

- **Calculation of Network Fees**:
```plaintext
NetworkFee = VerificationCost + (tx.size * FeePerByte)
```
  - **VerificationCost**: The computational expense incurred during the execution of EVM instructions aimed at authenticating transaction signatures.
  - **tx.size**: Reflects the transaction's data footprint, measured in bytes.
  - **FeePerByte**: Standardized transaction fee for each byte of data, established at 0.00001 epicpulse, as defined in the PolicyContract.

Transactions not satisfying the minimum fee threshold are deemed invalid, upholding the network's integrity and performance efficacy.

### System Fees:
Integral to the execution of smart contracts on EpicChain, system fees cover the computational costs associated with processing instructions within the EpicChain Virtual Machine (EpicChainVM).

- **Determination of System Fees**:
```plaintext
SystemFee = InvocationCost = Aggregate of all executed opcode fees
```
The system fee is directly proportional to the complexity and computational demand of the executed smart contract instructions, ensuring a fair and equitable fee mechanism aligned with resource usage.

## Enhanced Blockchain Terminologies and Concepts
- **Consensus Nodes**: The linchpin of blockchain integrity and security, responsible for validating transactions and generating new blocks within the EpicChain ecosystem.
- **EpicChain Virtual Machine (EpicChainVM)**: A high-performance, Turing-complete virtual machine designed to execute smart contracts and manage digital assets on EpicChain, embodying flexibility and security.
- **Opcode Fees**: Micro-fees associated with specific operational codes (opcodes) executed within a smart contract, contributing to the calculation of the system fee.



# Enhancing Smart Contract Development with Reduced Instruction Fees on EpicChain

EpicChain's continuous evolution has led to significant advancements in smart contract development, particularly through the reduction of instruction fees on the EpicChain Virtual Machine (EpicChainVM). This cost optimization greatly benefits developers by lowering the barrier to entry for deploying and executing smart contracts on the blockchain.

## Instruction Fee Reduction

The instruction fees on EpicChainVM have been scaled down dramatically to 1/1000th of their original value in comparison to EpicChain Legacy. This substantial decrease directly translates to a more cost-effective development environment, encouraging innovation and broader experimentation with smart contract functionalities.

## Efficient Query Transactions

The EpicChain infrastructure supports diverse methods for querying user balances and asset information, crucial for exchange platforms to provide accurate and up-to-date financial information to their users.

### Querying User Deposit Address Balances

Exchanges can query the balance of user deposit addresses by constructing and sending JSON payloads to invoke specific RPC methods on the EpicChain RPC server:

1. **getnep17balances**: Requires the TokensTracker plugin. It’s designed to fetch the balance of NEP-17 assets associated with a specific address.

2. **invokefunction**: Requires the RpcServer plugin. It’s used for invoking smart contract methods, such as `balanceOf` to obtain the balance of a particular asset.

### Example of getnep17balances Request

To query NEP-17 asset balances for a given address, an exchange would send a request structured as follows:

```json
{
  "jsonrpc": "2.0",
  "method": "getnep17balances",
  "params": ["NVfJmhP28Q9qva9Tdtpt3af4H1a3cp7Lih"],
  "id": 1
}
```

### Response Analysis

The response from the EpicChain RPC server provides detailed information about the assets, including their hashes, the amounts held, and the last updated block.

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "balance": [
      {
        "asset_hash": "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
        "amount": "2",
        "last_updated_block": 52675
      },
      {
        "asset_hash": "0xd2a4cff31913016155e38e474a2c06d08be276cf",
        "amount": "700000000",
        "last_updated_block": 52675
      }
    ],
    "address": "NVfJmhP28Q9qva9Tdtpt3af4H1a3cp7Lih"
  }
}
```

To ascertain the corresponding asset symbol and decimals, an additional `invokefunction` request is necessary. This step is crucial for accurately calculating the user's balance by converting the numeric balance based on the asset's divisibility.

## Significance of Fee Reduction

The reduction of EpicChainVM instruction fees is a pivotal development that lowers the cost associated with smart contract execution. This adjustment not only fosters a developer-friendly ecosystem on EpicChain but also paves the way for more intricate and resource-intensive smart contract applications, driving innovation and expanding the possibilities within the EpicChain ecosystem.


The process described above highlights the technical nuances and specific methodologies employed for querying and interpreting digital asset balances on the EpicChain blockchain. By utilizing the `invokefunction` RPC call, exchanges and users alike can access detailed asset information, including balances, decimals, and symbols, which are crucial for accurately reflecting a user's digital asset holdings. Below is a step-by-step guide, enriched with blockchain terminologies and insights, that elaborates on querying asset information via the `invokefunction` method and interpreting responses for comprehensive balance management.

## Leveraging `invokefunction` for Asset Information

### Initiating the Query

To query specific details about a user's asset holding, one would typically use the `invokefunction` RPC call. The request necessitates the inclusion of the script hash of the NEP-17 asset you are interested in, the method name you wish to invoke on the contract, and any optional arguments required by the contract method.

#### Building the JSON Request

A well-constructed JSON request for invoking a function looks like this:

```json
{
  "jsonrpc": "2.0",
  "method": "invokefunction",
  "params": [
    "<script hash>",
    "<method name>",
    [
      "<optional arguments>"
    ]
  ],
  "id": 1
}
```

#### Example: Querying Balance

To query the balance for a specific account (for example, `NYxb4fSZVKAz8YsgaPK2WkT3KcAE9b3Vag`), one must convert the account address into a Hash160 format as required by the contract, and pass it as a parameter in the `invokefunction` request.

```json
{
  "jsonrpc": "2.0",
  "method": "invokefunction",
  "params": [
    "0xd2a4cff31913016155e38e474a2c06d08be276cf",
    "balanceOf",
    [
      {
        "type": "Hash160",
        "value": "0x762f8a2bf0e8673c64cc608ba69b9c2a946a188f"
      }
    ]
  ],
  "id": 3
}
```

### Interpreting the Response

The response from the EpicChain RPC server provides detailed insights into the queried operation. For balance queries, the response includes the script executed, the execution state (HALT indicates success), the EpicPulse consumed for the transaction, and the actual data stack containing the queried information, often in integer or ByteString format for account balances.

#### Decimals and Symbol Queries

Additional `invokefunction` calls are necessary for fetching the asset's decimals and symbol. These calls are similar in structure to the balance query but invoke the `decimals` and `symbol` methods respectively, allowing for accurate interpretation and display of asset balances in user interfaces.

### Practical Considerations

1. **EpicPulse Consumed**: Indicates the computational cost incurred for the query, guiding users and exchanges in managing their query operations efficiently.
2. **Result Interpretation**: The result section contains the essence of the query. For balances, it directly indicates the held amount. For symbol and decimals, it provides foundational data necessary for correct balance representation.

3. **Blockchain Synchronization**: Ensure that your node or interface with the EpicChain blockchain is fully synchronized for the most accurate and current informational retrieval.

4. **Security Practices**: While constructing and sending RPC calls, always follow best security practices, especially when handling user data and performing operations that might affect digital asset balances.

This enhanced guide serves as a comprehensive resource for managing and querying digital asset information on EpicChain, ensuring clarity, accuracy, and efficiency in handling NEP-17 assets within the evolving blockchain ecosystem.


# Comprehensive Management of NEP-17 Asset Transactions on EpicChain

EpicChain's advanced infrastructure facilitates robust management of NEP-17 asset transactions. This includes monitoring and recording user deposits, withdrawals, and efficiently handling query transactions to maintain accurate user balances in exchange databases. Here, we delve into the methods used to manage these transactions, capitalizing on EpicChain's blockchain capabilities and RPC (Remote Procedure Call) techniques.

## Calculating User Balances

After obtaining asset information through RPC calls, exchanges can compute user balances with precision. Balances are calculated by dividing the returned numeric value by `10^decimals`, where `decimals` is the granularity of the asset in question. This calculation ensures accurate representation of users' holdings, taking into account the asset's divisibility.

## Responding to User Queries

- **Database Synchronization**: Exchanges maintain their internal databases to record and reflect user balances. This involves synchronizing with each new block on EpicChain to track transactions affecting user accounts.
- **Transaction Monitoring**: Custom programs are written to parse through block details, identifying deposit and withdrawal transactions. These are then recorded in the exchange's database to keep user balances up to date.

## Processing User Deposits

To accurately process user deposits:

1. **Block Details Retrieval**: Utilize the `getblock` API to fetch comprehensive details of transactions within each block.
2. **Transaction Log Analysis**: Employ the `getapplicationlog` API to obtain detailed logs of each "InvocationTransaction". Parsing these logs allows for the accurate recording of deposits into users' accounts.

### Example: Utilizing `getapplicationlog`

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
        "txid": "...",
        "trigger": "Application",
        "vmstate": "HALT",
        "EpicPulseconsumed": "...",
        "notifications": [
            {
                "contract": "0xd2c...f699ec",
                "eventname": "Transfer",
                "state": {
                    "value": [
                        {"type": "ByteString", "value": "..."},
                        {"type": "ByteString", "value": "..."},
                        {"type": "Integer", "value": "800000000000"}
                    ]
                }
            }
        ]
    }
}
```

### Interpreting the Log:

- **Contract**: Identifies the NEP-17 asset's smart contract that processed the transfer.
- **EventName**: Indicates a transfer event.
- **State**: Contains transaction details:
  - **From Account**: Source account of the transfer.
  - **To Account**: Destination account (exchange's deposit address).
  - **Amount**: The transferred asset amount, accurately reflecting user deposits after considering the asset's divisibility.

## Handling Transfer Outcomes

- **Successful Transfer**: Indicated by "HALT" state and presence of a "Transfer" event.
- **Failed Transfer**: No "Transfer" notification returned, and transaction ends in "HALT" with a failing condition.
- **EpicChainVM Exception**: Potentially no "Transfer" event, and transaction ends in "FAULT" state, indicative of execution issues.

## Data Conversion and Format

EpicChain processes hexadecimal strings in specific orders based on their prefix, a crucial factor in transaction data handling. Understanding and correctly applying this data format is essential for accurate transaction parsing and balance updates.

By adhering to these methodologies, exchanges can ensure the integrity of user balances, reflecting actual holdings and transaction outcomes on the EpicChain blockchain. This comprehensive approach aids in maintaining a transparent, secure, and efficient platform for managing NEP-17 asset transactions.


# Efficient Handling of User Withdrawals on EpicChain

EpicChain offers a versatile set of tools for exchanges to process user withdrawals efficiently and securely. These tools range from command line options to robust RPC methods, catering to different operational preferences and requirements. Below is a detailed exploration of the mechanisms available for processing withdrawals on EpicChain.

## EpicChain-CLI Command: `send`

The `send` command in the EpicChain-CLI is a straightforward way to transfer assets from one address to another. It supports both asset IDs and their aliases, enabling quick transactions directly from the command line interface.

### Syntax and Parameters:

```plaintext
send <id|alias> <address> <amount>|all [from=null] [signerAccounts=null]
```

- **id|alias**: The asset's unique identifier or its alias.
- **address**: The recipient's address.
- **amount|all**: The amount to be sent or "all" to send the entire balance.
- **from**: The sender's address (optional).
- **signerAccounts**: The signer's address (optional).

### Command Verification:

Executing a `send` command requires wallet password verification to ensure transactional security.

#### Example:

```plaintext
EpicChain> send epicpulse NYxb4fSZVKAz8YsgaPK2WkT3KcAE9b3Vag 100
```

This command transfers 100 EpicPulse to the specified address, with the transaction ID displayed upon successful execution.

## RPC Method: `sendfrom`

For a more programmatic approach to withdrawals, the `sendfrom` RPC method allows specifying the sender and recipient addresses directly in the transaction request.

### Request Structure:

```json
{
  "jsonrpc": "2.0",
  "method": "sendfrom",
  "params": ["<script hash>", "<address from>", "<address to>", "<amount>"],
  "id": 1
}
```

- **script hash**: The smart contract's script hash for the asset.
- **address from**: The sender's address.
- **address to**: The recipient's address.
- **amount**: The amount to be transferred.

#### Executing Withdrawals:

To initiate a withdrawal, construct the JSON request with the necessary parameters and send it to the EpicChain RPC server. The response will contain the transaction hash (`txid`), signifying the successful execution of the withdrawal.

## Opening the Wallet with `openwallet`

Prior to executing any wallet-related RPC commands, it's essential to unlock the wallet using the `openwallet` method. This step ensures that transactions can be signed and sent from the specified wallet.

### Request Example:

```json
{
  "jsonrpc": "2.0",
  "method": "openwallet",
  "params": ["a.json", "111111"],
  "id": 1
}
```

This command opens the wallet file `a.json` using the provided password, enabling subsequent transactional operations.

## Handling Multiple Withdrawals with `sendmany`

For batch processing of withdrawals, the `sendmany` RPC method facilitates multiple transfers in a single request. This method is particularly useful for exchanges managing a high volume of withdrawal requests efficiently.


# Streamlined Withdrawal Processes on EpicChain via RPC Methods

To accommodate various withdrawal needs, EpicChain supports multiple RPC methods, including `sendtoaddress` and `sendmany`, enabling seamless asset transfers to users. These methods are crucial for exchanges and wallet services looking to facilitate user withdrawals with efficiency and security.

## Using `sendtoaddress` for Direct Transfers

The `sendtoaddress` RPC method offers a straightforward approach to send a specific amount of assets to a designated address.

### Constructing the Request:

```json
{
  "jsonrpc": "2.0",
  "method": "sendtoaddress",
  "params": [
    "0x70e2301955bf1e74cbb31d18c2f96972abadb328",
    "Nhiuh11SHF4n9FE6G5LuFHHYc7Lgws9U1z",
    1000
  ],
  "id": 1
}
```

In this example, 1000 units of the specified asset (identified by its script hash) are being sent to the given address.

### Response Analysis:

Upon successful transaction submission, you receive a response detailing the transaction's hash, size, version, and various other parameters such as system fee (`sysfee`), network fee (`netfee`), and transaction attributes.

## Leveraging `sendmany` for Batch Transactions

For more complex withdrawal scenarios, such as sending multiple assets to various addresses in a single transaction, the `sendmany` method is invaluable.

### Example Request:

```json
{
    "jsonrpc": "2.0",
    "method": "sendmany",
    "params": [
        "NcphtjgTye3c3ZL5J5nDZhsf3UJMGAjd7o",
        [
            {
                "asset": "0xf61eebf573ea36593fd43aa150c055ad7906ab83",
                "value": 100,
                "address": "Nhiuh11SHF4n9FE6G5LuFHHYc7Lgws9U1z"
            },
            {
                "asset": "0x70e2301955bf1e74cbb31d18c2f96972abadb328",
                "value": 1000,
                "address": "Nhiuh11SHF4n9FE6G5LuFHHYc7Lgws9U1z"
            }
        ]
    ],
    "id": 1
}
```

This request demonstrates how to send 100 units of one asset and 1000 units of another asset to the same recipient, but it can easily be adapted for multiple recipients or different asset quantities.

### Understanding the Response:

The response to a `sendmany` request will similarly detail the transaction hash, along with other transaction-specific information, offering a complete view of the batch transfer's outcome.

## Key Takeaways:

- **RPC Accessibility**: Both `sendtoaddress` and `sendmany` provide programmatic access to withdrawals on EpicChain, accommodating a range of scenarios from simple single-asset transfers to complex, multi-asset batch transactions.
- **Security Considerations**: As with all blockchain operations, ensuring the security of transaction requests—such as guarding against unauthorized access and confirming transaction details before submission—is paramount.
- **Operational Efficiency**: These RPC methods streamline the withdrawal process for exchanges and wallets, enabling them to handle user requests with greater speed and fewer resources.
- **Flexibility**: Whether dealing with straightforward withdrawals or managing complex distribution schemes, EpicChain's RPC options cater to various user and application needs efficiently and securely.

By integrating these methods into its operational workflow, an exchange or wallet service can significantly enhance its withdrawal and transaction management processes, delivering a better user experience on the EpicChain platform.








<br/>