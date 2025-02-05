use uuid::Uuid;

use super::TaskResponse;
use crate::models::TaskStatus;

impl super::Packet for TaskResponse {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x07);
        data.extend_from_slice(self.task_uuid.as_bytes());
        data.push(self.status.to_bytes());
        if self.result_size > 0 {
            data.extend_from_slice(&self.result_size.to_le_bytes());
            data.extend_from_slice(&self.result.clone().unwrap());
        } else {
            data.extend_from_slice(&(0 as u32).to_le_bytes());
        }
        data
    }

    fn deserialize(data: &[u8]) -> Result<Self, super::Error> {
        let task_uuid = match Uuid::from_slice(&data[0..16]) {
            Ok(uuid) => uuid,
            Err(_) => return Err(super::Error::InvalidData),
        };
        let status = TaskStatus::from(data[16]);
        let result_size = u32::from_le_bytes([data[17], data[18], data[19], data[20]]);
        if result_size == 0 {
            return Ok(Self {
                task_uuid,
                status,
                result_size,
                result: None,
            });
        }
        if data.len() < 21 + result_size as usize {
            return Err(super::Error::InvalidData);
        }
        let result = data[21..21 + result_size as usize].to_vec();
        Ok(Self {
            task_uuid,
            status,
            result_size,
            result: Some(result),
        })
    }
}

impl TaskResponse {
    pub fn new(task_uuid: Uuid, status: TaskStatus, result: Option<Vec<u8>>) -> Self {
        let result_size = match &result {
            Some(r) => r.len() as u32,
            None => 0,
        };
        Self {
            task_uuid,
            status,
            result_size,
            result,
        }
    }
}
