---
title: Consensus Algorithm
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';




## Terms and Definitions

To understand the dBFT 2.0 algorithm, it's essential to familiarize oneself with key terms within the EpicChain ecosystem:

- **Consensus Node**: Nodes with the privilege to propose and vote on new blocks.
- **Normal Node**: Nodes engaging in transfers and transactions without block proposing or voting capabilities.
- **Speaker**: A validator tasked with creating a proposal block and broadcasting it across the network.
- **Delegate**: Validators responsible for voting on the block proposal.
- **Candidate**: Accounts nominated for validator elections.
- **Validator**: Accounts elected from candidates to participate in the consensus process.
- **View**: Refers to the dataset used during a consensus round. The View number starts from 0 in each round, incrementing with each consensus failure until block proposal approval.

## Consensus Messages

In the dBFT 2.0 mechanism, six types of consensus messages facilitate communication:

- **Prepare Request**: Initiates a new consensus round.
- **Prepare Response**: Signals the collection of necessary transactions for block creation.
- **Commit**: Indicates enough Prepare Response messages have been collected.
- **Change View Request**: Attempts view change in case of consensus hurdles.
- **Recovery Request**: Synchronizes consensus data among nodes.
- **Recovery Message**: Responds to a Recovery Request message.

## 3-Stage Consensus Flow

The consensus round comprises four pivotal steps:

1. The **Speaker** broadcasts a **Prepare Request** message to begin consensus.
2. **Delegates** respond with **Prepare Response** messages upon receiving the Prepare Request.
3. **Validators** issue **Commit** messages following sufficient Prepare Responses.
4. Upon collecting enough **Commit** messages, validators create and broadcast a new block, resetting the consensus process.

## Change View Request

Change View Requests are necessitated under certain conditions, such as transaction verification failures or time-outs during the Prepare Request or Prepare Response phase. It triggers a protocol attempting to replace the current speaker to resolve the impasse.

## Recovery Mechanism

EpicChain employs a nuanced recovery mechanism, including **Recovery Request** and **Recovery Message** exchanges, to ensure consensus protocol resilience against node failures, network issues, and malicious activities.

## Fault Tolerance and Block Finality in dBFT 2.0

EpicChain’s dBFT 2.0 achieves significant milestones in consensus protocols:

- It can accommodate up to ⌊(N-1)/3⌋ faulty nodes within a system, maintaining functionality and stability.
- dBFT 2.0 introduces the concept of **Single Block Finality**, eliminating the possibility of forks, a notable improvement over dBFT 1.0, and ensuring unequivocal assurance of transaction finality upon block generation.

## Consensus Policy and the dBFT 2.0 Lifecycle

A comprehensive **Consensus Policy** underpins the protocol, guiding transaction verification, view changes, and transaction selections for block proposals. This policy ensures that only transactions complying with network consensus and policy rules are processed, maintaining network integrity and preventing malicious activities.

## Conclusion

EpicChain’s dBFT 2.0 consensus mechanism represents a paradigm shift towards more sustainable, secure, and efficient blockchain operation. By expertly navigating the challenges of decentralized consensus with innovations like the three-stage consensus flow, recovery mechanisms, and the benchmark of Single Block Finality, dBFT 2.0 positions EpicChain at the forefront of blockchain technology, ready to accommodate a new era of distributed digital systems.






<br/>