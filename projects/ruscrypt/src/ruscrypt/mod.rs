pub(crate) struct Crypto {
    key: Vec<u8>,
}

/// Structure `Crypto` is a cryptographic feature that allows us to perform a
/// simple encryption and decryption of our messages.
///
/// This algorithm uses one of the simplest ways to encrypt and decrypt the data
/// using basic XOR operation with the key provided.
///
/// To add more security, we perform XOR operation with different key with
/// character at key which is get using modulus of the index of the message.
impl Crypto {
    pub(crate) fn new(key: String) -> Self {
        Self {
            key: key.as_bytes().to_owned(),
        }
    }

    /// the `encrypt` method is used to encrypt the message using the key
    /// provided during initialization of the Crypto structure.
    /// the algorithm will first convert the key into a vector of bytes and then
    /// performs XOR operation with each byte that is enumerated with the index
    /// of the byte in the message string.
    ///
    /// for example:
    /// If key is ABC, it's bytes are [0x41, 0x42, and 0x43]
    ///
    /// If the message string is Hello, then it's bytes are
    /// [0x48, 0x65, 0x6C, 0x6C, 0x6F]
    ///
    /// so to encrypt the message, we perform XOR operation with corresponding
    /// byte of the key. if the index is larger, we perform modulus operation to
    /// get the byte for that index.
    ///
    /// the final output might contain non-printable characters, so these can
    /// also be saved as hex strings.
    pub(crate) fn encrypt(&self, message: String) -> String {
        let len = self.key.len();
        let enc = message
            .bytes() // convert to bytes
            .enumerate() // enumerate with index
            .map(|(idx, ch)| {
                let target = self.key[idx % len]; // find encrypting character
                (target ^ ch) as char // perform XOR operation
            })
            .collect::<String>();
        return enc;
    }

    /// the decrypt operation is exactly the same as that of encryption
    /// since we performed the XOR operation on the message, performing the XOR
    /// operation on the encrypted data will revert back to the original message
    /// for example:
    ///      0 0 1 0 1 1 0 1  (original message)
    /// XOR  1 0 1 1 0 1 0 1  (encryption key)
    /// ---------------------
    ///   =  1 0 0 1 1 0 0 0  (encrypted message)
    /// XOR  1 0 1 1 0 1 0 1  (encryption key)
    /// ---------------------
    ///   =  0 0 1 0 1 1 0 1  (original message)
    pub(crate) fn decrypt(&self, message: String) -> String {
        return self.encrypt(message);
    }
}
