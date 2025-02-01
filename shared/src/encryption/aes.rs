use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use rand::Rng;

pub fn encrypt(key: &[u8], plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let cipher = Aes256Gcm::new(Key::<aes_gcm::aes::Aes256>::from_slice(key));
    let nonce: [u8; 12] = rand::rng().random();
    let ciphertext = match cipher.encrypt(Nonce::from_slice(&nonce), plaintext) {
        Ok(ciphertext) => ciphertext,
        Err(_) => panic!("Failed to encrypt"),
    };
    (ciphertext, nonce.to_vec())
}

pub fn decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::<aes_gcm::aes::Aes256>::from_slice(key));
    match cipher.decrypt(Nonce::from_slice(nonce), ciphertext) {
        Ok(decrypted) => decrypted,
        Err(_) => panic!("Failed to decrypt"),
    }
}
