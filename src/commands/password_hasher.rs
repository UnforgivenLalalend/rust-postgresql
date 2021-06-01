use sha256::digest;

pub fn password_hasher(password: &str) -> String {
    digest(password)
}
