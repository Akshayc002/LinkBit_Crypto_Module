use bitcoin::{
    PublicKey, Network,
    blockdata::script::Builder,
    opcodes::all::OP_CHECKMULTISIG,
    Address, ScriptBuf,
};

pub fn create_2of3_multisig(
    keys: Vec<PublicKey>,
    network: Network
) -> (Address, ScriptBuf) {

    assert_eq!(keys.len(), 3);

    let redeem_script = Builder::new()
        .push_int(2)
        .push_key(&keys[0])
        .push_key(&keys[1])
        .push_key(&keys[2])
        .push_int(3)
        .push_opcode(OP_CHECKMULTISIG)
        .into_script();

    let address = Address::p2wsh(&redeem_script, network);

    (address, redeem_script)
}
