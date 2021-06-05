use pbkdf2::{
    password_hash::{PasswordHasher, SaltString},
    Pbkdf2,
};
use rand_core::OsRng;

pub fn password_hasher(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Pbkdf2
        .hash_password_simple(password.as_bytes(), salt.as_ref())
        .unwrap()
        .to_string();

    password_hash
}

pub fn password_hasher_with_salt(salt: &str, password: &str) -> String {
    let password_hash = Pbkdf2
        .hash_password_simple(password.as_bytes(), salt)
        .unwrap()
        .to_string();

    password_hash
}
