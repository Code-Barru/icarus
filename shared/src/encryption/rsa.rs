use aes_gcm::aead::OsRng;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey, sha2::Sha256};

pub fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let bits = 2048;
    let private_key =
        RsaPrivateKey::new(&mut OsRng, bits).expect("Failed to generate a private key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

pub fn encrypt(public_key: &RsaPublicKey, plaintext: &[u8]) -> Vec<u8> {
    let padding = Oaep::new::<Sha256>();
    public_key
        .encrypt(&mut OsRng, padding, plaintext)
        .expect("Failed to encrypt")
}

pub fn decrypt(private_key: &RsaPrivateKey, ciphertext: &[u8]) -> Vec<u8> {
    let padding = Oaep::new::<Sha256>();
    private_key
        .decrypt(padding, ciphertext)
        .expect("Failed to decrypt")
}
