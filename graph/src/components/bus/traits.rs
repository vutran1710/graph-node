use super::err::BusError;
use crate::prelude::Logger;
use crate::tokio::sync::mpsc::UnboundedReceiver;
use crate::tokio::sync::mpsc::UnboundedSender;
use async_trait::async_trait;
use std::sync::Arc;
use std::sync::Mutex;

pub struct BusMessage {
    pub subgraph_id: String,
    pub value: String,
}

#[async_trait]
pub trait Bus: Send + Sync + Clone + 'static {
    async fn new(connection_uri: String, logger: Logger) -> Self;

    fn get_name(&self) -> &str;

    async fn send_plain_text(&self, value: BusMessage) -> Result<(), BusError>;

    fn mpsc_sender(&self) -> UnboundedSender<BusMessage>;

    fn mpsc_receiver(&self) -> Arc<Mutex<UnboundedReceiver<BusMessage>>>;
}
