use amiquip::{Connection, Exchange, ExchangeDeclareOptions, ExchangeType, Publish};
pub use async_trait::*;
use graph::components::store::EventStore;
use graph::prelude::BlockPtr;
use graph::prelude::EntityModification;
use graph::prelude::Logger;
use graph::slog::warn;
use std::sync::Arc;
use std::sync::Mutex;

pub struct RabbitEventStore {
    pub name: String,
    connection: Arc<Mutex<Connection>>,
    logger: Logger,
}

impl RabbitEventStore {
    pub fn new(logger: Logger) -> RabbitEventStore {
        let connection = Connection::insecure_open("amqp://guest:guest@localhost:5672").unwrap();
        RabbitEventStore {
            name: String::from("my rabbit store"),
            connection: Arc::new(Mutex::new(connection)),
            logger,
        }
    }
}

#[async_trait]
impl EventStore for RabbitEventStore {
    async fn publish_data(
        &self,
        _block_ptr_to: BlockPtr,
        mods: Vec<EntityModification>,
        manifest_idx_and_name: Vec<(u32, String)>,
    ) -> () {
        let mut connection = self.connection.lock().unwrap();
        let channel = connection.open_channel(None).unwrap();
        let mut exchange_opts = ExchangeDeclareOptions::default();
        exchange_opts.durable = true;

        let data = format!("{:?}", mods);
        let data_as_bytes = data.as_bytes();
        let routing_key = "mods";

        for (_, subgraph_id) in manifest_idx_and_name {
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
    }
}
