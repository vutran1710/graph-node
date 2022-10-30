use crate::prelude::thiserror::Error;
use std::fmt::Display;

#[derive(Error, Debug)]
pub enum BusError {
    SendMappingError(String),
    SendModificationError(String),
}

impl Display for BusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BusError::SendMappingError(err) => {
                write!(f, "BusError: sending mapping failed => {:?}", err)
            }
            BusError::SendModificationError(err) => {
                write!(f, "BusError: sending modifications failed => {}", err)
            }
        }
    }
}
