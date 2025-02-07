use super::RTClient;
use rsa::pkcs1::DecodeRsaPublicKey;
use shared::packets::{
    self, EncryptionRequest, EncryptionResponse, LoginRequest, Packet, PacketEnum,
};
use std::error::Error;
use tracing::{debug, error};
use uuid::Uuid;

impl RTClient {
    pub async fn handshake(&self, uuid: Uuid) -> Result<(), Box<dyn Error>> {
        match self.send_login_request(uuid).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to send login request: {:?}", e);
                return Err(e.into());
            }
        };
        debug!("Sent login request");

        let response = match self.receive_raw().await {
            Ok(response) => response,
            Err(e) => {
                error!("Failed to receive Encryption Request {:?}", e);
                return Err(e.into());
            }
        };

        let encryption_request = match packets::from_packet_bytes(&response) {
            Ok(PacketEnum::EncryptionRequest(packet)) => packet,
            _ => {
                return Err("Invalid packet".into());
            }
        };
        debug!("Received Encryption Request");

        match self.send_encryption_response(encryption_request).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to send encryption response: {:?}", e);
                return Err(e.into());
            }
        };
        debug!("Sent encryption response");

        Ok(())
    }

    async fn send_login_request(&self, uuid: Uuid) -> Result<(), Box<dyn Error>> {
        let data = LoginRequest::new(uuid).serialize();
        self.send_raw(data).await;

        Ok(())
    }

    async fn send_encryption_response(
        &self,
        encryption_request: EncryptionRequest,
    ) -> Result<(), Box<dyn Error>> {
        let key_str = match std::str::from_utf8(&encryption_request.public_key) {
            Ok(key) => key,
            Err(e) => {
                error!("Failed to parse public key: {:?}", e);
                return Err(Box::new(e));
            }
        };

        let pub_key = match rsa::RsaPublicKey::from_pkcs1_pem(&key_str) {
            Ok(key) => key,
            Err(e) => {
                error!("Failed to parse public key: {:?}", e);
                return Err(Box::new(e));
            }
        };

        let shared_secret: [u8; 32] = rand::random();
        let mut shared_secret_guard = self.shared_secret.lock().await;
        *shared_secret_guard = shared_secret;

        let encrypted_shared_secret = get_encrypted_shared_secret(&pub_key, &shared_secret);
        let verify_token = get_encrypted_verify_token(&pub_key, encryption_request.verify_token);

        let encryption_response = EncryptionResponse::new(
            encrypted_shared_secret,
            verify_token,
            self.connection_type.clone(),
        );
        self.send_raw(encryption_response.serialize()).await;

        Ok(())
    }
}

fn get_encrypted_shared_secret(pub_key: &rsa::RsaPublicKey, shared_secret: &[u8; 32]) -> [u8; 256] {
    let shared_secret = shared::encryption::rsa::encrypt(pub_key, shared_secret);
    shared_secret
        .try_into()
        .expect("shared_secret length mismatch")
}

fn get_encrypted_verify_token(pub_key: &rsa::RsaPublicKey, verify_token: u32) -> [u8; 256] {
    let verify_token = shared::encryption::rsa::encrypt(pub_key, &verify_token.to_be_bytes());
    verify_token
        .try_into()
        .expect("verify_token length mismatch")
}
