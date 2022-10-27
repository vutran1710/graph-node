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
        _mods: Vec<EntityModification>,
        _manifest_idx_and_name: Vec<(u32, String)>,
    ) -> () {
        let mut connection = self.connection.lock().unwrap();
        let channel = connection.open_channel(None).unwrap();
        let mut exchange_opts = ExchangeDeclareOptions::default();
        exchange_opts.durable = true;

        let exchange = channel
            .exchange_declare(ExchangeType::Direct, "graph-node", exchange_opts)
            .unwrap();

        let result = exchange
            .publish(Publish::new("hello there".as_bytes(), "hello"))
            .unwrap();
        warn!(self.logger, "Done publishing..."; "result" => format!("{:?}", result));
    }
}
