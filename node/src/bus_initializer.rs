use bus_rabbitmq::RabbitmqBus;
use graph::components::bus::Bus;
use graph::slog::warn;

pub struct BusInitializer;

pub enum BusScheme {
    RabbitMQ,
}

impl BusInitializer {
    pub fn new(uri: Option<String>, logger: graph::slog::Logger) -> Option<impl Bus> {
        match BusInitializer::get_bus_scheme(&uri) {
            Some(BusScheme::RabbitMQ) => {
                warn!(logger, "Starting Bus of RabbitMQ";);
                Some(RabbitmqBus::new(uri.unwrap(), logger))
            }
            _ => {
                warn!(logger, "No bus at work";);
                None
            }
        }
    }
    pub fn get_bus_scheme(uri: &Option<String>) -> Option<BusScheme> {
        if uri.is_none() {
            return None;
        }

        let scheme = &uri.clone().unwrap()[0..8];
        match String::from(scheme).as_str() {
            "amqp" => Some(BusScheme::RabbitMQ),
            _ => None,
        }
    }
}
