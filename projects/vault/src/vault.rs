use aes::{
    cipher::{
        block_padding::{Iso10126, Pkcs7},
        generic_array::GenericArray,
        BlockDecrypt, BlockDecryptMut, BlockEncrypt, BlockEncryptMut, KeyInit, KeyIvInit,
    },
    Aes256,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use std::io::Write;
use std::{collections::HashMap, fs::File};

// define encoder and decoder types
type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

fn get_password_hash(password: String) -> [u8; 32] {
    let mut key = [0u8; 32]; // Can be any desired size
    match Argon2::default().hash_password_into(
        password.as_bytes(),
        // salt,
        b"test1234567890",
        &mut key,
    ) {
        Ok(data) => println!("{data:?}"),
        Err(error) => println!("{error:?}"),
    }
    println!("key: {:?}", key);
    key
}

fn encrypt(key: [u8; 32], message: String) -> Vec<u8> {
    let key = GenericArray::from(key);
    let mut message = message.as_bytes().to_vec();
    let len = message.len();
    let cipher = Aes256::new(&key);
    cipher
        .encrypt_padded_mut::<Iso10126>(&mut message, len)
        .unwrap();
    println!("{:?}", message);
    message
}
pub(crate) struct Vault {
    pub(crate) name: String,
    credentials: HashMap<String, String>,
}

impl Vault {
    pub(crate) fn new(name: &str) -> Vault {
        Vault {
            name: name.to_owned(),
            credentials: HashMap::new(),
        }
    }
    pub(crate) fn open(name: &str, password: String) -> Self {
        let key = get_password_hash(password);
        let key_string: String = key.iter().map(|k| return format!("{k:02x}")).collect();
        println!("key_string: {:?}", key_string);

        let encrypted = encrypt(key, String::from("Test data 111111111111111"));
        println!("Encrypted: {:?}", encrypted);

        Self {
            name: name.to_owned(),
            credentials: HashMap::new(),
        }
    }
}
