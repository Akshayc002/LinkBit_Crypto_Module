use bitcoin::{Transaction, psbt::Psbt};

#[derive(Debug)]
pub enum PsbtCreateError {
    InvalidHex,
    InvalidTransaction,
}

pub fn create_psbt_from_hex(tx_hex: &str) -> Result<Psbt, PsbtCreateError> {
    let tx_bytes = hex::decode(tx_hex)
        .map_err(|_| PsbtCreateError::InvalidHex)?;

    let tx: Transaction = bitcoin::consensus::deserialize(&tx_bytes)
        .map_err(|_| PsbtCreateError::InvalidTransaction)?;

    Psbt::from_unsigned_tx(tx)
        .map_err(|_| PsbtCreateError::InvalidTransaction)
}
