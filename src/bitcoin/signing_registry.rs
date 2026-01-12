use std::collections::HashMap;
use std::sync::Mutex;

use crate::domain::signing::{SignerRole, SigningState};

lazy_static::lazy_static! {
    static ref REGISTRY: Mutex<HashMap<String, SigningState>> =
        Mutex::new(HashMap::new());
}

pub fn record_signature(escrow_id: &str, role: SignerRole) -> SigningState {
    let mut map = REGISTRY.lock().unwrap();

    let state = map
        .entry(escrow_id.to_string())
        .or_insert_with(|| SigningState::new(escrow_id.to_string()));

    state.add_signature(role.clone());
    state.clone()
}