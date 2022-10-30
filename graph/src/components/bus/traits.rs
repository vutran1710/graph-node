use super::*;
use crate::prelude::Logger;
use async_trait::async_trait;

#[async_trait]
pub trait Bus: Send + Sync + 'static {
    fn new(connection_uri: String, logger: Logger) -> Self;
    async fn send_mapping_data() -> Result<(), BusError>;
    async fn send_modification_data() -> Result<(), BusError>;
}
