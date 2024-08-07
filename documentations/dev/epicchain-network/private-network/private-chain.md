---
title: Private Chain
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';









# Setting Up a Private Chain with One Node on EpicChain

Creating a private chain with a single node on EpicChain can be invaluable for developers looking to experiment or test functionalities in isolation. This guide simplifies the process, from installation to operation, ensuring a seamless experience in establishing your own private EpicChain environment.

## **Prerequisites**

Before diving into the setup, ensure you have EpicChain-CLI installed on your system. If you haven’t done so, please follow the [EpicChain-CLI Installation Guide](https://EpicChain.org/cli-installation). Also, downloading the DBFTPlugin is crucial for your node operation.

**Steps to Create Your Private Chain:**

### **1. Wallet Creation**

Launch EpicChain-CLI and create a wallet using:
```
create wallet <path_to_your_wallet>/consensus.json
```
Set and confirm your wallet password, and make a note of the wallet's `pubkey`. This information will be critical moving forward.

### **2. Configuration Adjustments**

#### **Modifying `config.json`**

Navigate to your EpicChain-CLI directory, locate the `config.json` file, and apply the following adjustments:

- In "UnlockWallet", enter the path to your wallet and its password.
- Set `IsActive` to `true`.
- Make sure `ConsoleOutput` and `Active` are set to `true`.
- Adjust `ValidatorsCount` to `1`.
- In `StandbyCommittee`, insert the `pubkey` of your `consensus.json` wallet.

**Example `config.json`:**
```json
{
  "ApplicationConfiguration": {
    "Logger": { "Path": "Logs", "ConsoleOutput": true, "Active": true },
    "Storage": { "Engine": "LevelDBStore", "Path": "Data_LevelDB_{0}" },
    "P2P": { "Port": 21333, "WsPort": 21334 },
    "UnlockWallet": { "Path": "consensus.json", "Password": "1", "IsActive": true }
  },
  "ProtocolConfiguration": {
    "Network": 5309138,
    "MillisecondsPerBlock": 15000,
    "MaxTraceableBlocks": 2102400,
    "ValidatorsCount": 1,
    "StandbyCommittee": ["Your_Wallet_Public_Key"]
  }
}
```

#### **Adjusting the DBFT Plugin `config.json`**

In the `Plugins\DBFTPlugin` directory, ensure the `AutoStart` is set to `true` and the network identifier matches that of the EpicChain-cli directory `config.json`.

**Example Plugin `config.json`:** 
```json
{
  "PluginConfiguration": {
    "AutoStart": true,
    "Network": 5309138
  }
}
```

### **3. Launching Your Private Chain**

Before initiating EpicChain-CLI, remove any existing `Data` folder if the node has previously downloaded the EpicChain testnet block files. This ensures your private chain starts afresh.

To start, either run `EpicChain-cli.exe` directly from the EpicChain-cli directory or execute `dotnet EpicChain-cli.dll` from the command line. Successful setup is indicated by the synchronization of blocks and activation of your private chain.

### **4. Accessing EpicChain and EpicPulse**

Upon initializing your private chain, 100 million EpicChain and 30 million EpicPulse tokens are generated. Follow these steps to access them:

- **Setup an External Node**: Duplicate your EpicChain-CLI directory and adjust the `SeedList` in the external node's `config.json` to include your consensus node TCP address (`localhost:21333`). Ensure its `P2P` port is distinct to avoid conflict.
- **Creating and Utilizing a Multi-signature Address**: Using the `import multisigaddress` command, create a multi-signature address with `pubkeys` as your `consensus.json` wallet's public key.
- **Withdrawing Tokens**: From the external node, open the `consensus.json` wallet and transfer EpicChain and EpicPulse to a new wallet address using the `send` command. Remember, a small network fee in EpicPulse may apply.

**EpicChain-CLI Example Command:** 
```shell
send EpicChain <address> <amount>
send EpicPulse <address> <amount>
```
Congratulations! You have successfully set up a private chain with one node on EpicChain. This private environment is perfect for development, testing, and experimenting with blockchain technology in a controlled setting.

















<br/>