# Smart Contracts Repository

This repository contains a collection of smart contracts for various decentralized applications (dApps) on the Solana blockchain.

## Contracts Overview

### 1. Concentrated Liquidity AMM (`concentrated-liquidity-amm`)
A **concentrated liquidity automated market maker (AMM)** that allows liquidity providers to allocate capital within specific price ranges.  
- Optimizes capital efficiency.  
- Supports precise swaps and liquidity provisioning.  
- Built using Solana and Anchor framework.

### 2. Solana AMM (`solana-amm`)
A standard **automated market maker** on Solana.  
- Facilitates token swaps.  
- Supports liquidity provision and fee accrual.  
- Designed for fast, decentralized token trading.

### 3. Solana Escrow (`solana-escrow`)
An **escrow smart contract** for secure token transfers between parties.  
- Holds tokens safely until predefined conditions are met.  
- Enables trustless exchange between users.  
- Useful for marketplaces, P2P trades, and decentralized agreements.

### 4. Stake Contract (`stake-contract`)
A **staking smart contract** to allow users to stake tokens and earn rewards.  
- Supports creating staking accounts.  
- Tracks staked amounts and rewards.  
- Built with security and efficiency in mind.

## Getting Started

### Prerequisites
- Rust and Solana Tool Suite installed.  
- Anchor framework installed.  
- Node.js and npm (for frontend integration if needed).  

### Installation
1. Clone the repository:
```bash
git clone https://github.com/yourusername/smart-contracts.git
cd smart-contracts

```
2.Run the programs
```bash
cd programs/<program-name>
anchor build

