use graph::components::bus::Bus;
use graph::components::bus::BusError;
use graph::prelude::DeploymentHash;
use graph::prelude::Logger;

#[derive(Clone)]
pub struct TestBus {
    name: String,
}

impl Bus for TestBus {
    fn new(_connection_uri: String, _logger: Logger) -> TestBus {
        TestBus {
            name: "test-bus".to_owned(),
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn send_plain_text(&self, _text: String, _deployment: DeploymentHash) -> Result<(), BusError> {
        Ok(())
    }
}
