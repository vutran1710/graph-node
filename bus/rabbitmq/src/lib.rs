use amiquip::{Connection, ExchangeDeclareOptions, ExchangeType, Publish};
pub use async_trait::*;
use graph::components::bus::Bus;
use graph::components::bus::BusError;
use graph::prelude::BlockPtr;
use graph::prelude::EntityModification;
use graph::prelude::Logger;
use graph::slog::warn;
use std::sync::Arc;
use std::sync::Mutex;

pub struct RabbitmqBus {
    pub name: String,
    pub logger: Logger,
    connection: Arc<Mutex<Connection>>,
}

#[async_trait]
impl Bus for RabbitmqBus {
    fn new(connection_uri: String, logger: Logger) -> RabbitmqBus {
        let connection = Connection::insecure_open(&connection_uri).unwrap();
        RabbitmqBus {
            name: String::from("my rabbit store"),
            connection: Arc::new(Mutex::new(connection)),
            logger,
        }
    }
    async fn send_mapping_data(
        &self,
        block_ptr: BlockPtr,
        mods: Vec<EntityModification>,
        manifest_idx_and_names: Vec<(u32, String)>,
    ) -> Result<(), BusError> {
        Ok(())
    }
    async fn send_modification_data(&self) -> Result<(), BusError> {
        Ok(())
    }
}
