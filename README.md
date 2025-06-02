# Solana-signature-verification
RareSkills course: Verifying Ed25519 Signatures in Solana Anchor Programs: A Distributor-Driven Airdrop. This tutorial shows how to verify an off-chain Ed25519 signature in a Solana program.

# Solana Airdrop Verification

A Solana program to verify off-chain Ed25519 signatures for an airdrop, as described in the [Airdrop Verification Tutorial](https://www.notion.so/Signature-verification-for-RareSkills-ab9001accbbb496cae96c735b75080fb?d=1f463b5fa3c880d59d62001c96837d3e#1d363b5fa3c880459f3fe530965d3399). This program uses Anchor to validate signatures against a fixed distributor public key, ensuring secure airdrop claims.

## Prerequisites

- **Rust**: 1.71.0+
  rustup install 1.71.0
  rustup default 1.71.0

Solana CLI: 1.17.30

sh -c "$(curl -sSfL https://release.solana.com/v1.17.30/install)"

Anchor CLI: 0.29.0

cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

Node.js: 18.x or later

npm install -g n
n 18

Yarn: 1.x

npm install -g yarn

Installation
Clone the repository:

git clone https://github.com/salina-dev/Solana-sig-verification.git
cd solana-sig-verification

Install Node.js dependencies:

npm install

Verify Rust, Solana, and Anchor versions:

rustc --version  # Should show 1.71.0
solana --version  # Should show 1.17.30
anchor --version  # Should show 0.29.0

Building the Program
Compile the Solana program:

anchor build

This generates the IDL (target/idl/airdrop_whitelist.json) and keypair (target/deploy/airdrop-keypair.json).
Running Tests
Run the unit tests to verify valid and invalid signature cases:
Start a local Solana validator:

solana-test-validator

In a new terminal, run the tests:

anchor test

The tests (tests/airdrop.ts) check:
A valid signature from the expected distributor.

An invalid signature from a wrong distributor, expecting an InvalidDistributorKey error.

Deploying to Devnet
Deploy the program to Solana’s devnet:
Ensure a funded wallet keypair at ~/.config/solana/id.json:
bash

solana-keygen new -o ~/.config/solana/id.json  # If needed
solana airdrop 2  # Fund with 2 SOL

