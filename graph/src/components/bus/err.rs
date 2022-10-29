#[derive(Error, Debug)]
pub enum BusError {
    SendMappingError(String),
    SendModificationError(String),
}
