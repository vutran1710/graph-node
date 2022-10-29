use super::*;

#[async_trait]
pub trait Bus: Send + Sync + 'static {
    pub fn new(connection_uri: String) -> Bus;
    pub async fn send_mapping_data() -> Result<(), BusError::SendMappingError>;
    pub async fn send_modification_data() -> Result<(), BusError::SendModificationError>;
}
