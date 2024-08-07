---
title: Permissioned Blockchain
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# EpicChain Node Deployment Guide

## Introduction

Welcome to the EpicChain Node Deployment Guide! This comprehensive guide will walk you through the process of setting up EpicChain nodes on your server, including installing the EpicChain client, .NET Core Runtime, downloading the EpicChain-CLI program, installing plugins, and modifying configuration files.

## Step-by-Step Deployment Process

### 1. Install EpicChain Client

Download the EpicChain client package suitable for your server's operating system from the official EpicChain website or repository. Use the package manager appropriate for your operating system to install the EpicChain client. For example, on Ubuntu:

```bash
wget https://example.com/epicchain-client.deb
sudo dpkg -i epicchain-client.deb
```

### 2. Install .NET Core Runtime

EpicChain-CLI requires the .NET Core Runtime version 5.0 or later. Download and install the .NET Core Runtime package for your server's operating system. For example, on Ubuntu:

```bash
wget https://example.com/dotnet-runtime.deb
sudo dpkg -i dotnet-runtime.deb
```

### 3. Download EpicChain-CLI

Clone the EpicChain-CLI repository from GitHub to your server:

```bash
git clone https://github.com/epicchain/epicchain-cli.git
cd epicchain-cli
```

### 4. Enable EpicChain Node

Use the EpicChain-CLI to enable the EpicChain node on your server. This command will start the necessary services for the node to function:

```bash
./epicchain-cli enable-node
```

### 5. Install Plugins

Download the required plugins from the EpicChain repository and unzip them into the Plugins folder in the EpicChain-CLI directory:

```bash
wget https://example.com/plugins.zip
unzip plugins.zip -d epicchain-cli/Plugins
```

### 6. Install Plugins Using CLI Commands

Alternatively, you can install the plugins using the EpicChain-CLI commands. Use the following commands to install the required plugins:

```bash
./epicchain-cli install ApplicationLogs
./epicchain-cli install LevelDBStore
./epicchain-cli install RpcServer
./epicchain-cli install TokensTracker
```

### 7. Modify Configuration Files

Before running the EpicChain-CLI, make necessary configurations in the configuration files. Open the config.json file in the EpicChain-CLI directory using a text editor and configure the settings as required:

```bash
nano epicchain-cli/config.json
```

### 8. Start EpicChain-CLI

Finally, start the EpicChain-CLI to enable your nodes. This will start the EpicChain node and allow it to communicate with the EpicChain network:

```bash
./epicchain-cli start
```

## Conclusion

By following these steps, you can successfully deploy EpicChain nodes on your server and manage them using the EpicChain-CLI. For more detailed instructions and best practices, please refer to the official EpicChain documentation.








<br/>