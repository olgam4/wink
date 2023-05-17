use argon2::{
    password_hash::SaltString,
    Argon2, PasswordHasher,
};
use base64::{engine::general_purpose, Engine};

use crate::utils::get_nano_length;

pub struct Hasher;
impl Hasher {
    pub fn hash(string: &str) -> (String, String) {
        let lenght = get_nano_length();
        let salt = nanoid::nanoid!(lenght);

        let b64 = general_purpose::STANDARD.encode(salt.as_bytes());
        let b64 = b64.split("=").collect::<Vec<&str>>()[0];
        let salt = SaltString::from_b64(&b64).unwrap();

        let argon2 = Argon2::default();

        let hash = argon2
            .hash_password(string.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let hash = hash.split("$").collect::<Vec<&str>>()[5];

        (hash.to_string(), salt.to_string())
    }

    pub fn hash_with_salt(string: &str, salt: &str) -> (String, String) {
        let argon2 = Argon2::default();

        let salt = SaltString::from_b64(salt).unwrap();

        let hash = argon2
            .hash_password(string.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let hash = hash.split("$").collect::<Vec<&str>>()[5];

        (hash.to_string(), salt.to_string())
    }
}
