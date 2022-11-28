mod schemas;

use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::client::Client;
use graph::components::bus::Bus;
use graph::components::bus::BusError;
use graph::components::bus::BusMessage;
use graph::prelude::async_trait;
use graph::prelude::Logger;
use graph::slog::error;
use graph::slog::warn;
use graph::tokio::sync::mpsc::UnboundedReceiver;
use schemas::{serialize_using_avro, Demo};
use serde::{Deserialize, Serialize};
use std::string::String;

pub struct GooglePubSub {
    logger: Logger,
    client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
struct GraphNodeBusMessage {
    topic: String,
    data: Vec<String>,
}

impl TryFrom<BusMessage> for GraphNodeBusMessage {
    type Error = BusError;
    fn try_from(msg: BusMessage) -> Result<Self, Self::Error> {
        let topic = msg
            .value
            .iter()
            .nth(0)
            .ok_or(BusError::BadMessage("No topic found".to_owned()))?
            .to_owned();

        let data = msg.value[1..].to_owned();

        let result = GraphNodeBusMessage { topic, data };
        Ok(result)
    }
}

#[async_trait]
impl Bus for GooglePubSub {
    async fn new(_: String, logger: Logger) -> GooglePubSub {
        let client = Client::default().await.unwrap();
        GooglePubSub { client, logger }
    }

    fn get_name(&self) -> &str {
        "google-pubsub"
    }

    async fn send_plain_text(&self, bus_msg: BusMessage) -> Result<(), BusError> {
        let message = GraphNodeBusMessage::try_from(bus_msg)?;

        warn!(self.logger, "Message received"; "msg" => format!("{:?}", message));

        let topic = self.client.topic(&message.topic);

        if !topic.exists(None, None).await.unwrap() {
            return Err(BusError::NoRoutingDefinition);
        }

        warn!(self.logger, "Sending to topic"; "topic" => message.topic.clone());

        let publisher = topic.new_publisher(None);
        let mut msg = PubsubMessage::default();

        let data: Vec<u8> = self.parse_data(message)?;
        msg.data = data;

        let awaiter = publisher.publish(msg).await;

        awaiter
            .get(None)
            .await
            .map_err(|e| BusError::SendPlainTextError(e.to_string()))?;

        Ok(())
    }

    async fn start(&self, mut receiver: UnboundedReceiver<BusMessage>) -> () {
        warn!(&self.logger, "Loop to consume data from bus");
        while let Some(data) = receiver.recv().await {
            warn!(
                self.logger,
                "Sending to Bus";
                "subgraph_id" => &data.subgraph_id,
                "value" => format!("{:?}", data.value),
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

impl GooglePubSub {
    fn parse_data(&self, message: GraphNodeBusMessage) -> Result<Vec<u8>, BusError> {
        let topic = message.topic.as_str();
        match topic {
            "demo" => {
                let event = message.data[0].to_owned();
                let value = message.data[1]
                    .parse::<i64>()
                    .map_err(|e| BusError::BadMessage(e.to_string()))?;
                let data = Demo { event, value };
                serialize_using_avro(topic, data).map_err(|e| BusError::BadMessage(e))
            }
            _ => Ok(message.data.concat().into_bytes()),
        }
    }
}
