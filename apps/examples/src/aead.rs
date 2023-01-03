use aes_gcm_siv::{aead::Aead, Aes256GcmSiv};
use crypto::aead::KeyInit;
use generic_array::GenericArray;
use rand::{rngs::OsRng, RngCore};

#[test]
fn try_aead() {
    let key = Aes256GcmSiv::generate_key(&mut OsRng);
    let aes_gcm_siv = Aes256GcmSiv::new(&key);

    let mut nonce = GenericArray::from([0u8; 12]);
    OsRng.fill_bytes(&mut nonce);

    let message = "oh hi!";
    let encrypted = aes_gcm_siv.encrypt(&nonce, message.as_bytes()).unwrap();

    eprintln!("MSG: {:?}", message);
    eprintln!("NONCE: {:?}", hex::encode(&nonce));
    eprintln!("ENC: {:?}", hex::encode(&encrypted));

    let decrypted = aes_gcm_siv.decrypt(&nonce, encrypted.as_ref()).unwrap();

    assert_eq!(decrypted, message.as_bytes());
}
