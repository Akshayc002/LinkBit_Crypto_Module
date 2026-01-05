pub fn validate_pubkeys_unique(keys: &[String]) -> bool {
    let mut set = std::collections::HashSet::new();
    keys.iter().all(|k| set.insert(k))
}
