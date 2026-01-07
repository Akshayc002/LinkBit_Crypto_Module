use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::PublicKey;

/// ⚠️ DEV ONLY — deterministic private keys
/// NEVER use this in production
pub enum DevRole {
    Borrower,
    Lender,
    Escrow,
}

pub fn dev_keypair(role: DevRole) -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();

    // Fixed secret keys (32 bytes, non-zero, valid)
    let sk_bytes = match role {
        DevRole::Borrower => [1u8; 32],
        DevRole::Lender => [2u8; 32],
        DevRole::Escrow => [3u8; 32],
    };

    let sk = SecretKey::from_slice(&sk_bytes).unwrap();
    let pk = PublicKey::new(bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &sk));

    (sk, pk)
}