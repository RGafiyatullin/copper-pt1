use aes::Aes256;
use crypto::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use generic_array::GenericArray;

#[test]
fn cipher_a_block() {
    let key = GenericArray::from([0u8; 32]);

    let block_in = GenericArray::from(std::array::from_fn::<_, 16, _>(|idx| idx as u8));

    let cipher = Aes256::new(&key);
    let mut block_out = block_in.clone();
    cipher.encrypt_block(&mut block_out);

    eprintln!("KEY: {}", hex::encode(&key));
    eprintln!("ORIG:{}", hex::encode(&block_in));
    eprintln!("ENC: {}", hex::encode(&block_out));

    cipher.decrypt_block(&mut block_out);
    eprintln!("DEC: {}", hex::encode(&block_out));

    assert_eq!(block_in, block_out);
}
