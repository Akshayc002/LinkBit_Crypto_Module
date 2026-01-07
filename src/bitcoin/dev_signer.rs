use bitcoin::{
    psbt::Psbt,
    secp256k1::{Secp256k1, Message},
    EcdsaSighashType,
};
use crate::bitcoin::dev_keys::DevRole;

/// ⚠️ STRUCTURAL signing only (DEV)
pub fn sign_psbt_dev(
    mut psbt: Psbt,
    role: DevRole,
) -> Psbt {
    let secp = Secp256k1::new();
    let (sk, pk) = crate::bitcoin::dev_keys::dev_keypair(role);

    // ⚠️ Fake message (DEV ONLY)
    // This is NOT real Bitcoin signing
    let msg = Message::from_slice(&[1u8; 32]).unwrap();

    let sig = secp.sign_ecdsa(&msg, &sk);

    for input in psbt.inputs.iter_mut() {
        input.partial_sigs.insert(
            pk,
            bitcoin::ecdsa::Signature {
                sig,
                hash_ty: EcdsaSighashType::All,
            },
        );
    }

    psbt
}
