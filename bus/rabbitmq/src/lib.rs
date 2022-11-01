use amiquip::{Connection, ExchangeDeclareOptions, ExchangeType, Publish};
pub use async_trait::*;
use graph::components::bus::Bus;
use graph::components::bus::BusError;
use graph::prelude::BlockPtr;
use graph::prelude::DeploymentHash;
use graph::prelude::EntityModification;
use graph::prelude::Logger;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
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
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
    async fn send_plain_text(
        &self,
        text: String,
        subgraph_id: DeploymentHash,
    ) -> Result<(), BusError> {
        let mut connection = self.connection.lock().unwrap();
        let channel = connection.open_channel(None).unwrap();
        let mut exchange_opts = ExchangeDeclareOptions::default();
        exchange_opts.durable = true;

        let data_as_bytes = text.as_bytes();
        let routing_key = "bus-test";
        let exchange = channel
            .exchange_declare(
                ExchangeType::Fanout,
                format!("{:?}", subgraph_id),
                exchange_opts.clone(),
            )
            .unwrap();

        let _send = exchange.publish(Publish::new(data_as_bytes.clone(), routing_key));
        Ok(())
    }
    async fn send_trigger_data(&self) -> Result<(), BusError> {
        Ok(())
    }
    async fn send_modification_data(
        &self,
        _block_ptr: BlockPtr,
        _mods: Vec<EntityModification>,
        _manifest_idx_and_names: Vec<(u32, String)>,
    ) -> Result<(), BusError> {
        Ok(())
    }
}
