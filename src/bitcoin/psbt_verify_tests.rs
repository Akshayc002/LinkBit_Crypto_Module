use crate::bitcoin::psbt_verify::verify_2of3_psbt;
use bitcoin::PublicKey;
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey as SecpPublicKey};

#[test]
fn rejects_unknown_signer() {
    let secp = Secp256k1::new();

    let sk1 = SecretKey::from_slice(&[1u8; 32]).unwrap();
    let sk2 = SecretKey::from_slice(&[2u8; 32]).unwrap();
    let sk3 = SecretKey::from_slice(&[3u8; 32]).unwrap();

    let pk1 = PublicKey::new(SecpPublicKey::from_secret_key(&secp, &sk1));
    let pk2 = PublicKey::new(SecpPublicKey::from_secret_key(&secp, &sk2));
    let pk3 = PublicKey::new(SecpPublicKey::from_secret_key(&secp, &sk3));

    let allowed_keys = vec![pk1, pk2, pk3];

    let psbt = "cHNidP8BAAAA";

    let result = verify_2of3_psbt(psbt, &allowed_keys);

    assert!(result.is_err());
}