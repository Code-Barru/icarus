use uuid::Uuid;

use super::DownloadRequest;

impl super::Packet for DownloadRequest {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x08);
        data.append(&mut self.task_uuid.as_bytes().to_vec());
        data
    }

    fn deserialize(data: &[u8]) -> Result<Self, super::Error> {
        if data.len() != 16 {
            return Err(super::Error::InvalidData);
        }

        let task_uuid = match Uuid::from_slice(&data[0..16]) {
            Ok(uuid) => uuid,
            Err(_) => return Err(super::Error::InvalidData),
        };

        Ok(DownloadRequest { task_uuid })
    }
}

impl DownloadRequest {
    pub fn new(task_uuid: Uuid) -> Self {
        DownloadRequest { task_uuid }
    }
}
