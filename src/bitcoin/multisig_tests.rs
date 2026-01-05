#[cfg(test)]
mod tests {
    use bitcoin::secp256k1::{Secp256k1, rand::thread_rng};
    use bitcoin::{PublicKey, Network};
    use crate::bitcoin::multisig::create_2of3_multisig;

    #[test]
    fn test_2of3_multisig_creation() {
        let secp = Secp256k1::new();
        let mut rng = thread_rng();

        let keys: Vec<PublicKey> = (0..3)
            .map(|_| {
                let keypair = bitcoin::key::Keypair::new(&secp, &mut rng);
                PublicKey::new(keypair.public_key())
            })
            .collect();

        let (address, script) =
            create_2of3_multisig(keys, Network::Regtest);

        assert!(address.to_string().starts_with("bcrt1"));
        assert!(script.len() > 30);
    }
}
