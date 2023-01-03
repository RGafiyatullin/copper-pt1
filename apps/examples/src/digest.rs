use crypto::digest::Digest;
use sha3::Sha3_256;
use sha3::Sha3_512;

#[test]
fn calculate_a_digest() {
    let input = "";

    let sha3_512 = Sha3_512::new().chain_update(input.as_bytes()).finalize();
    let sha3_256 = Sha3_256::new().chain_update(input.as_bytes()).finalize();

    eprintln!("input: {:?}", input);
    eprintln!("sha3-512: {}", hex::encode(sha3_512));
    eprintln!("sha3-256: {}", hex::encode(sha3_256));
}
