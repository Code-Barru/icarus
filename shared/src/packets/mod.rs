mod encryption_request;
mod encryption_response;
mod login_request;
mod task_request;
mod task_response;
mod update_request;
mod update_response;

use uuid::Uuid;

use crate::models::{ConnectionType, TaskStatus, TaskType};

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
        0x04 => match UpdateRequest::deserialize(data) {
            Ok(packet) => Ok(PacketEnum::UpdateRequest(packet)),
            Err(_) => Err(Error::ParseError),
        },
        0x05 => match UpdateResponse::deserialize(data) {
            Ok(packet) => Ok(PacketEnum::UpdateResponse(packet)),
            Err(_) => Err(Error::ParseError),
        },
        0x06 => match TaskRequest::deserialize(data) {
            Ok(packet) => Ok(PacketEnum::TaskRequest(packet)),
            Err(_) => Err(Error::ParseError),
        },
        0x07 => match TaskResponse::deserialize(data) {
            Ok(packet) => Ok(PacketEnum::TaskResponse(packet)),
            Err(_) => Err(Error::ParseError),
        },
        _ => Err(Error::UnknownPacket),
    }
}

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
    InvalidData,
}

pub enum PacketEnum {
    LoginRequest(LoginRequest),
    EncryptionRequest(EncryptionRequest),
    EncryptionResponse(EncryptionResponse),
    UpdateRequest(UpdateRequest),
    UpdateResponse(UpdateResponse),
    TaskRequest(TaskRequest),
    TaskResponse(TaskResponse),
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
    pub connection_type: ConnectionType,
}

#[derive(Debug)]
pub struct UpdateRequest {
    pub agent_hash: String,
}

#[derive(Debug)]
pub struct UpdateResponse {
    pub need_update: bool,
}

#[derive(Debug)]
pub struct TaskRequest {
    pub task_uuid: Uuid,
    pub task_type: TaskType,
    pub parameters_size: u32,
    pub parameters: Option<Vec<u8>>,
}
#[derive(Debug)]
pub struct TaskResponse {
    pub task_uuid: Uuid,
    pub status: TaskStatus,
    pub result_size: u32,
    pub result: Option<Vec<u8>>,
}
