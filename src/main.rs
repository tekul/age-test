use std::io::Write;

use anyhow::Result;
use age::{Encryptor, secrecy::Secret};

fn main() {
    let encryptor = Encryptor::with_user_passphrase(Secret::new("somepassword".to_string()));
    let bytes = [0; 100];
    let result = encrypt_bytes(encryptor, &bytes);
    print!("{result:?}");
}

fn encrypt_bytes(encryptor: Encryptor, bytes: &[u8]) -> Result<Vec<u8>> {
    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(bytes)?;
    writer.finish()?;
    Ok(encrypted)
}
