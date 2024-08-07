---
title: EpicChain Cli
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';



# In-Depth Guide to Installing and Setting Up EpicChain-CLI

Diving into the world of blockchain technology with EpicChain requires a powerful and versatile toolchain, and the EpicChain Command-Line Interface (CLI) is at the forefront of this technological expedition. The CLI serves as your gateway to the EpicChain ecosystem, providing a comprehensive suite of functionalities that enable developers and users alike to interact seamlessly with the blockchain, manage wallets, deploy smart contracts, and much more.

## Ways to Install EpicChain-CLI

Embarking on your EpicChain journey involves setting up the EpicChain-CLI on your system. You have two paths to achieve this:

1. **Install the Officially Released EpicChain-CLI Package**: This method is straightforward and recommended for users who wish to get started quickly without diving into the compilation process.

2. **Compile and Publish EpicChain-CLI from Source Code**: For those who prefer a hands-on approach or use macOS, compiling from source ensures the latest features and updates directly from the development repository on GitHub.

This document delves into both installation methods, ensuring you find the path that best suits your needs.

## Hardware Requirements: Crafting Your Digital Forge

Before you embark on the installation, it's essential to ensure your system meets the necessary hardware specifications. The table below outlines the minimum and recommended hardware requirements to run EpicChain-CLI effectively.

|                | **Minimum**                          | **Recommended**                    |
|----------------|--------------------------------------|------------------------------------|
| **System**     | Windows 10, Ubuntu 16.04/18.04, CentOS 7.4/7.6 | Windows 10, Ubuntu 16.04/18.04, CentOS 7.4/7.6 |
| **CPU**        | Dual Core                            | Quad Core                          |
| **Memory**     | 8GB                                  | 16GB                               |
| **Hard Disk**  | 50GB SSD                             | 100GB SSD                          |

## Installing EpicChain-CLI Package

Whether you're operating within the realms of Windows, Linux, or macOS, downloading the latest EpicChain-CLI package is your first step. Navigate to the [EpicChain-CLI Releases](https://github.com/epic-chain/cli/releases) on GitHub and select the version that aligns with your operating system.

### Special Steps for Linux Users:

Linux enthusiasts need to prepare their systems by installing essential packages such as LevelDB and SQLite3. For Ubuntu users, execute:

```bash
sudo apt-get install libleveldb-dev sqlite3 libsqlite3-dev libunwind8-dev
```

CentOS warriors can beckon the requisite libraries with:

```bash
sudo yum -y install leveldb-devel libunwind-devel libsqlite3x-devel sqlite3*
```

Additionally, if opting for RocksDB as the storage engine, modify `config.json` to use `RocksDBStore` and install the necessary package on Ubuntu 18.04 with:

```bash
sudo apt-get install librocksdb-dev
```

Windows users have a slightly easier path; the installation package includes all necessary files.

## Publishing from EpicChain-CLI Source Code

For those preferring to wield the tools of creation themselves, compiling the EpicChain-CLI from source allows for the most updated and customizable experience. Begin by cloning the EpicChain-CLI repository with:

```bash
git clone https://github.com/epic-chain/node.git
```

### Required Installations:

- **LevelDB**: Ensure LevelDB is downloaded and prepared for use.
- **.NET Core Runtime**: Equip your system with the latest version of the .NET Core Runtime for seamless compilation.

### Publishing with Visual Studio (Windows):

Developers armed with Visual Studio 2022 have a streamlined path. Load the `EpicChain-node.sln` project and publish `EpicChain-cli` directly from the IDE. Remember to ensure that `libleveldb.dll` is copied to the output directory.

### Command Line Publishing (Cross-Platform):

For those who traverse the terminal's depths, the .NET Core CLI facilitates the publishing process across all supported platforms, using:

```bash
cd epicchain\epicchain-cli
dotnet restore
dotnet publish -c release -r <RUNTIME_IDENTIFIER>
```

