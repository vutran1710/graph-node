use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::client::ClientConfig;
use graph::components::bus::Bus;
use graph::components::bus::BusError;
use graph::components::bus::BusMessage;
use graph::prelude::async_trait;
use graph::prelude::Logger;
use graph::slog::error;
use graph::slog::warn;
use graph::tokio::sync::mpsc::UnboundedReceiver;
use graph::url::Url;
use std::collections::HashSet;
use std::string::String;

#[derive(Clone)]
pub struct GooglePubSub {
    project_id: String,
    topics: HashSet<String>,
    logger: Logger,
    client: Client,
}

fn pubsub_config_from_string(msg: String) -> ClientConfig {
    /* example connection string:
    pubsub://localhost:8080/PROJECT_ID?pool_size=10
     */
    let url = Url::parse(&msg).unwrap();
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
        warn!(logger, "PubSub Config"; "msg" => format!("{:?}", config));
        let project_id = config.project_id.clone().unwrap();
        let client = Client::new(config).await.unwrap();

        GooglePubSub {
            topics: HashSet::new(),
            project_id,
            client,
            logger,
        }
    }

    fn get_name(&self) -> &str {
        &self.project_id
    }

    async fn send_plain_text(&self, message: BusMessage) -> Result<(), BusError> {
        let result = async {
            let topic = self.client.topic(&message.subgraph_id);

            if !topic.exists(None, None).await.unwrap() {
                topic.create(None, None, None).await.unwrap();
            }

            warn!(self.logger, "Topic to send to"; "fully_qualified_name" => topic.fully_qualified_name());

            let publisher = topic.new_publisher(None);
            let mut msg = PubsubMessage::default();
            msg.data = message.value.into();
            let awaiter = publisher.publish(msg).await;
            awaiter.get(None).await
        }
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(status) => Err(BusError::SendPlainTextError(status.to_string())),
        }
    }

    async fn start(&self, mut receiver: UnboundedReceiver<BusMessage>) -> () {
        warn!(&self.logger, "Loop to consume data from bus");
        while let Some(data) = receiver.recv().await {
            warn!(
                self.logger,
                "Sending to Bus";
                "subgraph_id" => &data.subgraph_id,
                "value" => &data.value,
            );

            if let Err(err) = self.send_plain_text(data).await {
                error!(
                    self.logger,
                    "Failed sending to Bus";
                    "reason" => format!("{:?}", err)
                );
            }
        }
    }
}
