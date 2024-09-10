# **EpicChain Node Configuration Guide**

This README provides a comprehensive explanation of each field in the configuration JSON file for an EpicChain node. This configuration file plays a pivotal role in ensuring the smooth operation of your node, handling logging, storage, P2P networking, and protocol-specific parameters.

---

## **ApplicationConfiguration**
The `ApplicationConfiguration` section controls essential application-level settings such as logging, storage, network, and wallet configurations. Each component here allows customization to optimize node performance and functionality.

### **1. Logger Configuration**
- **`Path:`**
  - Directory for storing log files. By default, logs are saved in a folder named "Logs". This is crucial for debugging and tracking node activities.

- **`ConsoleOutput:`**
  - Boolean flag (`true` or `false`) that determines if log outputs will also appear in the console. For development purposes, enabling this can help in real-time tracking of node behavior. Default is `false`.

- **`Active:`**
  - Another boolean flag that controls whether logging is activated for the node. Disabling logging (`false`) might enhance performance, though at the cost of losing traceability in case of issues. Default is `false`.

### **2. Storage Configuration**
- **`Engine:`**
  - Specifies the type of storage engine the node will use to store blockchain data. The following options are available:
    - `MemoryStore`: Temporary storage used mostly for testing and development. Data is stored in-memory and is lost on shutdown.
    - `LevelDBStore`: A persistent and disk-based storage engine that provides faster read/write operations. Suitable for production environments.
    - `RocksDBStore`: A high-performance storage engine, especially useful for high-load environments.

- **`Path:`**
  - Specifies the file system path where blockchain data is stored. Use `{0}` as a placeholder for network ID to dynamically allocate the storage path based on the network your node is connecting to.

### **3. P2P Configuration**
- **`Port:`**
  - Defines the port number used by the P2P network protocol. The default port for MainNet is `10111`, and for TestNet, it's `20111`. Ensure this port is open on your firewall and correctly configured to allow connections.

- **`MinDesiredConnections:`**
  - The minimum number of connections the node will try to maintain with other nodes in the network. This helps ensure a healthy connection state. Default is `10`.

- **`MaxConnections:`**
  - This defines the maximum number of peer connections that the node will support simultaneously. A higher number can increase network resilience but may use more system resources. Default is `40`.

- **`MaxConnectionsPerAddress:`**
  - Specifies the maximum number of connections that can be established from a single IP address. This helps prevent spamming from a particular node. Default is `3`.

### **4. UnlockWallet Configuration**
- **`Path:`**
  - The path to the wallet file containing private keys. This file is essential for signing transactions and participating in network consensus.

- **`Password:`**
  - The password used to decrypt the wallet. Ensure that this is stored securely, as it provides access to your wallet's private keys.

- **`IsActive:`**
  - Boolean flag that activates or deactivates the wallet functionality. Set to `true` if the wallet is to be used by the node, otherwise `false`. Default is `false`.

### **5. Contracts Configuration**
- **`EpicChainNameService:`**
  - This field contains the script hash of the EpicChain Name Service contract, which manages the name resolution on the blockchain. This contract ensures unique names are assigned to addresses and entities.
  - MainNet Hash: `0x50ac1c37690cc2cfc594472833cf57505d5f46de`
  - TestNet Hash: `0x50ac1c37690cc2cfc594472833cf57505d5f46de`

### **6. Plugins Configuration**
- **`DownloadUrl:`**
  - This specifies the URL where plugins can be downloaded. Plugins extend the functionality of your node by adding features such as new consensus mechanisms, additional logging, or wallet enhancements. The default URL points to EpicChain's GitHub repository: `https://api.github.com/repos/epicchainlabs/epicchain/releases`.

---

## **ProtocolConfiguration**
The `ProtocolConfiguration` section dictates the rules and parameters that govern the blockchain network and consensus mechanism. These settings control block times, transaction limits, and consensus validator behavior.

### **1. Network**
- **`Network:`**
  - This field specifies the unique network ID that the node will connect to. This ensures that nodes on different networks (MainNet, TestNet) are appropriately segregated.
    - MainNet ID: `860833102`
    - TestNet ID: `894710606`

### **2. AddressVersion**
- **`AddressVersion:`**
  - This field defines the version byte used during EpicChain address generation. Changing this can lead to incompatibility between different network addresses. Default value is `76`.

### **3. Block Timing**
- **`MillisecondsPerBlock:`**
  - This defines the block time interval, measured in milliseconds. Shorter intervals lead to faster transaction finality but may reduce security. The default interval is `15000` milliseconds (15 seconds per block).

### **4. Transaction Limits**
- **`MaxTransactionsPerBlock:`**
  - Maximum number of transactions that can be packed into a single block. Adjusting this can control block size and overall network throughput. Default is `512`.

- **`MemoryPoolMaxTransactions:`**
  - This field sets the upper limit of transactions that can be held in the memory pool awaiting inclusion in a block. By default, this is set to `50000`, ensuring that the pool can handle a large backlog without overwhelming the node.

### **5. Block Traceability**
- **`MaxTraceableBlocks:`**
  - Defines how far back in the blockchain history the node can trace transactions. Default is `2102400` blocks.

### **6. Hard Fork Configurations**
- **`HF_Aspidochelone:`**
  - Block height at which the Aspidochelone hard fork occurs.
    - MainNet: `1730000`
    - TestNet: `210000`

- **`HF_Basilisk:`**
  - Block height at which the Basilisk hard fork occurs.
    - MainNet: `4120000`
    - TestNet: `2680000`

- **`HF_Cockatrice:`**
  - Block height for the Cockatrice hard fork.
    - MainNet: `5450000`
    - TestNet: `3967000`

### **7. EpicPulse Distribution**
- **`InitialEpicPulseDistribution:`**
  - The total amount of EpicPulse initially distributed across the network. This field uses the smallest unit of EpicPulse, called "Datoshi."
  - Default: `50,000,000,000,000,000 Datoshi` (equivalent to `500,000,000 EpicPulse`).

### **8. Consensus Validators**
- **`ValidatorsCount:`**
  - This setting controls the number of validators participating in the consensus mechanism. Adjusting this affects decentralization and consensus strength. Default is `7`.

### **9. Standby Committee**
- **`StandbyCommittee:`**
  - A list of public keys for the standby committee members. These members are responsible for ensuring the stability of the network by serving as backup validators in case active validators go offline.

### **10. Seed Nodes**
- **`SeedList:`**
  - A list of seed nodes that your node can connect to in order to synchronize with the blockchain network. These nodes serve as entry points to the network, helping your node discover peers and begin operations. Below are the addresses for both MainNet and TestNet:

  **MainNet Seed Nodes:**
  - `mainnet1-seed.epic-chain.org:10111`
  - `mainnet2-seed.epic-chain.org:10111`
  - `mainnet3-seed.epic-chain.org:10111`
  - `mainnet4-seed.epic-chain.org:10111`
  - `mainnet5-seed.epic-chain.org:10111`

  **TestNet Seed Nodes:**
  - `testnet1-seed.epic-chain.org:20111`
  - `testnet2-seed.epic-chain.org:20111`
  - `testnet3-seed.epic-chain.org:20111`
  - `testnet4-seed.epic-chain.org:20111`
  - `testnet5-seed.epic-chain.org:20111`

---

### **Summary**

This configuration file is essential for setting up and running an EpicChain node. It ensures proper logging, efficient storage management, robust peer-to-peer network connectivity, and adherence to consensus protocol parameters. Understanding each of these fields allows for the fine-tuning of your node to meet network requirements and operational goals effectively.