Replace `<RUNTIME_IDENTIFIER>` with your system's specific platform RID, such as `win-x64`, `linux-x64`, or `osx-x64`. Upon completion, copy `libleveldb.dll` to the output directory for a fully operational setup.

## Next Steps: Operational Mastery

With EpicChain-CLI installed, the next crucial steps involve configuring and initializing `EpicChain-cli` to fully harness the might of the EpicChain ecosystem. This journey into blockchain technology with EpicChain is not just about installing software; it's about equipping yourself with the tools and knowledge to innovate, secure, and transform the digital landscape.

Embark on this epic adventure with confidence, knowing that each command, each line of code, brings us closer to a decentralized, transparent, and empowered future. Welcome to EpicChain.



# A Voyage Through Configuration and Launch

Welcome, trailblazers, to the bridge of the EpicChain-CLI, your command center. Herein lies the map to maneuvering through the intricate layers of technology that power the EpicChain ecosystem. As you embark on this journey, you are not merely configuring a tool; you are sculpting the digital clay that will form the basis of transactions, smart contracts, and decentralized applications of tomorrow. Let’s unfurl the sails and delve into the sanctum of configuring and initiating the EpicChain-CLI.

## Navigating Through Configuration Files

Imagine the `config.json` as the compass of EpicChain-CLI, guiding it through the digital ethers. Your first quest is to adeptly tune this compass.

### **Charting the Wallet’s Coordinates**

Configuring your EpicChain-CLI to automatically acquaint itself with your wallet is akin to setting the North Star in your night sky. This beacon in `config.json` ensures a steadfast journey through EpicChain seas.

```json
"UnlockWallet": {
  "Path": "path/to/your/wallet.json",
  "Password": "your_wallet_password",
  "IsActive": true
}
```

**Key to the Treasure:**

- `Path`: The treasure map leading to your wallet.
- `Password`: The key to your treasure chest. Guard it well, for it is exposed in plaintext.
- `IsActive`: Set this to `true` to let your CLI hoist the sails at dawn automatically.

**The Compass Configurations:**

Navigate further into `config.json` to discover:

```json
{
  "ApplicationConfiguration": {
    /* Your Personal Configuration */
    "UnlockWallet": {
      "Path": "path/to/your/wallet.json",
      "Password": "YourSecretPassword",
      "IsActive": true
    }
  },
  "ProtocolConfiguration": {
    /* Protocol Details */
    "Network": 860833102,
    "AddressVersion": 76,
    "MillisecondsPerBlock": 5000,
    /* Further Sailing Instructions */
  }
}
```

## **Connecting to the EpicChain Mainland and Beyond**

- By default, your vessel is anchored in the majestic harbors of the N3 main net. To sail the test net seas, exchange the content of `config.json` with that of `config.testnet.json`.
- For those captains wishing to chart their private waters, the "Setting up a Private Chain" atlas is your guide.

## **Augmenting Your Vessel: Plugins**

Enhance your ship with plugins from the EpicChain armory. Each plugin is a unique artifact, brimming with magic to enhance your node’s capabilities.

### **Treasure Hunting for Plugins:**

1. **From the GitHub Isles:** Fetch the desired plugins and gently place them in the holds of your EpicChain-cli directory.
   
2. **Commanding the CLI to Fetch:** With a mere utterance, command your CLI to fetch or cast away plugins into the sea of digital oblivion.

   **Enchantments to Install:**
   ```shell
   EpicChain> install PluginName
   ```

   **Enchantments to Uninstall:**
   ```shell
   EpicChain> uninstall PluginName
   ```

Welcoming the plugins aboard and giving them quarters restarts the heart of your EpicChain-CLI, breathing life into the newly integrated functions.

## **Embarking on Your Journey**

At the crack of dawn, with your ship laden with configurations and plugins, the moment to set sail arrives. Command your vessel to life:

**On the Shores of Windows 10:**
```shell
dotnet EpicChain-cli.dll
```
Or simply pronounce,
```shell
epicchain-cli.exe
```

