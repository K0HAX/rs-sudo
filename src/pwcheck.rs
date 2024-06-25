use inquire::{Password, PasswordDisplayMode, InquireError};
use argon2::{
    password_hash::{
        PasswordHash, PasswordVerifier
    },
    Argon2
};

/// Prompt the user for their password.
pub fn get_password() -> Result<String, InquireError> {
    Password::new("Password:")
        .with_display_toggle_enabled()
        .with_display_mode(PasswordDisplayMode::Masked)
        .without_confirmation()
        .with_formatter(&|_| String::from("Input received"))
        .prompt()
}

/// Compare a cleartext password to a hashed password.
///
/// Return `true` if the password matches the hashed password, otherwise return `false`.
pub fn check_password(password: &str, hash: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
