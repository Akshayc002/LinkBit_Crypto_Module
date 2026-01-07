use bitcoin::Network;

pub fn bitcoin_network() -> Network {
    match std::env::var("BTC_NETWORK").as_deref() {
        Ok("regtest") => Network::Regtest,
        Ok("testnet") => Network::Testnet,
        Ok("mainnet") => Network::Bitcoin,
        _ => Network::Regtest, // SAFE DEFAULT
    }
}

pub fn dev_signing_enabled() -> bool {
    std::env::var("DEV_SIGNING")
        .map(|v| v == "true")
        .unwrap_or(false)
}
