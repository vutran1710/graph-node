use graph::components::bus::Bus;
use graph::components::bus::BusError;
use graph::components::bus::BusMessage;
use graph::prelude::async_trait;
use graph::prelude::Logger;
use graph::tokio::sync::mpsc::UnboundedReceiver;

#[derive(Clone)]
pub struct TestBus {
    name: String,
}

#[async_trait]
impl Bus for TestBus {
    async fn new(_connection_uri: String, _logger: Logger) -> TestBus {
        TestBus {
            name: "test-bus".to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    async fn send_plain_text(&self, _msg: BusMessage) -> Result<(), BusError> {
        Ok(())
    }

    async fn start(&self, _r: UnboundedReceiver<BusMessage>) -> () {
        ()
    }
}
