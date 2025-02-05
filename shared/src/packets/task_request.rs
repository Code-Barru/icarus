use super::TaskRequest;
use uuid::Uuid;

impl super::Packet for TaskRequest {
    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(0x06);
        data.extend_from_slice(self.task_uuid.as_bytes());
        data.push(self.task_type.to_bytes());
        if self.parameters_size > 0 {
            data.extend_from_slice(&self.parameters_size.to_le_bytes().to_vec());
            data.extend_from_slice(&self.parameters.clone().unwrap());
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
        let task_type = match data[16] {
            0x1 => super::TaskType::ShellCommand,
            _ => return Err(super::Error::InvalidData),
        };
        let parameters_size = u32::from_le_bytes([data[17], data[18], data[19], data[20]]);
        if data.len() < 21 + parameters_size as usize {
            return Err(super::Error::InvalidData);
        }
        if parameters_size == 0 {
            return Ok(Self {
                task_uuid,
                task_type,
                parameters_size,
                parameters: None,
            });
        }

        let parameters = data[21..21 + parameters_size as usize].to_vec();
        Ok(Self {
            task_uuid,
            task_type,
            parameters_size,
            parameters: Some(parameters),
        })
    }
}

impl TaskRequest {
    pub fn new(task_uuid: Uuid, task_type: super::TaskType, parameters: Option<String>) -> Self {
        let parameters = parameters.map(|p| p.into_bytes());

        if parameters.is_none() {
            return Self {
                task_uuid,
                task_type,
                parameters_size: 0,
                parameters: None,
            };
        }

        Self {
            task_uuid,
            task_type,
            parameters_size: parameters.clone().unwrap().len() as u32,
            parameters,
        }
    }
}
