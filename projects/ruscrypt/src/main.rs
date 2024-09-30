mod ruscrypt;

use ruscrypt::Crypto;

/// A basic implementation of `ruscrypt::Crypto` structure
/// to run/test the solution, please run the following:
/// * `cargo run --bin ruscrypt`
/// * `cargo test --bin ruscrypt`
fn main() {
    // please check the Crypto struct for documentation of how it works
    let crypto = Crypto::new("test".to_owned());
    let encrypted = crypto.encrypt("This is a original text".to_owned());
    println!("Encrypted: {encrypted}");
    let decrypted = crypto.decrypt(encrypted);
    println!("Decrypted: {decrypted}");
}

#[cfg(test)]
mod tests {
    use crate::ruscrypt::Crypto;

    #[test]
    fn test_encrypt_decrypt() {
        let key = String::from("My Encryption Key");
        let message = String::from("This is my Secret Message");

        let crypt = Crypto::new(key);
        assert_eq!(crypt.decrypt(crypt.encrypt(message.clone())), message);
    }
}
