# Algorithmic Stablecoin Program

## Overview

This program implements an algorithmic stablecoin system. The key idea is to maintain a peg (e.g., to USD) via algorithmic supply-adjustment / mint & burn mechanisms rather than relying purely on full fiat-backed collateral. The design aims for high capital efficiency while enforcing price stability via on-chain logic.

## Features

* Minting and burning of the stablecoin token by users under defined conditions.

* A dual-token or auxiliary mechanism (governance/share token) that absorbs volatility or enables seigniorage-type dynamics.

* Supply expansion when price is above peg, contraction when price is below peg.

* On-chain governance and parameter update logic (e.g., peg target, expansion rate, contraction limits).

* Audit-friendly implementation in Rust with clear modules for token, treasury/reserve, and policy logic.

## Getting Started

### Prerequisites


* Rust toolchain (e.g., `rustup` , `cargo`).

* Solana CLI (if targeting Solana chain) and Anchor framework (if using Anchor).

* Network environment (devnet/testnet/mainnet) configured.

## Installation

```bash 
git clone https://github.com/openforgelabshq/stablecoin.git  
cd stablecoin/program  
cargo build-bpf   # or the appropriate build command for your chain  
```
## Deployment

1. Configure program ID in Anchor.toml (if using Anchor).

2. Deploy the program:
   ```bash
   anchor deploy
   ```
3. Initialize the system with the setup instruction to define parameters, treasury, and governance accounts.


## Usage
### Core Instructions

* *Initialize* — Set up initial parameters, token mints, and treasury accounts.

* *Mint Stablecoin* — Trigger minting when market price > 1 (expansion phase).

* *Burn Stablecoin* — Reduce supply when market price < 1 (contraction phase).

* *Update Parameters* — Governance can modify rates, fees, or peg target.

* *Treasury Operations* — Manages reserves, share token distribution, and seigniorage.

## Testing 
```bash
cargo test
# or if using Anchor
anchor test
```
## Configuration

* Anchor.toml / Environment Parameters

* Program ID

* Token mint addresses

* Treasury and reserve accounts

* Peg target, expansion/contraction rates

* Governance authority address


## License

This project is licensed under the `MIT License`. See the LICENSE file for details.
