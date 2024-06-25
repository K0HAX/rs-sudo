pub mod pwcheck;
pub mod errno;
pub mod system;
pub mod errors;
pub mod types;
mod externs;

use crate::pwcheck::{get_password, check_password};
use crate::system::{getuid, setuid};
use crate::types::uid_t;

fn elevate() -> Result<(), Box<dyn std::error::Error>> {
    // Get the old UID to compare later.
    let olduid: uid_t = getuid();
    println!("olduid: {:?}", olduid);
    let _ = setuid(0)?;
    let newuid: uid_t = getuid();
    println!("New UID: {:?}", newuid);
    if newuid == 0 {
        println!("We are root!");
        return Ok(());
    } else {
        panic!("We could not elevate to root!");
    }
}

fn exec_shell() -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;
    let shell: String = match std::env::var_os("SHELL") {
        Some(val) => val.into_string().expect("Failed to convert SHELL environment variable into a string."),
        None => "/bin/sh".into(),
    };

    // Save and clean up environment variables
    let mut env_vec = Vec::new();
    for (key, value) in std::env::vars_os() {
        env_vec.push((key.clone(), value.clone()));
        std::env::remove_var(key);
    }
    // Make env_vec immutable
    let env_vec = env_vec;

    // Spawn the shell as a login shell
    let shell_status = Command::new(shell)
        .arg("--login")
        .status()
        .expect("Failed to start shell: {shell}");

    // Restore saved environment variables
    for (key, value) in env_vec {
        std::env::set_var(key, value);
    }

    println!("process finished with: {shell_status}");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This is the password hash that needs to be matched to become root
    let internal_hash: &'static str = include_str!("../pw-hash.txt")
        .strip_suffix("\n")
        .unwrap();
    println!("{:#?}", internal_hash);

    // Get the password from the user, and check if it matches the hash
    let valid: bool = {
        let pw_clear = get_password().unwrap();
        check_password(&pw_clear, internal_hash).unwrap()
    };
    match valid {
        // The password matches!
        true => {
            println!("The password is valid!");
            // Try to become root, panic if there is a problem when we try to become root.
            let _ = elevate().map_err(|e| {
                panic!("Unable to elevate: {}", e);
            });
            // Run the shell, and panic if there is an error.
            exec_shell().unwrap();
            Ok(())
        },
        // The password does not match! Do not elevate the user.
        false => {
            println!("The password is wrong!");
            Ok(())
        }
    }
}

