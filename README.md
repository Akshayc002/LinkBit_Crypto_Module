
# Linkbit Bitcoin Multisig Escrow Service (Rust)

A **non-custodial, cryptographic Bitcoin escrow microservice** built in **Rust** for the **Linkbit** platform.

This service is responsible for **creating, validating, and enforcing 2-of-3 Bitcoin multisig escrow rules** using industry-standard Bitcoin primitives such as **SegWit**, **PSBT (BIP-174)**, and **secp256k1**.

> ⚠️ This service **never holds real user funds or private keys in production**.

---

## 🎯 Purpose

This module acts as the **Bitcoin trust layer** for Linkbit.

It ensures:
- No single party can move escrowed Bitcoin
- Only valid **2-of-3 multisig transactions** are accepted
- All Bitcoin data structures are **consensus-correct**
- Business logic stays **outside** cryptographic enforcement

---

## 🧱 High-Level Architecture

Spring Boot (Business Logic)
        |
        | REST / gRPC
        |
Rust Bitcoin Escrow Service   ← this repository
        |
        | (future integration)
        |
Bitcoin Core / Wallet / HSM

---

## 🔐 Escrow Model (2-of-3 Multisig)

Participants:
- **Borrower**
- **Lender**
- **Linkbit Escrow**

Rules:
- Any **2 of the 3** must sign to spend funds
- No single party can move funds alone

Valid spend paths:
- Borrower + Lender → normal repayment
- Lender + Escrow → liquidation
- Borrower + Escrow → dispute resolution

---

## ✅ Current Capabilities

### 1️⃣ Health Check

**Endpoint**
```
GET /health
```

**Purpose**
- Confirms service availability
- Used by monitoring & orchestration tools

---

### 2️⃣ Multisig Escrow Creation

**Endpoint**
```
POST /escrow/create
```

**Input**
```json
{
  "borrower_pubkey": "02...",
  "lender_pubkey": "03...",
  "escrow_pubkey": "02..."
}
```

**Output**
```json
{
  "escrow_address": "bcrt1q... / tb1q...",
  "redeem_script": "522102...53ae"
}
```

**Guarantees**
- All public keys are valid secp256k1 keys
- Deterministic 2-of-3 multisig script
- Native SegWit (P2WSH)
- No private keys involved

---

### 3️⃣ PSBT Creation (Unsigned)

**Endpoint**
```
POST /transaction/create-psbt
```

**Input**
```json
{
  "unsigned_tx_hex": "<valid_unsigned_transaction_hex>"
}
```

**Output**
```json
{
  "psbt_base64": "cHNidP8BA..."
}
```

**Notes**
- Transaction structure is strictly validated
- Invalid hex or malformed transactions are rejected
- No signing or broadcasting happens here

---

### 4️⃣ PSBT Structural Verification (2-of-3 Rule)

**Endpoint**
```
POST /transaction/verify-psbt
```

**Input**
```json
{
  "psbt_base64": "cHNidP8BA...",
  "borrower_pubkey": "02...",
  "lender_pubkey": "03...",
  "escrow_pubkey": "02..."
}
```

**Output (success)**
```json
{
  "valid": true
}
```

**Output (failure example)**
```json
{
  "error": "NotEnoughSignatures"
}
```

**What is enforced**
- Only allowed signers are accepted
- Exactly **2 unique signers** required
- Unknown or duplicate signers are rejected

> This is **structural verification**. Full cryptographic sighash verification is added later with Bitcoin Core.

---

### 5️⃣ Dev-Only PSBT Signing Helper (Testing Harness)

**Endpoint**
```
POST /dev/sign-psbt
```

**Environment Flag (Required)**
```
DEV_SIGNING=true
```

**Input**
```json
{
  "psbt_base64": "cHNidP8BA...",
  "role": "borrower | lender | escrow"
}
```

**Output**
```json
{
  "psbt_base64": "cHNidP8BA...signed..."
}
```

⚠️ **DEV ONLY**
- Uses deterministic in-memory private keys
- NOT real Bitcoin signing
- Disabled by default
- Must never be enabled in production

---

## 🚫 Explicit Non-Goals

This service does **NOT**:
- Hold private keys in production
- Custody Bitcoin
- Broadcast transactions
- Perform KYC or business logic
- Manage users or accounts

Those responsibilities belong to:
- Spring Boot services
- Wallets / HSMs
- Compliance & ops layers

---

## 🔐 Security Guarantees

- No single-party fund movement
- No fake or malformed public keys
- No backend custody of funds
- Deterministic, auditable scripts
- Strict Bitcoin consensus parsing

---

## 🧪 Development & Testing

### Run in DEV mode
```
$env:DEV_SIGNING="true"
cargo run
```

### Disable dev signing
```
$env:DEV_SIGNING=""
```

### Example unsigned transaction for testing
```json
{
  "unsigned_tx_hex": "010000000100000000000000000000000000000000000000000000000000000000000000000000000000ffffffff0100e1f505000000000000000000"
}
```

---

## 🛣️ Roadmap

Next planned additions:
- Wallet / HSM-based real signing
- Bitcoin Core (regtest/testnet) integration
- Funding validation
- Liquidation & dispute flows
- Transaction finalization & broadcast
- Audit logging & persistence

---

## 📜 License

Internal – Linkbit Pvt Ltd  
All rights reserved.
