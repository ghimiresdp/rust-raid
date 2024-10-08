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
use rand::Rng;
use std::io::Write;
use std::{collections::HashMap, fs::OpenOptions};

use crate::input;

// define encoder and decoder types
type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

fn bytes_to_hex_string(bytes: Vec<u8>) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
}
fn generate_salt(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen()).collect()
}

fn generate_key(password: String, salt: &Vec<u8>) -> [u8; 32] {
    let mut key = [0u8; 32]; // Can be any desired size
    match Argon2::default().hash_password_into(password.as_bytes(), salt, &mut key) {
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

#[derive(Clone)]
pub(crate) struct Vault {
    pub(crate) name: String,
    key: [u8; 32],
    credentials: HashMap<String, String>,
}

impl Vault {
    /// Create a new Vault
    ///
    /// the `new` method creates a new instance of `Vault` with the given name
    /// and an empty list of credentials
    pub(crate) fn new(name: String, key: [u8; 32]) -> Vault {
        Vault {
            name,
            key,
            credentials: HashMap::new(),
        }
    }

    /// Open an existing vault
    ///
    /// If a vault exists, it opens the vault, else shows an error message saying it doesn't exist.
    pub(crate) fn open(name: String, password: String) -> Self {
        let salt = generate_salt(20);
        let key = generate_key(password, &salt);
        let key_string: String = key.iter().map(|k| return format!("{k:02x}")).collect();
        println!("key_string: {:?}", key_string);

        let encrypted = encrypt(key, String::from("Test data 111111111111111"));
        println!("Encrypted: {:?}", encrypted);

        let vault = Self::new(name, key);
        vault
    }

    /// Create
    ///
    /// This method creates a new vault with credentials.
    /// The vault contains encrypted credentials along with few encryption data
    /// such as number of iterations and salt.
    pub(crate) fn create(name: String) {
        match OpenOptions::new()
            .write(true)
            .create_new(true) // This will cause an error if the file already exists
            .open(format!("{name}.vault"))
            .as_mut()
        {
            Ok(file) => {
                let password = input("Enter password:");
                let salt = generate_salt(20);
                let key = generate_key(password, &salt);

                println!("The vault \"{name}\" has been successfully created");
                let _ = file.write(bytes_to_hex_string(salt).as_bytes());
                let _ = file.write(b"\n");
                // let _ = file.writ
            }
            Err(e) => {
                println!("Could not create the vault \"{name}\"");
                println!("{e}");
            }
        }
    }
}
