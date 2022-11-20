use graph::components::bus::Bus;
use graph::components::bus::BusError;
use graph::prelude::DeploymentHash;
use graph::prelude::Logger;
use graph::tokio::sync::mpsc::unbounded_channel;
use graph::tokio::sync::mpsc::UnboundedReceiver;
use graph::tokio::sync::mpsc::UnboundedSender;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct TestBus {
    name: String,
    s: UnboundedSender<String>,
    r: Arc<Mutex<UnboundedReceiver<String>>>,
}

impl Bus for TestBus {
    fn new(_connection_uri: String, _logger: Logger) -> TestBus {
        let (s, r) = unbounded_channel();
        TestBus {
            name: "test-bus".to_owned(),
            s,
            r: Arc::new(Mutex::new(r)),
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn mpsc_sender(&self) -> UnboundedSender<String> {
        self.s.clone()
    }

    fn mpsc_receiver(&self) -> Arc<Mutex<UnboundedReceiver<String>>> {
        self.r.clone()
    }

    fn send_plain_text(&self, _text: String, _deployment: DeploymentHash) -> Result<(), BusError> {
        Ok(())
    }
}
