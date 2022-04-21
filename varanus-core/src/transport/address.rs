use std::cmp::Ordering;
use std::fmt::Debug;

pub trait TransportIdentifier: PartialOrd + Clone + Send + Sync + Debug {
    fn to_string() -> String;
    fn from_string(input: &str) -> Self;
}


pub(super) trait InternalTransportIdentifier: Send + Sync + Debug {
    fn to_self_string(&self) -> String;
    fn partial_cmp_str(&self, other: &str) -> Option<Ordering>;
}

impl<T: TransportIdentifier> InternalTransportIdentifier for T {
    fn to_self_string(&self) -> String {
        T::to_string()
    }

    fn partial_cmp_str(&self, other: &str) -> Option<Ordering> {
       let rhs = T::from_string(other);

       T::partial_cmp(self, &rhs)
    }

}

impl PartialEq for Box<dyn InternalTransportIdentifier> {
    fn eq(&self, other: &Self) -> bool {
        let other_str = other.to_self_string();
        let ordering_opt = self.partial_cmp_str(&other_str);
        if ordering_opt.is_none() {
            return false
        }
        let ordering = ordering_opt.unwrap();

        matches!(ordering, Ordering::Equal)
    }
}



impl PartialOrd for Box<dyn InternalTransportIdentifier> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let other_str = other.to_self_string();
        self.partial_cmp_str(&other_str)
    }
}

pub trait GenericAddress: Clone + Send + Sync + Debug {
    type Associated: TransportIdentifier + 'static;

    fn transport_identifier() -> Self::Associated;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(&mut self, input: &[u8]);
}

pub(super) trait InternalGenericAddress: Send + Sync + Debug {
    fn get_identifier(&self) -> Box<dyn InternalTransportIdentifier>;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(&mut self, input: &[u8]);
}

impl<T: GenericAddress> InternalGenericAddress for T {
    fn get_identifier(&self) -> Box<dyn InternalTransportIdentifier> {
       let ret = T::transport_identifier();
       Box::new(ret)
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes()
    }

    fn from_bytes(&mut self, input: &[u8]) {
        self.from_bytes(input);
    }
}
