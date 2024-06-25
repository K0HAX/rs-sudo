pub mod pwhash;
pub mod pwcheck;
use crate::pwhash::gen_hash;
use crate::pwcheck::{get_password, check_password};

fn main() {
    let pw_initial = get_password();

    let pw_value = match pw_initial {
        Ok(x) => x,
        Err(_) => {
            panic!("An error occurred when asking for your password.");
        },
    };

    let pw_hashed = gen_hash(&pw_value).unwrap();
    println!("{}", pw_hashed);
    let valid = check_password(&pw_value, &pw_hashed).unwrap();
    println!("Valid: {:?}", valid);
}
