pub(crate) struct Crypto {
    key: Vec<u8>,
}

impl Crypto {
    pub(crate) fn new(key: String) -> Self {
        Self {
            key: key.as_bytes().to_owned(),
        }
    }

    pub(crate) fn encrypt(&self, message: String) -> String {
        let len = self.key.len();
        let enc = message
            .bytes()
            .enumerate()
            .map(|(idx, ch)| {
                let target = self.key[idx % len];
                (target ^ ch) as char
                // format!("{:0x}", target ^ ch)
            })
            .collect::<String>();
        return enc;
    }

    pub(crate) fn decrypt(&self, message: String) -> String {
        let len = self.key.len();
        let dec = message
            .bytes()
            .enumerate()
            .map(|(idx, ch)| {
                let target = self.key[idx % len];
                (target ^ ch) as char
            })
            .collect::<String>();
        return dec;
    }
}
