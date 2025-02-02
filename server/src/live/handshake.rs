use rsa::{pkcs1::EncodeRsaPublicKey, pkcs8::LineEnding};
use shared::packets::{
    EncryptionRequest, EncryptionResponse as EncryptionResponseStruct, Packet,
    PacketEnum::{EncryptionResponse, LoginRequest},
};
use std::error::Error;
use tokio::net::TcpStream;
use tracing::{debug, error};
use uuid::Uuid;

use crate::live::rt_server::{receive, send};

use super::{Connection, RTServer};

impl RTServer {
    pub async fn handshake(
        &self,
        data: [u8; 17],
        mut socket: TcpStream,
    ) -> Result<Connection, Box<dyn Error>> {
        debug!("Got login request");
        let uuid = match self.get_login_request(data) {
            Ok(uuid) => uuid,
            Err(e) => {
                error!("Failed to get login request: {:?}", e);
                return Err(e.into());
            }
        };
        match self.verify_uuid(uuid).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to verify uuid: {:?}", e);
                return Err(e.into());
            }
        };

        let encryption_request = match self.get_encryption_request().await {
            Ok(encryption_request) => encryption_request,
            Err(e) => {
                error!("Failed to get encryption request: {:?}", e);
                return Err(e.into());
            }
        };
        match send(&mut socket, encryption_request.serialize()).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to send encryption request: {:?}", e);
                return Err(e.into());
            }
        };
        debug!("Sent encryption request");
        let encryption_response = match receive(&mut socket).await {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to receive encryption response: {:?}", e);
                return Err(e.into());
            }
        };

        let encryption_response = match shared::packets::from_packet_bytes(&encryption_response) {
            Ok(EncryptionResponse(packet)) => packet,
            _ => {
                return Err("Invalid packet".into());
            }
        };
        debug!("Got encryption response");

        let (shared_secret, verify_token) =
            match self.parse_encryption_response(encryption_response).await {
                Ok((shared_secret, verify_token)) => (shared_secret, verify_token),
                Err(e) => {
                    error!("Failed to parse encryption response: {:?}", e);
                    return Err(e.into());
                }
            };

        if verify_token != encryption_request.verify_token {
            return Err("Verify token mismatch".into());
        }
        debug!("Handshake successful");
        Ok(Connection::new(socket, uuid, shared_secret))
    }

    fn get_login_request(&self, data: [u8; 17]) -> Result<Uuid, Box<dyn Error>> {
        let packet = shared::packets::from_packet_bytes(&data);
        let uuid = match packet {
            Ok(LoginRequest(packet)) => packet.uuid,
            _ => {
                return Err("Invalid packet".into());
            }
        };
        Ok(uuid)
    }

    async fn verify_uuid(&self, uuid: Uuid) -> Result<(), Box<dyn Error>> {
        let state = self.state.lock().await;
        let _ = match state.get_agent(uuid).await {
            Ok(agent) => match agent {
                Some(_) => (),
                None => {
                    return Err(format!("Agent not found with uuid: {}", uuid).into());
                }
            },
            Err(e) => {
                return Err(e.into());
            }
        };
        Ok(())
    }

    async fn get_encryption_request(&self) -> Result<EncryptionRequest, Box<dyn Error>> {
        let token: u32 = rand::random();
        let pub_key = match self
            .rsa_key
            .lock()
            .await
            .to_public_key()
            .to_pkcs1_pem(LineEnding::LF)
        {
            Ok(key) => key,
            Err(e) => {
                error!("Failed to get public key: {:?}", e);
                return Err(e.into());
            }
        };

        Ok(EncryptionRequest::new(pub_key.as_bytes(), token))
    }

    async fn parse_encryption_response(
        &self,
        encryption_response: EncryptionResponseStruct,
    ) -> Result<([u8; 32], u32), Box<dyn Error>> {
        let key = self.rsa_key.lock().await;
        let key = key.clone();

        let shared_secret: [u8; 32] =
            shared::encryption::rsa::decrypt(&key, &encryption_response.shared_secret)
                .try_into()
                .map_err(|_| "Invalid shared secret length")?;
        let verify_token =
            shared::encryption::rsa::decrypt(&key, &encryption_response.verify_token);

        let verify_token = u32::from_be_bytes(
            verify_token
                .try_into()
                .map_err(|_| "Invalid verify token length")?,
        );

        Ok((shared_secret, verify_token))
    }
}
