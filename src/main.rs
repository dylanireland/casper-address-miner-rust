use casper_types::AsymmetricType;
use casper_types::{PublicKey, SecretKey};
use casper_node::crypto::AsymmetricKeyExt;

// TODO: Support for Secp256k1, variable target char and length

fn main() {
    let target: char = '7';
    let length: usize = 7;

    look(target, length);
}

fn look(target: char, length: usize) {
    loop {
        let secret_key = SecretKey::generate_ed25519().unwrap();
        let secret_key_pem_result = secret_key.to_pem();
        let secret_key_pem = match secret_key_pem_result {
            Ok(pem) => pem,
            Err(error) => panic!("Error generating secret key: {:?}", error),
        };
        let public_key = PublicKey::from(&secret_key).to_hex();
        for (i, c) in public_key.chars().enumerate() { 
            if i < 2 { // Don't care about the guaranteed '01'
                continue;
            }
            if c != target {
                break;
            }

            if i - 1 == length {
                println!("{}", public_key);
                println!("{}", secret_key_pem);
                break;
            }
        }
    }
}