**In the Linux Highlands (Ubuntu 16.04/18.04):**
```shell
./epicchain-cli
```
Or invoke the spirits of .NET,
```shell
dotnet epicchain-cli.dll
```

**Guarding Your Troves:**
As you traverse the digital realms, your treasure—secured within the API vaults and wallet caverns—must be safeguarded. Erect magical barriers and selective passageways (firewall configurations) to protect your riches.

With the configurations set and the wind favorable, your EpicChain-CLI is now a formidable galleon ready to conquer the high seas of blockchain. Each command you input steers you closer to uncharted territories ripe for discovery, invention, and digital prosperity.

Bon Voyage, intrepid navigator of the EpicChain! May your journey be fruitful, and your explorations bring forth new realms of possibilities.



# The EpicChain-CLI Navigator's Guide

Embark on a digital odyssey with the EpicChain-CLI Navigator's Guide, your comprehensive atlas through the boundless realms of the EpicChain network. Within this guide, you'll find not merely commands, but the keys to unlock the mysteries of the blockchain, manage assets with unparalleled finesse, and command the forces of smart contracts to do your bidding. Each command is a rune, laden with power, waiting for you, the intrepid explorer, to harness its potential.

## Embarking on Your Journey

At the command prompt of your vessel, the EpicChain-CLI, you initiate the launch sequence into the blockchain cosmos:

```bash
dotnet epicchain-cli.dll
```

This incantation prepares you to unlock an armory of commands, each designed to navigate the blockchain, manage digital treasures, interface with the arcane arts of smart contracts, and decipher the enigms of consensus and governance within the EpicChain universe.

## The Cosmos of Commands

Behold the celestial bodies of commands, each a star in the vast EpicChain galaxy:

### The Elemental Commands
- `version`: Unveil the era of your EpicChain-CLI galaxy.
- `help [plugin-name]`: Your guiding light through nebulous doubts.
- `clear`: Banish storms, offering clarity on your console's horizon.
- `exit`: When the call to return home becomes irresistible.

### The Treasury Commands
- `create wallet <path>`: Summoning a vault from the cosmic ether.
- `open wallet <path>`: Unsealing arcane secrets of your digital vault.
- **Voyager's Ledger**:
    - `list address`: The catalog of all ensigns under your flag.
    - `create address [count=1]`: Bringing forth sanctuaries for your digital assets into existence.

### The Arcanum of Contracts
- `deploy <nefFilePath> [manifestFile]`: Launch your inventions into the blockchain firmament.
- `invoke <scripthash> <operation> [parameters]`: Commanding the entities of the smart contract universe.

### The Observatorium
- `show state`: Peer into the oracle, discerning the essence of the blockchain reality.
- `show pool [verbose=False]`: Gaze into the maelstrom of transactions in awaiting confirmation.

### The Voyager's Network Commands
- `relay <jsonObjectToSign>`: Cast your intentions over the network, akin to setting sails for distant lands.

### Delving Deeper into the Celestial Mysteries

Advance deeper into the EpicChain cosmos with commands that bridge realms, govern the ethers of blockchain governance, and channel the elemental forces of smart contracts.

- `broadcast transaction <transaction hash>`: Proclaiming your intentions across the realms, ensuring every entity bears witness.
- `plugins`: A compendium of magical augmentations at your behest, each amplifying your command over the EpicChain-CLI vessel.
- `install [Plugin name]`: In quest of artifacts unknown, enhancing your voyager's capabilities and understanding.

## Divine Commandments and Runes

This guide bestows upon you the wisdom to interpret the commandments:

- `<>`: A marker, guiding you to inscribe a parameter.
- `[]`: The echo of optional parameters, whispered by the ancients.
- `|`: The crossroads of fate, presenting choices of paths to tread.
- `=`: The revelation of hidden truths, the default value when silence prevails.


