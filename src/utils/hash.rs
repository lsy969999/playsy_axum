use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use sha2::{Digest, Sha256, Sha512};

pub fn hash_argon2(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_argon2(password: &str, password_hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    let argon2 = Argon2::default();
    let ok = argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok();
    Ok(ok)
}

pub fn hash_sha_256(digest: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(digest.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn hash_sha_512(digest: String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(digest.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}