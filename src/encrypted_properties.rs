extern crate base64;
extern crate openssl;

use anyhow::{anyhow, Result};
use base64::{Engine, engine::general_purpose};
use openssl::symm::{Cipher, Crypter, Mode};

use crate::encrypted_properties_error::EncryptedPropertiesError;

pub struct EncryptedProperties {
    key: Vec<u8>,
}

impl EncryptedProperties {
    pub fn new(master_password: &str) -> Result<Self> {
        if master_password.is_empty() {
            return Err(anyhow!(EncryptedPropertiesError::EmptyPassword));
        }

        let master_key = general_purpose::STANDARD.decode(master_password).map_err(|_|
            EncryptedPropertiesError::Base64DecodeError)?;
        let key_length = master_key.len() * 8;

        if key_length != 128 && key_length != 192 && key_length != 256 {
            return Err(anyhow!(EncryptedPropertiesError::InvalidKeyLength));
        }

        Ok(EncryptedProperties { key: master_key })
    }

    pub fn decrypt(&self, encoded_encrypted: &str) -> Result<String> {
        let encrypted = general_purpose::STANDARD.decode(encoded_encrypted)?;
        let iv = &encrypted[0..16];
        let ciphertext = &encrypted[16..];
        let cipher = Cipher::aes_256_cbc();
        let mut decrypter = Crypter::new(cipher, Mode::Decrypt, &self.key, Some(iv))?;
        let mut decrypted_text = vec![0; ciphertext.len() + cipher.block_size()];
        let count = decrypter.update(ciphertext, &mut decrypted_text)?;
        decrypted_text.truncate(count);
        let mut final_result = vec![0; cipher.block_size()];
        let final_count = decrypter.finalize(&mut final_result)?;
        decrypted_text.extend_from_slice(&final_result[0..final_count]);
        Ok(String::from_utf8(decrypted_text)?)
    }

    pub fn encrypt(&self, plain: &str) -> Result<String> {
        let cipher = Cipher::aes_256_cbc();
        let mut iv = vec![0; cipher.iv_len().unwrap()];
        openssl::rand::rand_bytes(&mut iv)?;
        let mut encrypter = Crypter::new(cipher, Mode::Encrypt, &self.key, Some(&iv))?;
        encrypter.pad(true);
        let mut encrypted_text = vec![0; plain.as_bytes().len() + cipher.block_size()];
        let count = encrypter.update(plain.as_bytes(), &mut encrypted_text)?;
        encrypted_text.truncate(count);
        let mut final_result = vec![0; cipher.block_size()];
        let final_count = encrypter.finalize(&mut final_result)?;
        encrypted_text.extend_from_slice(&final_result[0..final_count]);
        final_result = iv;
        final_result.append(&mut encrypted_text);
        Ok(general_purpose::STANDARD.encode(&final_result))
    }
}