Given the EpicChain-CLI's nature, it would be inherently developed to interact with a blockchain, likely through command-line operations and possibly connecting with blockchain nodes or APIs. While C# isn't typically used for writing Solidity smart contracts, it is entirely feasible for blockchain-related CLI tools, especially those interacting with .NET applications or needing to interface with blockchain nodes through RPC or REST APIs.

Let’s illustrate with some foundational C# example source code snippets for the specified commands:

### 1. `version`: Unveil the Era of the EpicChain-CLI Galaxy

**Command Description**: Reveals the current version of the EpicChain-CLI.

**Example Usage**:
```bash
epicchain-cli version
```

**Hypothetical Source Code Snippet**:
```csharp
public class VersionCommand
{
    public void Execute()
    {
        Console.WriteLine("EpicChain-CLI Version: 1.0.0");
    }
}
```

### 2. `create wallet <path>`: Summoning a Vault from the Cosmic Ether

**Command Description**: Creates a new wallet file at the specified path.

**Example Usage**:
```bash
epicchain-cli create wallet "./user/wallet.json"
```

**Hypothetical Source Code Snippet**:
```csharp
public class WalletCommand
{
    public void CreateWallet(string path)
    {
        Wallet newWallet = Wallet.Create();
        string walletJson = JsonConvert.SerializeObject(newWallet);
        File.WriteAllText(path, walletJson);
        Console.WriteLine($"Wallet successfully created at {path}");
    }
}
```

### 3. `invoke <scripthash> <operation> [parameters]`: Commanding the Entities of the Smart Contract Universe

**Command Description**: Invokes a function of the smart contract identified by its script hash.

**Example Usage**:
```bash
epicchain-cli invoke 0x123ab...def "transfer" "[\"0xabc123...efg\", \"0xdef456...hij\", 100]"
```

**Hypothetical Source Code Snippet**:
```csharp
public class ContractCommand
{
    private readonly IBlockchainService _blockchainService;

    public ContractCommand(IBlockchainService blockchainService)
    {
        _blockchainService = blockchainService;
    }

    public async Task InvokeAsync(string scripthash, string operation, string parametersJson)
    {
        var contract = await _blockchainService.GetContractAsync(scripthash);
        if (contract == null)
        {
            Console.WriteLine("Contract not found.");
            return;
        }

        var parameters = JsonConvert.DeserializeObject<object[]>(parametersJson);
        var result = await contract.InvokeAsync(operation, parameters);

        Console.WriteLine($"Invoke result: {result}");
    }
}
```
In these snippets, `IBlockchainService` is an interface that abstracts the operations for fetching contract details and invoking them, which would be implemented with the logic specific to how EpicChain networks are accessed and interacted with. The `Wallet` and `ContractCommand` classes provide a high-level overview of how one might structure commands in a CLI application written in C#.


The provided text outlines a series of commands integral to interacting with the EpicChain network through a CLI (Command Line Interface). These commands facilitate various blockchain operations, such as creating addresses, querying balances, transferring tokens, and managing smart contracts. Below is an expanded explanation of these commands:

### Create Address Command
**Command**: `create address [n]`

This command generates a new blockchain address. Optionally, you can specify the number `n` to create multiple addresses at once. Created addresses are conveniently stored in an `address.txt` file.

**Example**: 
```
epicchain> create address 3
```
This creates three new addresses and saves them to `address.txt`, asking for file overwrite confirmation if the file already exists.

### BalanceOf Command
**Command**: `balanceof <tokenHash> <address>`

Queries and displays the balance of a specified token (identified by its `tokenHash`) at a given address. This is essential for checking how much of a particular token an account holds.

**Example**:
```
epicchain> balanceof 0xd2c270ebfc2a1cdd3e470014a4dff7c091f699ec NcphtjgTye3c3ZL5J5nDZhsf3UJMGAjd7o
```
Fetches the balance of the specified token at the provided address, showcasing the power of blockchain in tracking asset distribution.

### Decimals Command
**Command**: `decimals <tokenHash>`

