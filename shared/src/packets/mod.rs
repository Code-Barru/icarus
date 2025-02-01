mod encryption_request;
mod encryption_response;
mod login_request;

use uuid::Uuid;

pub trait Packet {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum Error {
    UnknownPacket,
    ParseError,
}

pub fn from_packet_bytes(data: &[u8]) -> Result<PacketEnum, Error> {
    let packet_code = data[0];
    let data = &data[1..];
    match packet_code {
        0x01 => match LoginRequest::deserialize(data) {
            Ok(packet) => Ok(PacketEnum::LoginRequest(packet)),
            Err(_) => return Err(Error::ParseError),
        },
        0x02 => match EncryptionRequest::deserialize(data) {
            Ok(packet) => Ok(PacketEnum::EncryptionRequest(packet)),
            Err(_) => Err(Error::ParseError),
        },
        0x03 => match EncryptionResponse::deserialize(data) {
            Ok(packet) => Ok(PacketEnum::EncryptionResponse(packet)),
            Err(_) => Err(Error::ParseError),
        },
        _ => Err(Error::UnknownPacket),
    }
}

pub enum PacketEnum {
    LoginRequest(LoginRequest),
    EncryptionRequest(EncryptionRequest),
    EncryptionResponse(EncryptionResponse),
}

impl Packet for PacketEnum {
    fn serialize(&self) -> Vec<u8> {
        match self {
            PacketEnum::LoginRequest(packet) => packet.serialize(),
            PacketEnum::EncryptionRequest(packet) => packet.serialize(),
            PacketEnum::EncryptionResponse(packet) => packet.serialize(),
        }
    }

    fn deserialize(_data: &[u8]) -> Result<PacketEnum, Error> {
        unimplemented!()
    }
}

pub struct LoginRequest {
    pub uuid: Uuid,
}

pub struct EncryptionRequest {
    pub key_length: u16,
    pub public_key: Vec<u8>,
    pub verify_token: u32,
}

pub struct EncryptionResponse {
    pub shared_secret: [u8; 256],
    pub verify_token: [u8; 256],
}
