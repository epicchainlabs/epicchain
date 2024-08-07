---
title: Overview
displayed_sidebar: mainSidebar
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';





## Overview

Blockchain technology, originally introduced by Satoshi Nakamoto with Bitcoin, has evolved beyond e-cash systems to more complex applications like stock equity exchanges and Smart Contract systems. EpicChain builds upon this legacy, addressing inherent challenges faced by distributed systems through an innovative consensus mechanism known as delegated Byzantine Fault Tolerance (dBFT).

## The Vital Need for Consensus

Given blockchain's decentralized nature, ensuring consistency across all participant nodes is paramount. The challenges include network latency, potential security vulnerabilities, and the presence of malicious nodes. Traditional fault tolerance mechanisms fall short in providing a comprehensive solution for these distributed system-specific issues.

Proof-of-Work (PoW), while effective, is resource-intensive. Consequently, emerging blockchains have sought alternative hashing algorithms to maintain security without the environmental cost. Building on these principles, EpicChain's dBFT consensus model introduces efficiency and robustness by leveraging real-time voting for validator selection, thus optimizing transaction confirmation times and achieving significant block time reductions.

## System Model and Roles

EpicChain's distributed ledger system operates on a peer-to-peer network where messages are broadcasted, and nodes can be categorized into two types: Ordinary nodes and Bookkeeping nodes. Ordinary nodes engage in transfers and transactions, while Bookkeeping nodes, or validators, maintain the ledger and provide accounting services for the network.

This dual-node model allows for a flexible network where integrity and authenticity are maintained through cryptographic measures, despite potential disruptions such as message loss, latency, or node malfunctions.

## The dBFT Algorithm

dBFT stands out by enabling a system to function and maintain stability with up to ⌊ (N−1) / 3 ⌋ erroneous nodes (where N is the total number of consensus nodes). It achieves this through a unique approach to consensus making, requiring each consensus node to maintain a state table and manage data sets or 'Views' for successful consensus achievement.

View changes are initiated when consensus cannot be reached, with each View identified numerically. Consensus nodes are also numerically identified, allowing for a systematic selection process for the "speaker" role, essential for driving consensus in each round.

## Consensus Procedure and Fault Tolerance

dBFT2.0 improves upon its predecessor by introducing three stages of consensus and a recovery mechanism, ensuring the robustness and safety of the network. A consensus system with N validators can tolerate F abnormal nodes, requiring at least M nodes for consensus continuation.

Furthermore, dBFT2.0 addressed the single block fork susceptibility inherent in dBFT 1.0 through a definitive finality rule: a new block's generation necessitates Commit messages from at least M validators, ensuring no view change post-Commit and preventing fork possibilities.

## Single Block Finality and Network Resilience

EpicChain’s dBFT2.0 amendment ensures single block finality, effectively eliminating fork risks and bolstering the network's resilience. The consensus mechanism mandates that a new block require Commit messages from a majority of validators, ensuring an unequivocal finality at every block height.

## Conclusion

EpicChain's adoption of the dBFT2.0 consensus mechanism marks a pivotal advancement in blockchain technology. By addressing critical issues like energy consumption, network latency, and security vulnerabilities, dBFT2.0 paves the way for a more sustainable, efficient, and secure distributed ledger system. This advancement not only enhances EpicChain's performance but also sets a new standard for blockchain consensus mechanisms, promoting a more robust and reliable ecosystem for future blockchain innovations.




<br/>