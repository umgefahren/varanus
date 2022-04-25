pub mod builder;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::node::builder::NodeStateBuilder;
use crate::transport::address::{InternalTransportIdentifier, TransportIdentifier};
use crate::transport::{InternalTransportProtocol, TransportProtocol};

pub enum GenericResolutionError {
    TransportNotFound,
    RwLockError
}

pub struct NodeState<Dt: TransportProtocol> {
    default_transport: Dt,
    alternate_transports: RwLock<HashMap<Box<dyn InternalTransportIdentifier>, Arc<dyn InternalTransportProtocol>>>,
}

impl<Dt: TransportProtocol> NodeState<Dt> {
    pub fn dial_default(&self, address: &Dt::TransportAddress) -> Dt::TransportFuture {
        self.default_transport.dial(address)
    }

    pub fn dial_generic<T: TransportProtocol>(&self, address: &T::TransportAddress) -> Result<T::TransportFuture, GenericResolutionError> {
        let transport_identifier = T::TransportIdentifier::new();
        let generic_transport_identifier: Box<dyn InternalTransportIdentifier> = Box::new(transport_identifier);
        let generic_transport_protocol = {
            let r_lock = match self.alternate_transports.read() {
                Ok(d) => d,
                Err(_) => {
                    return Err(GenericResolutionError::RwLockError);
                }
            };
            match r_lock.get(&generic_transport_identifier) {
                Some(d) => {
                    d.clone().as_dyn_arc()
                },
                None => {
                    return Err(GenericResolutionError::TransportNotFound);
                }
            }
        };
        let transport_protocol: Arc<T> = match generic_transport_protocol.downcast::<T>() {
            Ok(d) => d,
            Err(_) => {
                return Err(GenericResolutionError::TransportNotFound)
            }
        };

        Ok(transport_protocol.dial(address))
    }

    pub fn builder() -> NodeStateBuilder<Dt> {
        NodeStateBuilder::new()
    }
}