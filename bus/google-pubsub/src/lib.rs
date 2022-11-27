use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::client::ClientConfig;
use graph::components::bus::Bus;
use graph::components::bus::BusError;
use graph::prelude::async_trait;
use graph::prelude::DeploymentHash;
use graph::prelude::Logger;
use graph::slog::warn;
use graph::tokio::sync::mpsc::unbounded_channel;
use graph::tokio::sync::mpsc::UnboundedReceiver;
use graph::tokio::sync::mpsc::UnboundedSender;
use graph::url::Url;
use std::collections::HashSet;
use std::string::String;
use std::sync::Arc;
use std::sync::Mutex;

pub struct GooglePubSub {
    project_id: String,
    topics: HashSet<String>,
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
    result.endpoint = format!(
        "http://{}:{}",
        url.host_str().unwrap(),
        url.port().unwrap_or(22)
    );
    result
}

#[async_trait]
impl Bus for GooglePubSub {
    async fn new(connection_uri: String, logger: Logger) -> GooglePubSub {
        let config = pubsub_config_from_string(connection_uri);
        warn!(logger, "PubSub Config"; "value" => format!("{:?}", config));
        let project_id = config.project_id.clone().unwrap();
        let client = Client::new(config).await.unwrap();
        let (sender, receiver) = unbounded_channel();

        GooglePubSub {
            topics: HashSet::new(),
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

    async fn send_plain_text(
        &self,
        value: String,
        subgraph_id: DeploymentHash,
    ) -> Result<(), BusError> {
        let result = async {
            let topic_name = subgraph_id.as_str();
            let topic = self.client.topic(topic_name);

            if !topic.exists(None, None).await.unwrap() {
                topic.create(None, None, None).await.unwrap();
            }

            let publisher = topic.new_publisher(None);
            let mut msg = PubsubMessage::default();
            msg.data = value.into();
            let awaiter = publisher.publish(msg).await;
            awaiter.get(None).await
        }
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(status) => Err(BusError::SendPlainTextError(status.to_string())),
        }
    }
}
