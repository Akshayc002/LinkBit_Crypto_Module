pub fn validate_2of3(signers: &[String]) -> bool {
    signers.len() == 2 && signers[0] != signers[1]
}
