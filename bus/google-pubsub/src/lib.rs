use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::client::ClientConfig;
use graph::components::bus::Bus;
use graph::components::bus::BusError;
use graph::prelude::async_trait;
use graph::prelude::DeploymentHash;
use graph::prelude::Logger;
use graph::tokio;
use graph::tokio::sync::mpsc::unbounded_channel;
use graph::tokio::sync::mpsc::UnboundedReceiver;
use graph::tokio::sync::mpsc::UnboundedSender;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct GooglePubSub {
    sender: UnboundedSender<String>,
    receiver: Arc<Mutex<UnboundedReceiver<String>>>,
    logger: Logger,
    client: Client,
}

fn pubsub_config_from_string(value: String) -> ClientConfig {
    let mut result = ClientConfig::default();
    let params = value.split(":").map(String::from).collect::<Vec<String>>();
    let pool_size = params.get(0).unwrap();
    let project_id = params.get(1).unwrap();
    let endpoint = params.get(2).unwrap();
    result.pool_size = usize::from_str_radix(pool_size, 10).ok();
    result.project_id = Some(project_id.to_owned());
    result.endpoint = endpoint.to_owned();
    result
}

#[async_trait]
impl Bus for GooglePubSub {
    fn new(connection_uri: String, logger: Logger) -> GooglePubSub {
        let builder = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let config = pubsub_config_from_string(connection_uri);
        let client = builder.block_on(async move { Client::new(config).await.unwrap() });
        let (sender, receiver) = unbounded_channel();

        GooglePubSub {
            client,
            logger,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    fn get_name(&self) -> &str {
        "Google PubSub"
    }

    fn mpsc_sender(&self) -> UnboundedSender<String> {
        self.sender.clone()
    }

    fn mpsc_receiver(&self) -> Arc<Mutex<UnboundedReceiver<String>>> {
        self.receiver.clone()
    }

    fn send_plain_text(&self, value: String, subgraph_id: DeploymentHash) -> Result<(), BusError> {
        Ok(())
    }
}
