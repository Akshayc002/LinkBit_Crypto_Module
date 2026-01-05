#[cfg(test)]
mod tests {
    use bitcoin::{
        secp256k1::{Secp256k1, rand::thread_rng},
        PublicKey, Amount, Address
    };
    use bitcoincore_rpc::{Client, Auth, RpcApi};
    use crate::bitcoin::multisig::create_2of3_multisig;

    fn rpc() -> Client {
        Client::new(
            "http://127.0.0.1:18443",
            Auth::UserPass("linkbit".into(), "linkbitpass".into())
        ).unwrap()
    }

    #[test]
    fn test_multisig_funding_on_regtest() {
        let secp = Secp256k1::new();
        let mut rng = thread_rng();

        let keys: Vec<PublicKey> = (0..3)
            .map(|_| {
                let keypair = bitcoin::key::Keypair::new(&secp, &mut rng);
                PublicKey::new(keypair.public_key())
            })
            .collect();

        let (escrow_address, _) =
            create_2of3_multisig(keys, bitcoin::Network::Regtest);

        let client = rpc();

        let txid = client.send_to_address(
            &escrow_address,
            Amount::from_btc(1.0).unwrap(),
            None, None, None, None, None, None
        ).unwrap();

        client.generate_to_address(1, &client.get_new_address(None, None).unwrap())
            .unwrap();

        let tx = client.get_transaction(&txid, None).unwrap();

        assert!(tx.info.confirmations.unwrap() > 0);
    }
}
