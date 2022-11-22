use amiquip::{Connection, ExchangeDeclareOptions, ExchangeType, Publish};
use graph::components::bus::Bus;
use graph::components::bus::BusError;
use graph::prelude::async_trait;
use graph::prelude::DeploymentHash;
use graph::prelude::Logger;
use graph::tokio::sync::mpsc::unbounded_channel;
use graph::tokio::sync::mpsc::UnboundedReceiver;
use graph::tokio::sync::mpsc::UnboundedSender;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct RabbitmqBus {
    pub name: String,
    pub logger: Logger,
    connection: Arc<Mutex<Connection>>,
    sender: UnboundedSender<String>,
    receiver: Arc<Mutex<UnboundedReceiver<String>>>,
}

#[async_trait]
impl Bus for RabbitmqBus {
    fn new(connection_uri: String, logger: Logger) -> RabbitmqBus {
        let connection = Connection::insecure_open(&connection_uri).unwrap();
        let (sender, receiver) = unbounded_channel();

        RabbitmqBus {
            name: String::from("my rabbit store"),
            connection: Arc::new(Mutex::new(connection)),
            logger,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    fn mpsc_sender(&self) -> UnboundedSender<String> {
        self.sender.clone()
    }

    fn mpsc_receiver(&self) -> Arc<Mutex<UnboundedReceiver<String>>> {
        self.receiver.clone()
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn send_plain_text(&self, text: String, deployment: DeploymentHash) -> Result<(), BusError> {
        // NOTE: this is very UGLY, but we are doing POC, so its ok for now
        let data_as_bytes = text.as_bytes();
        let routing_key = deployment.as_str();
        let exchange_name = deployment.as_str();
        let mut exchange_opts = ExchangeDeclareOptions::default();
        exchange_opts.durable = true;

        return match self.connection.lock() {
            Ok(mut conn) => match conn.open_channel(None) {
                Ok(channel) => {
                    match channel.exchange_declare(
                        ExchangeType::Topic,
                        exchange_name,
                        exchange_opts.clone(),
                    ) {
                        Ok(exchange) => {
                            match exchange.publish(Publish::new(data_as_bytes.clone(), routing_key))
                            {
                                Ok(()) => Ok(()),
                                Err(e) => Err(BusError::SendPlainTextError(e.to_string())),
                            }
                        }
                        Err(e) => Err(BusError::SendPlainTextError(e.to_string())),
                    }
                }
                Err(e) => Err(BusError::SendPlainTextError(e.to_string())),
            },
            Err(e) => Err(BusError::SendPlainTextError(e.to_string())),
        };
    }
}