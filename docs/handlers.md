## **EpicChain Core Events**

EpicChain's core events are the foundational mechanisms that trigger various blockchain operations and interactions. By utilizing a well-structured event-based architecture, developers can integrate custom functionalities and ensure that specific actions are performed during critical stages of the blockchain lifecycle.

### **1. Block Committing Event**
---
#### Event Name: `Committing`
- **Handler Interface:** `ICommittingHandler`

**Description:**
This event is initiated when a transaction begins the process of being committed to the blockchain. It marks the critical moment where data is validated, executed, and prepared for addition to the block. By implementing the `ICommittingHandler` interface, developers can define custom behaviors, such as pre-validation checks, business logic execution, or external system notifications, ensuring that all relevant processes are executed during this stage.

### **2. Block Committed Event**
---
#### Event Name: `Committed`
- **Handler Interface:** `ICommittedHandler`

**Description:**
After a transaction has been successfully added to the blockchain, the `Committed` event is triggered. This is the final confirmation that the transaction is immutable and secure within the block. By using the `ICommittedHandler` interface, you can integrate post-commit actions, such as updating user balances, sending confirmations, or interacting with external systems. This is ideal for executing follow-up actions that rely on transaction finality.

### **3. Logging Event**
---
#### Event Name: `Logging`
- **Handler Interface:** `ILoggingHandler`

**Description:**
Blockchain operations are extensive, and tracking them is essential. The `Logging` event allows developers to implement custom logging strategies using the `ILoggingHandler` interface. It offers an opportunity to capture details of actions, errors, and performance metrics at various stages of blockchain operations, providing valuable insights for auditing and debugging.

### **4. General Log Event**
---
#### Event Name: `Log`
- **Handler Interface:** `ILogHandler`

**Description:**
The `Log` event functions as a generalized mechanism for recording specific activities across the blockchain system. Developers can leverage the `ILogHandler` interface to ensure consistent and accurate logging of specific actions or unexpected errors, helping to monitor the blockchain health and activity.

### **5. Notification Event**
---
#### Event Name: `Notify`
- **Handler Interface:** `INotifyHandler`

**Description:**
Notifications play a vital role in keeping users and systems informed about significant events in the blockchain network. The `Notify` event allows developers to specify when and how notifications should be triggered. By implementing the `INotifyHandler` interface, you can define custom logic for sending real-time alerts for activities like completed transactions, system updates, or contract executions.

### **6. Service Added Event**
---
#### Event Name: `ServiceAdded`
- **Handler Interface:** `IServiceAddedHandler`

**Description:**
The `ServiceAdded` event signifies the introduction of a new service or functionality to the blockchain system. With the `IServiceAddedHandler` interface, developers can automate tasks such as service registration, integration with other blockchain components, and activation processes, ensuring that the new service is seamlessly incorporated into the network.

### **7. Transaction Added Event**
---
#### Event Name: `TransactionAdded`
- **Handler Interface:** `ITransactionAddedHandler`

**Description:**
Whenever a new transaction is introduced to the blockchain, the `TransactionAdded` event is fired. This is a crucial checkpoint for executing preliminary actions before the transaction is committed. By implementing the `ITransactionAddedHandler` interface, developers can set up initial actions like transaction monitoring, pre-processing, or validation checks.

### **8. Transaction Removed Event**
---
#### Event Name: `TransactionRemoved`
- **Handler Interface:** `ITransactionRemovedHandler`

**Description:**
The removal of a transaction from the blockchain system can be triggered by various conditions, such as transaction conflicts or invalidation. The `TransactionRemoved` event allows developers to implement corrective actions, rollbacks, or system notifications through the `ITransactionRemovedHandler` interface, ensuring the integrity of the system remains intact.

### **9. Wallet Changed Event**
---
#### Event Name: `WalletChanged`
- **Handler Interface:** `IWalletChangedHandler`

**Description:**
Whenever a significant change occurs within a wallet (e.g., balance updates, transactions received or sent), the `WalletChanged` event is triggered. Developers can use the `IWalletChangedHandler` interface to implement custom actions like sending notifications to users, updating the UI, or syncing wallet states across multiple platforms.

### **10. Remote Node Message Received Event**
---
#### Event Name: `MessageReceived`
- **Handler Interface:** `IMessageReceivedHandler`

**Description:**
The decentralized nature of blockchain often involves communication between peer nodes. The `MessageReceived` event is triggered whenever a new message is received from a remote node, facilitating peer-to-peer communication within the network. By implementing the `IMessageReceivedHandler` interface, you can handle inter-node messages, validate them, and execute any necessary actions to maintain network cohesion and security.

---

### **Summary**
The above events in EpicChain provide the foundation for creating a flexible and customizable blockchain experience. By leveraging these events through their respective handler interfaces, developers can seamlessly integrate specialized logic that addresses both functional and business needs. This level of customization empowers developers to craft a blockchain ecosystem that aligns perfectly with their specific requirements.

