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
use url::Url;

#[derive(Clone)]
pub struct GooglePubSub {
    project_id: String,
    sender: UnboundedSender<String>,
    receiver: Arc<Mutex<UnboundedReceiver<String>>>,
    logger: Logger,
    client: Client,
}

fn pubsub_config_from_string(value: String) -> ClientConfig {
    /* example connection string:
    pubsub://localhost:8080/PROJECT_ID?pool_size=10
     */
    let url = Url::parse(&value).unwrap();
    let mut result = ClientConfig::default();
    result.project_id = Some(url.path_segments().unwrap().next().unwrap().to_owned());
    result.endpoint = url.host_str().unwrap().to_owned();
    result
}

#[async_trait]
impl Bus for GooglePubSub {
    fn new(connection_uri: String, logger: Logger) -> GooglePubSub {
        let builder = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let config = pubsub_config_from_string(connection_uri);
        let project_id = config.project_id.clone().unwrap();
        let client = builder.block_on(async move { Client::new(config).await.unwrap() });
        let (sender, receiver) = unbounded_channel();

        GooglePubSub {
            project_id,
            client,
            logger,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    fn get_name(&self) -> &str {
        &self.project_id
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
