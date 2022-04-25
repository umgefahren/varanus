use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

pub trait TransportIdentifier: PartialOrd + Clone + Send + Sync + Debug + Any {
    fn new() -> Self;
    fn string() -> String;
    fn from_string(input: &str) -> Self;
}


pub(crate) trait InternalTransportIdentifier: Send + Sync + Debug {
    fn to_self_string(&self) -> String;
    fn inner_typeid(&self) -> TypeId;
}

impl<T: TransportIdentifier> InternalTransportIdentifier for T {
    fn to_self_string(&self) -> String {
        T::string()
    }

    fn inner_typeid(&self) -> TypeId {
        T::type_id(self)
    }
}



impl PartialEq for Box<dyn InternalTransportIdentifier> {
    fn eq(&self, other: &Self) -> bool {
        let other_type_id = other.type_id();
        let self_type_id = self.type_id();
        self_type_id.eq(&other_type_id)
    }
}

impl Eq for Box<dyn InternalTransportIdentifier> {

}

impl Hash for Box<dyn InternalTransportIdentifier> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let self_type_id = self.type_id();
        self_type_id.hash(state);
    }
}

pub trait GenericAddress: Clone + Send + Sync + Debug + Any {
    type Associated: TransportIdentifier + 'static;

    fn transport_identifier() -> Self::Associated;
}

pub(crate) trait InternalGenericAddress: Any + Send + Sync + Debug {
    fn get_identifier(&self) -> Box<dyn InternalTransportIdentifier>;
}

impl<T: GenericAddress> InternalGenericAddress for T {
    fn get_identifier(&self) -> Box<dyn InternalTransportIdentifier> {
       let ret = T::transport_identifier();
       Box::new(ret)
    }
}
