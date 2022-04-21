use super::address::{GenericAddress, InternalGenericAddress};
use tokio::io::{AsyncRead, AsyncWrite};


pub trait TransportConnection: AsyncWrite + AsyncRead + Unpin {
    type TransportAddress: GenericAddress + 'static;

    fn local_address(&mut self) -> Option<Self::TransportAddress>;
    fn remote_address(&mut self) -> Option<Self::TransportAddress>;

}

pub(super) trait InternalTransportConnection: AsyncWrite + AsyncRead + Unpin {
    fn local_address(&mut self) -> Option<Box<dyn InternalGenericAddress>>;
    fn remote_address(&mut self) -> Option<Box<dyn InternalGenericAddress>>;
    
}

impl<T: TransportConnection> InternalTransportConnection for T {
    fn local_address(&mut self) -> Option<Box<dyn InternalGenericAddress>> {
       let internal_ret = T::local_address(self);
       match internal_ret {
           None => None,
           Some(d) => {
               let boxed: Box<dyn InternalGenericAddress> = Box::new(d);
               Some(boxed)
           }
       }
    }

    fn remote_address(&mut self) -> Option<Box<dyn InternalGenericAddress>> {
        let internal_ret = T::remote_address(self);
        match internal_ret {
            None => None,
            Some(d) => {
                let boxed: Box<dyn InternalGenericAddress> = Box::new(d);
                Some(boxed)
            }
        }
    }

}


