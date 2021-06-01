use sha256::digest;

pub fn sha256_hasher(password: &str) -> String {
    digest(password)
}