Retrieves the decimal precision of a specified token. This information dictates how divisible a token is and is crucial for transactions and displaying balances correctly.

**Example**:
```
epicchain> decimals 0xd2c270ebfc2a1cdd3e470014a4dff7c091f699ec
```
Returns the number of decimal places used by the token, adding clarity to the amount representation of tokens.

### Transfer Command
**Command**: `transfer <tokenHash> <to> <amount> [from=null] [data=null] [signersAccounts=null]`

Executes a token transfer operation from one account to another. It offers optional parameters for specifying the sender, attaching data to the transaction, and defining signer accounts for additional validation.

**Example**:
```
epicchain> transfer 0xd2c270ebfc2a1cdd3e470014a4dff7c091f699ec Nhe4mzfQRoKojkXhqxJHjANvBMT7BYAXDv 6000 NNU67Fvdy3LEQTM374EJ9iMbCRxVExgM8Y transferdata NNU67Fvdy3LEQTM374EJ9iMbCRxVExgM8Y
```
Demonstrates how to transfer tokens, underscoring the ability to execute and record transactions on the blockchain.

### List Native Contract Command
**Command**: `list nativecontract`

Lists all the native contracts deployed on the blockchain, providing their names and script hashes. This is invaluable for developers and users needing to interact with these contracts.

**Example**:
```
epicchain> list nativecontract
```
Returns a list of system-level smart contracts and their identifiers, allowing for direct interaction and interrogation of these core blockchain functionalities.

Each of these commands, inherent to the CLI's interaction with the EpicChain, unlocks the vast potential embedded within the blockchain, from asset management and transfers to complex contract interactions. They demonstrate the CLI's powerful capabilities in managing blockchain operations efficiently.

### Get Account State Command
**Command**: `get accountstate <address>`

This command retrieves the current state and voting information for a specific account address, indicating the address's participation in the network consensus and governance.

**Example**:
```
epicchain> get accountstate NNz4ppADL3mke7HT8RvRr5nX8zTAbNdWjv
```
Fetches account details, including voting status and block height, showcasing the transparency and traceability features of blockchain governance.

### Get Candidates Command
**Command**: `get candidates`

Lists all current candidates for the blockchain's consensus mechanism, displaying their public keys and the number of votes each has received. This command emphasizes the decentralized decision-making process in blockchain networks.

**Example**:
```
epicchain> get candidates
```
Revealing those vying to be validators within the EpicChain network, this command underscores the participatory nature of blockchain governance.

### Get Committee Command
**Command**: `get committee`

Fetches the public keys of the current committee members who are part of the governance structure. This illustrates the active participants in the network's operational and governance decisions.

**Example**:
```
epicchain> get committee
```
Displays the committee members, offering insight into the network's current governance body.

### Get Next Validators Command
**Command**: `get next validators`

Retrieves a list of the public keys of the validators for the next consensus round. It shows the future validators determined through the governance process.

**Example**:
```
epicchain> get next validators
```
Showcases the upcoming validators, highlighting the dynamic nature of consensus participants in the network.

### Register Candidate Command
**Command**: `register candidate <account> [maxepicpulse=1010]`

Enables an account to register as a candidate for becoming a validator, subject to network governance rules. It specifies an optional maximum gas fee for the operation.

**Example**:
```
epicchain> register candidate NUNtEBBbJkmPrmhiVSPN6JuM7AcE8FJ5sE
```
Initiates a candidacy for network validation, indicating a user's intent to participate actively in network consensus.

### Unregister Candidate Command
**Command**: `unregister candidate <account>`

Withdraws an account's candidacy from the validator election process. This command is used when an account no longer wishes to participate as a potential validator.

**Example**:
```
epicchain> unregister candidate NUNtEBBbJkmPrmhiVSPN6JuM7AcE8FJ5sE
```
Cancels a validator candidacy, reflecting the flexible governance and participation framework of the blockchain.

### Vote Command
**Command**: `vote <senderAccount> <publicKey>`

