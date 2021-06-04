use sha256::digest;

pub fn password_hasher(username: &str, password: &str) -> String {
    let mut text = String::from(&username[0..username.len() / 2]);
    text.push_str(password);

    for _ in 0..10 {
        text = digest(text);
    }

    text
}
