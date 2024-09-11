# Zero-Knowledge Proof Multiplier on Solana

This project implements a simple zero-knowledge proof system on Solana to verify multiplications without revealing the input factors. It demonstrates the basics of building and verifying zk-SNARKs on Solana.

## Project Overview

- Implements a basic ZKP circuit for multiplication using Circom
- Generates proving/verification keys using Powers of Tau ceremony  
- Compiles the circuit and exports a Solana verifier contract
- Creates tests to generate proofs and verify them on Solana

## Key Components

1. Circom Circuit
   - Defines a `Multiplier` circuit that takes private inputs `a` and `b` and outputs their product `c`

2. Trusted Setup
   - Uses Powers of Tau to generate the initial parameters
   - Creates the circuit-specific zkey

3. Solana Verifier Contract  
   - Exports verification key and pairing functions to Rust
   - Implements the Groth16 verification algorithm

4. Tests
   - Generates a valid proof using snarkjs
   - Formats proof data for the Solana program
   - Verifies the proof on-chain

## Development Process

1. Set up project structure and dependencies
2. Write Circom circuit 
3. Perform trusted setup and circuit compilation
4. Export Solana verifier contract
5. Implement proof generation and verification tests
6. Deploy and test on Solana testnet (when v1.17 is available)

## Key Learnings

- Basics of zero-knowledge proofs and zk-SNARKs
- Circom circuit development
- Cryptographic ceremony processes (Powers of Tau)
- Integrating ZKP systems with Solana programs
- Formatting and verifying Groth16 proofs on-chain

## Next Steps

- Extend circuit for more complex computations
- Optimize for reduced proof size/verification time
- Explore applications like private voting, auctions, identity systems