Casts a vote from a sender account to a candidate identified by their public key. It's a fundamental aspect of participatory governance in a blockchain network.

**Example**:
```
epicchain> vote Nhiuh11SHF4n9FE6G5LuFHHYc7Lgws9U1z 02344389a36dfc3e95e05ea2adc28cf212c0651418cfcf39e69d19d18b567b221d
```
Exercises the voting right, contributing to the decentralized decision-making process by supporting a candidate for network validation.

### Unvote Command
**Command**: `unvote <senderAccount>`

Withdraws any votes cast by a sender account. This could be used when a voter changes their preference or wishes to abstain from the current governance cycle.

**Example**:
```
epicchain> unvote 0x39edfd1534af1ab7e91bab25fcee9a3b93bfae21
```
Reverses a previously cast vote, demonstrating the ability of participants to dynamically adjust their governance participation.

These commands collectively reveal the depth and flexibility of the EpicChain's CLI, offering users comprehensive tools for managing addresses, tokens, and smart contracts. Additionally, they facilitate active participation in the network's governance, encapsulating the decentralized and democratic ethos of blockchain technology.

### Export Key Command
**Command**: `export key [address] [path]`

Exports the private key of a specific address to a designated file path or prints it directly to the console if no path is provided. It's a crucial operation for securely managing and backing up access to blockchain assets.

**Example**:
```
epicchain> export key NPpH6FxNaVXZCrsecNWEHGLwMe87UkPdm5 key.txt
```
This would export the private key of the specified address into the file named `key.txt`, underlining the importance of secure key management.

### Import Key Command
**Command**: `import key <wif | path>`

Imports a private key or a set of private keys from a specified file or directly from a Wallet Import Format (WIF) string. This facilitates the restoration of access to blockchain accounts and assets.

**Example**:
```
epicchain> import key L4q37aCJzjEXhAUJ6npdxbjGGbyTXuWhpgYxkb2NWPmzXv4DdxiD
```
Actively restores access to an account by importing its private key, essential for account recovery or migration.

### Import MultisigAddress Command
**Command**: `import multisigaddress m pubkeys...`

Creates and imports a multisig address using `m` (the minimum number of signatures required) and a list of public keys. Multisig addresses enhance security by requiring multiple parties to authorize transactions.

**Example**:
```
epicchain> import multisigaddress 2 03fadbc9b25d1b6827124665c50801e602240c9d8ebdda2bae49de6683f8f86af9 02ff249d06faaf0b5ba865e1531bfabe07f89aef39ab59082e3bc140be0318055d
```
Demonstrates setting up a multisig address that needs at least two signatures out of the specified public keys to authorize transactions, bolstering security mechanisms for asset management.

### Import WatchOnly Command
**Command**: `import watchonly <addressOrFile>`

Adds an address as "watch-only" into the wallet, enabling the tracking of its transactions and balance without access to its private key. Useful for monitoring purposes without needing transaction capability.

**Example**:
```
epicchain> import watchonly NcphtjgTye3c3ZL5J5nDZhsf3UJMGAjd7o
```
Allows a user to keep an eye on a particular address's activity, fostering transparency and oversight without granting transactional control.

### Send Command
**Command**: `send <id | alias> <address> <amount> [from=null] [data=null] [signerAccounts=null]`

Executes the transfer of assets or tokens from one address to another, with options for specifying the sender, attaching additional data, and including signer accounts for multi-signature transactions.

**Example**:
```
epicchain> send epicchain Nhe4mzfQRoKojkXhqxJHjANvBMT7BYAXDv 100
```
Facilitates asset transfers within the network, highlighting the fluid movement of value between participants and contributing to the dynamic economic environment of the blockchain.

### Sign Command
**Command**: `sign <jsonObjectToSign>`

Signs a transaction or a piece of arbitrary data with the private key of an account in the wallet. This command is indispensable for manually constructing transactions or messages that require authentication.

