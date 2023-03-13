use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string()
}

pub fn verify_password(
    password: &str,
    password_received: String,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password)?;
    Argon2::default().verify_password(password_received.as_bytes(), &parsed_hash)?;
    Ok(())
}
