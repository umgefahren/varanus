use std::any::Any;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use pin_project::pin_project;

use crate::protocol::GenericProtocol;
use crate::transport::address::InternalTransportIdentifier;
use crate::transport::connection::InternalTransportConnection;

use self::{connection::TransportConnection, address::{TransportIdentifier, GenericAddress}};

pub mod address;
pub mod connection;

pub trait TransportProtocol: GenericProtocol + Any {
    type Connection: TransportConnection<TransportAddress = Self::TransportAddress>;
    type TransportIdentifier: TransportIdentifier;
    type TransportAddress: GenericAddress;
    type TransportError: Error + Send + Sync + 'static;
    type TransportFuture: Future<Output = Result<Self::Connection, Self::TransportError>> + 'static;

    fn dial(&self, address: &Self::TransportAddress) -> Self::TransportFuture;
}

#[pin_project]
pub(super) struct InternalDial<C: TransportConnection, E: 'static + Error + Send + Sync, F: Future<Output = Result<C, E>>> {
    #[pin]
    internal: F,
}

impl<C: TransportConnection, E: 'static + Error + Send + Sync, F: Future<Output = Result<C, E>>> InternalDial<C, E, F> {
    pub(crate) fn new(internal: F) -> Self {
        Self {
            internal
        }
    }
}

impl<C: TransportConnection, E: Error + Send + Sync, F: Future<Output = Result<C, E>>> Future for InternalDial<C, E, F> {
    type Output = Result<Box<dyn InternalTransportConnection>, Box<dyn Error + Send + Sync>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let poll_result: Poll<Result<C, E>> = this.internal.poll(cx);
        match poll_result {
            Poll::Ready(e) => {
                let out = e.map(|e| {
                    let boxed: Box<dyn InternalTransportConnection> = Box::new(e);
                    boxed
                }).map_err(|e| {
                    let boxed: Box<dyn Error + Send + Sync> = Box::new(e);
                    boxed
                });
                Poll::Ready(out)
            }
            Poll::Pending => Poll::Pending
        }
    }
}

pub(crate) trait InternalTransportProtocol: 'static + Any + Send + Sync {
    fn transport_identifier(&self) -> Box<dyn InternalTransportIdentifier>;

    fn dial(&self, address: Box<dyn Any>) -> Box<dyn Future<Output = Result<Box<dyn InternalTransportConnection>, Box<dyn Error + Send + Sync>>>>;

    fn as_dyn_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static>;
}


impl<T: TransportProtocol> InternalTransportProtocol for T {
    fn transport_identifier(&self) -> Box<dyn InternalTransportIdentifier> {
        let transport_identifier = T::TransportIdentifier::new();
        let boxed: Box<dyn InternalTransportIdentifier> = Box::new(transport_identifier);
        boxed
    }

    fn dial(&self, address: Box<dyn Any>) -> Box<dyn Future<Output=Result<Box<dyn InternalTransportConnection>, Box<dyn Error + Send + Sync>>>> {
        let any_address: Box<dyn Any> = address;
        let transport_address = any_address.downcast::<T::TransportAddress>().expect("unexpected error in generic address downcast");

        let m = T::dial(self, &transport_address);
        let future = InternalDial::new(m);
        let boxed: Box<dyn Future<Output=Result<Box<dyn InternalTransportConnection>, Box<dyn Error + Send + Sync>>>> = Box::new(future);
        boxed
    }

    fn as_dyn_arc(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static> {
        self
    }
}