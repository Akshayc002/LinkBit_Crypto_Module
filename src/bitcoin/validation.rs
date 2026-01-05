use bitcoin::{Transaction, Address, Amount};

pub fn validate_funding(
    tx: &Transaction,
    escrow_address: &Address,
    min_amount_sat: u64
) -> bool {
    tx.output.iter().any(|o| {
        Address::from_script(
            &o.script_pubkey,
            *escrow_address.network()
        )
        .map(|addr| {
            addr == *escrow_address &&
            o.value >= Amount::from_sat(min_amount_sat)
        })
        .unwrap_or(false)
    })
}