**Example**:
```
epicchain> sign {"transaction":"..."}
```
Illustrates the manual signing process, empowering users to independently verify transactions or messages before broadcast, enhancing both security and flexibility.

These commands collectively enrich the end-user's ability to interact seamlessly with the EpicChain network, manage assets securely, participate in governance, and conduct transactions with confidence. Importantly, they underscore the CLI's role as a powerful interface for nuanced blockchain operations and management tasks.

### Deploy Command
**Command**: `deploy <nefFilePath> [manifestFile]`

Deploys a new smart contract to the blockchain network. This command requires the path to the compiled contract file (`.nef`) and optionally its manifest file, defining permissions, trusts, and other contract metadata.

**Example**:
```
epicchain> deploy myContract.nef myContract.manifest.json
```
Facilitates the transition of smart contracts from development to live operational status on the blockchain, marking an essential step in blockchain application deployment.

### Invoke Command
**Command**: `invoke <scriptHash> <operation> [contractParameters=null] [sender=null] [signerAccounts=null] [maxepicpulse]`

Executes a specified operation on a smart contract identified by its script hash, with an array of parameters, defining transaction specifics including the sender, signers, and gas constraints.

**Example**:
```
epicchain> invoke 0xd2c27...9ec transfer "[\"Nhe4m...AXDv\", \"Ncpht...GAjd7o\", \"100\"]"
```
Demonstrates direct interaction with smart contracts, allowing users to execute contract functions such as transfers, state updates, and more.

### Update Command
**Command**: `update <scriptHash> <filePath> <manifestPath> <sender> [signerAccounts]`

Allows for the updating of an existing smart contract on the blockchain. Parameters include the contract's script hash, updated contract file, and manifest, plus transaction details.

**Example**:
```
epicchain> update 0xf3d2c...2c3e4 updatedContract.nef updatedContract.manifest.json Nhe4m...AXDv
```
Enables the iterative development and enhancement of smart contracts post-deployment, ensuring flexibility and adaptability of blockchain applications.

### Relay Command
**Command**: `relay <jsonObjectToSign>`

Broadcasts a pre-signed transaction or message to the network. This command is crucial for transactions constructed or signed offline, adding a layer of security.

**Example**:
```
epicchain> relay {"type":"...","value":"..."}
```
Offers an advanced mechanism to securely introduce transactions into the network, catering to scenarios requiring higher security measures or transaction batching.

### Broadcast Commands
- **`broadcast transaction <transaction hash>`**: Announces a transaction to the network.
- **`broadcast block <block-hash | block-height>`**: Broadcasts a block or request for a block by height.

These commands facilitate network-wide dissemination of transactions and blocks, ensuring network consensus and data integrity.

### Plugins and Extensions
- **`plugins`**: Lists all currently loaded plugins.
- **`install [PluginName]`**: Installs a plugin by name, expanding the CLI's capabilities.
- **`uninstall [PluginName]`**: Removes a previously installed plugin.

Plugins extend the functionality of the EpicChain CLI, allowing for customizable and scalable blockchain solutions catering to specific network, governance, or application needs.

**Example**:
```
epicchain> plugins
```
Provides an overview of the CLI's extensibility through plugins, highlighting the adaptable architecture of the EpicChain framework.

---

These commands illuminate the depth and breadth of interactions possible with the EpicChain network through its CLI, spanning everything from development and deployment to governance and monitoring. Through the effective use of these commands, users can fully leverage the power and flexibility of the EpicChain platform, ensuring robust, secure, and efficient blockchain operations.

## In Retrospect

As you hold the EpicChain-CLI Navigator's Guide, remember, you are equipped to chart unexplored territories in the EpicChain universe. Navigate the digital seas with precision, conjure wealth and interactions with ease, and, most importantly, engage in the grand odyssey of innovation and discovery. The EpicChain cosmos is vast, fraught with mysteries, treasures, and legends yet to be written by none other than you, the esteemed navigator. Sail forth, for the blockchain horizon beckons with infinite possibilities.








<br/>