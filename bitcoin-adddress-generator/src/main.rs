use itertools::Itertools;
use rand::Rng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};

fn main() {

    //  Step 1: generate a 256bit random private key
    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    println!("private key : {:?}",private_key);

    // Step 2: Apply ECDSA to get the public key
    let public_key = generate_publickey_from_privatekey(&private_key);
    println!("Public Key: {:02x}", public_key.iter().format(""));

}

fn generate_publickey_from_privatekey(private_key: &[u8; 32]) -> [u8; 33] {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key).unwrap();
    return PublicKey::from_secret_key(&secp, &secret_key).serialize();
}