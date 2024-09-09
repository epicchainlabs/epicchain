## 🐞 Bug Report

Thank you for helping us improve EpicChain by reporting a bug. This form is designed to collect all the relevant information needed to reproduce, investigate, and resolve any issues you're encountering. Please be as detailed and clear as possible when filling out this template. The more information you provide, the faster we can diagnose and address the problem.

---

### **Title of the Bug**
*Provide a brief, descriptive title summarizing the issue.*
Example: `Smart contract deployment fails under specific conditions in version 2.1.0`

---

### **Bug Description**
*Describe the bug in detail, including what is happening and what you believe should be happening instead. If possible, explain the impact this issue has on your work or the behavior of the system.*

Example:
> When attempting to deploy a smart contract using the EpicChain CLI, the deployment fails with an error message stating that the contract is invalid, even though the syntax is correct and it complies with all documented requirements.

---

### **Steps to Reproduce the Bug**
*Please list out the specific steps or actions that reliably reproduce the issue. Include as much detail as possible so that others can follow these steps to observe the same behavior.*

1. Clone the project from the repository.
2. Navigate to the `contracts` folder and attempt to compile the sample contract by running `epicchain-cli compile`.
3. Once compiled, run the `epicchain-cli deploy` command to deploy the contract.
4. Observe the error message displayed in the console output.

---

### **Expected Behavior**
*Describe what you expected to happen when performing the actions described above. This helps clarify the problem and allows for comparison between actual and intended outcomes.*

Example:
> The smart contract should successfully compile and be deployed to the blockchain without any errors.

---

### **Screenshots and Logs**
*If possible, please include screenshots, error logs, or any other visual aids to help illustrate the issue. This information is crucial for understanding the context of the bug and can expedite the debugging process.*

Example:
![Screenshot of Error Message](link-to-screenshot)

---

### **Platform Information**

- **Operating System (OS):** [e.g., Windows 10 x64, macOS 12.1, Ubuntu 20.04]
- **EpicChain Version:** [e.g., epicchain-cli v2.10.2, EpicChain Core v1.5.3]
- **Hardware Configuration:** [Optional, but helpful—e.g., Intel i7, 16GB RAM, SSD]

Please make sure your system is up-to-date and check if the issue persists across multiple environments (if applicable).

---

### **(Optional) Additional Context**
*If there’s any other context or relevant details that could aid in diagnosing the issue, please include it here. This could include the configuration of your system, any custom settings, or edge cases where the issue arises.*

Example:
> The issue only seems to occur when the smart contract involves large arrays. Small contracts are deployed without any issues.

---

### **Help Us Help You**
If your bug report does not neatly fit into the fields provided above or requires a more flexible structure, feel free to submit it in any format that best conveys the issue. We appreciate any and all information that leads us to better understand the problem and work toward a solution.

---

Thank you for taking the time to submit this report! We’ll review it promptly and may reach out if we need more details to resolve the issue.
