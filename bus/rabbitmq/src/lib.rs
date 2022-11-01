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
    async fn send_trigger_data(&self) -> Result<(), BusError> {
        Ok(())
    }
    async fn send_modification_data(
        &self,
        block_ptr: BlockPtr,
        mods: Vec<EntityModification>,
        manifest_idx_and_names: Vec<(u32, String)>,
    ) -> Result<(), BusError> {
        let mut connection = self.connection.lock().unwrap();
        let channel = connection.open_channel(None).unwrap();
        let mut exchange_opts = ExchangeDeclareOptions::default();
        exchange_opts.durable = true;

        let data = format!("{:?}", mods);
        let data_as_bytes = data.as_bytes();
        let routing_key = "mods";

        for (_, subgraph_id) in manifest_idx_and_names {
            let exchange = channel
                .exchange_declare(
                    ExchangeType::Fanout,
                    format!("{:?}", subgraph_id),
                    exchange_opts.clone(),
                )
                .unwrap();

            let _result = exchange.publish(Publish::new(data_as_bytes.clone(), routing_key));
        }

        warn!(self.logger, "Done publishing...";);
        Ok(())
    }
}
