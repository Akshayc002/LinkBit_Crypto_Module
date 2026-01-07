Linkbit Bitcoin Multisig Escrow Service (Rust)

A non-custodial Bitcoin multisig escrow microservice, built in Rust, designed to act as the cryptographic enforcement layer for the Linkbit platform.

This service creates, validates, and enforces 2-of-3 Bitcoin multisig escrow rules using PSBT (BIP-174), without ever holding real user funds or private keys.

🧠 Purpose

Provide cryptographic guarantees for Bitcoin escrow transactions while keeping all business logic, custody, and compliance outside this service.

This module is intentionally narrow in scope and security-first.

🔐 Key Principles

Non-custodial – No private keys stored or managed in production

Deterministic & auditable – Same inputs always produce same outputs

PSBT-first design – Compatible with wallets, HSMs, and hardware signers

Strict validation – Invalid Bitcoin data is rejected early

Dev safety – Dangerous functionality gated by environment flags

📦 What This Service Does (Current Capabilities)
1. Health Check
GET /health


Confirms the service is running and reachable.

2. 2-of-3 Multisig Escrow Creation

POST /escrow/create


Input

Borrower public key

Lender public key

Escrow (Linkbit) public key

Output

P2WSH multisig address

Redeem script (hex)

Guarantees

Only valid secp256k1 public keys are accepted

No single party can control funds

No private keys are ever used

3. PSBT Creation (Unsigned)
POST /transaction/create-psbt


Input

Fully valid unsigned Bitcoin transaction (hex)

Output

PSBT (Base64)

Guarantees

Strict Bitcoin consensus parsing

No malformed or truncated transactions allowed

Safe failure (no panics)

4. PSBT Verification (2-of-3 Enforcement)
POST /transaction/verify-psbt


Input

PSBT (Base64)

Borrower, lender, escrow public keys

What is verified

PSBT structure is valid

Signers are from the allowed set

Exactly 2 unique signers

No unknown or duplicate signers

Output

{ "valid": true } or a specific error

⚠️ This is structural verification, not full sighash/UTXO verification (added later with Bitcoin Core).

5. Dev-Only PSBT Signing (Testing Harness)
POST /dev/sign-psbt


Purpose

Enables full end-to-end testing without wallets, Bitcoin Core, or real BTC

Guard

DEV_SIGNING=true


Behavior

Uses deterministic in-memory keys

Simulates borrower / lender / escrow signatures

Allows Postman-based demos and testing

🚫 Never enabled in production

🚫 What This Service Does NOT Do (By Design)

❌ Hold or manage private keys (production)

❌ Store or custody Bitcoin

❌ Broadcast transactions

❌ Decide loan or business rules

❌ Perform KYC / compliance

❌ Act as a wallet or node

These responsibilities belong to:

Spring Boot business services

Wallets / HSMs

Bitcoin Core or external providers

🧱 Architecture Fit
Spring Boot (Business Logic, Users, Loans)
        |
        | HTTP / gRPC
        |
Rust Bitcoin Escrow Service  ← this repo
        |
        | (later)
        |
Bitcoin Core / Wallet / HSM


This separation ensures:

Strong security boundaries

Audit-friendly design

Regulator-friendly non-custodial model

🛠️ Tech Stack

Rust

Axum – HTTP server

bitcoin (Rust crate) – Bitcoin primitives & PSBT

secp256k1 – Cryptography

Serde – Serialization

Tokio – Async runtime

▶️ Running the Service (Dev)
cargo run


Health check:

curl http://localhost:9000/health

🔧 Enable Dev Signing (Testing Only)
PowerShell
$env:DEV_SIGNING="true"
cargo run

Linux / macOS
export DEV_SIGNING=true
cargo run


Disable before production:

unset DEV_SIGNING

🧪 Typical Dev Test Flow (Postman)

Create multisig escrow

Create PSBT

Sign PSBT as borrower (dev)

Sign PSBT as lender (dev)

Verify PSBT → { "valid": true }

This demonstrates the full escrow happy path without risk.

🔐 Security Notes

Dev signing is explicitly gated

No unwrap() on user input

Strict Bitcoin parsing

Deterministic behavior for audits

Designed for later HSM / wallet integration

🚀 Roadmap (Next Steps)

Real wallet / HSM signing

Bitcoin Core integration (regtest / testnet)

Funding validation

Transaction finalization & broadcast

Liquidation flow (lender + escrow)

Dispute flow (borrower + escrow)

Persistence & audit logging

📄 License

Internal – Linkbit
(Not intended for public custody or wallet usage)