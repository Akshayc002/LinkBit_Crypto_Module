use crate::domain::signing::{SigningState, SignerRole};

#[test]
fn approves_only_when_exactly_two_signers() {
    let mut state = SigningState::new("escrow-1".to_string());

    // No signers
    assert!(!state.is_approved());

    // One signer → NOT approved
    state.add_signature(SignerRole::Borrower);
    assert!(!state.is_approved());

    // Second signer → APPROVED
    state.add_signature(SignerRole::Lender);
    assert!(state.is_approved());
}

#[test]
fn duplicate_signer_does_not_unlock_escrow() {
    let mut state = SigningState::new("escrow-2".to_string());

    state.add_signature(SignerRole::Borrower);
    state.add_signature(SignerRole::Borrower); // duplicate

    assert!(!state.is_approved());
}
