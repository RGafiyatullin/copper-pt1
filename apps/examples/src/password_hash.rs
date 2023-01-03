use crypto::password_hash::PasswordHasher;
use crypto::password_hash::PasswordVerifier;
use crypto::password_hash::SaltString;

use rand::rngs::OsRng;

use scrypt::Scrypt;

#[test]
fn hash_a_password() {
    let password = "secureme!";
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Scrypt.hash_password(password.as_bytes(), &salt).unwrap();

    eprintln!("input: {:?}", password);
    eprintln!("salt:  {:?}", salt);
    eprintln!("hash:  {:?}", password_hash);

    assert!(Scrypt
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok());
    assert!(Scrypt
        .verify_password("invalid".as_bytes(), &password_hash)
        .is_err());
}
