# NearGigs Smart Contract

NearGigs is a decentralized freelance marketplace on the NEAR blockchain. It offers secure, transparent transactions between freelancers and clients by implementing an escrow system. This smart contract allows clients to fund gigs, track progress through defined stages, and securely release payments upon completion.

## Features

- **Secure Payments:** Funds are securely held in escrow, with payments released only upon job completion.
- **Stage Tracking:** The gig progresses through stages - Offer, Deposit, In-Progress, Completed, and Confirm.
- **Role-Based Control:** Each stage can only be triggered by either the client or the freelancer as defined at gig creation.

## Prerequisites

- **Rust**: Install Rust using [Rustup](https://rustup.rs/).
- **NEAR CLI**: Install the NEAR CLI globally on your machine:
  ```bash
  npm install -g near-cli
  ```

## Setup

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/your-username/NearGigs
   cd NearGigs

   ```

2. **Install Rust: Install Rust using Rustup and ensure that the wasm32-unknown-unknown target is added**:

   ```bash
   rustup target add wasm32-unknown-unknown

   ```

3. **Building the Smart Contract**:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```
