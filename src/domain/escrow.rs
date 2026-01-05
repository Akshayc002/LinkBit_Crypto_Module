use bitcoin::Address;

pub struct Escrow {
    pub address: Address,
    pub redeem_script_hash: String,
}
