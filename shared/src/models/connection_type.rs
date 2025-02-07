use super::ConnectionType;

impl ConnectionType {
    pub fn to_bytes(&self) -> u8 {
        match self {
            ConnectionType::Main => 0x1,
            ConnectionType::Update => 0x2,
            ConnectionType::FileUpload => 0x3,
            ConnectionType::FileDownload => 0x4,
        }
    }
}

impl From<u8> for ConnectionType {
    fn from(byte: u8) -> Self {
        match byte {
            0x1 => ConnectionType::Main,
            0x2 => ConnectionType::Update,
            0x3 => ConnectionType::FileUpload,
            0x4 => ConnectionType::FileDownload,
            _ => ConnectionType::Main,
        }
    }
}
