use std::{collections::HashMap, sync::Arc};

use crate::protocol::{identifier::ProtocolIdentifier, InternalGenericProtocol};

pub struct NodeState {
    generic_protocols: HashMap<ProtocolIdentifier, Arc<dyn InternalGenericProtocol>>,
}

