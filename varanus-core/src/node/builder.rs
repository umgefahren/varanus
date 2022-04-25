use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::node::NodeState;
use crate::transport::{InternalTransportProtocol, TransportProtocol};
use crate::transport::address::InternalTransportIdentifier;

fn transport_vec_to_map(input: Vec<Box<dyn InternalTransportProtocol>>) -> HashMap<Box<dyn InternalTransportIdentifier>, Arc<dyn InternalTransportProtocol>> {
	let mut ret = HashMap::new();
	input
		.into_iter()
		.for_each(|boxed| {
			let identifier = boxed.transport_identifier();
			let arc = Arc::from(boxed);
			ret.insert(identifier, arc);
		});
	ret
}


pub struct NodeStateBuilder<Dt: TransportProtocol> {
	default_transport: Option<Dt>,
	alternate_transports: Vec<Box<dyn InternalTransportProtocol>>,
}


impl<Dt: TransportProtocol> NodeStateBuilder<Dt> {
	pub(crate) fn new() -> Self {
		Self {
			default_transport: None,
			alternate_transports: Vec::new(),
		}
	}

	pub fn add_default_transport(mut self, transport: Dt) -> Self {
		self.default_transport = Some(transport);
		self
	}

	pub fn add_generic_transport<T: TransportProtocol>(mut self, transport: T) -> Self {
		let dynamic_transport = Box::new(transport);
		self.alternate_transports.push(dynamic_transport);
		self
	}

	pub fn build(self) -> NodeState<Dt> {
		let transport_map = transport_vec_to_map(self.alternate_transports);
		let default_transport = self.default_transport.expect("default transport wasn't specified");
		NodeState {
			default_transport,
			alternate_transports: RwLock::new(transport_map),
		}
	}
}

