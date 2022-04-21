use std::error::Error;

use crate::protocol::GenericProtocol;

use self::{connection::TransportConnection, address::{TransportIdentifier, GenericAddress}};



pub mod address;
pub mod connection;

pub trait TransportProtocol: GenericProtocol {
    type Connection: TransportConnection<TransportAddress = Self::TransportAddress>;
    type TransportIdentifier: TransportIdentifier;
    type TransportAddress: GenericAddress;
    type TransportError: Error + Send + Sync;

    fn dial(&self, address: &Self::TransportAddress) -> Result<Self::Connection, Self::TransportError>;
}
