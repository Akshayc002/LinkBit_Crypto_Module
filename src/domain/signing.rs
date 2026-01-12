use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SignerRole {
    Borrower,
    Lender,
    Escrow,
}

#[derive(Debug, Clone)]
pub struct SigningState {
    pub escrow_id: String,
    pub signed_roles: HashSet<SignerRole>,
}

impl SigningState {
    pub fn new(escrow_id: String) -> Self {
        Self {
            escrow_id,
            signed_roles: HashSet::new(),
        }
    }

    pub fn add_signature(&mut self, role: SignerRole) {
        self.signed_roles.insert(role);
    }

    pub fn is_approved(&self) -> bool {
        self.signed_roles.len() == 2
    }
}