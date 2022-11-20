use super::err::BusError;
use crate::prelude::DeploymentHash;
use crate::prelude::Logger;
use async_trait::async_trait;

#[async_trait]
pub trait Bus: Send + Sync + 'static {
    fn new(connection_uri: String, logger: Logger) -> Self;

    fn get_name(&self) -> &str;

    fn send_plain_text(&self, value: String, subgraph_id: DeploymentHash) -> Result<(), BusError>;
}
