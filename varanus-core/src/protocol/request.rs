use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use crate::protocol::GenericProtocol;
use serde::{Serialize, Deserialize};
use crate::node::NodeState;
use crate::transport::TransportProtocol;

pub trait RequestType<'a>: Serialize + Deserialize<'a> {
}

pub trait ResponseType<'a>: Serialize + Deserialize<'a> {
}

pub trait RequestProtocol<'a, Dt: TransportProtocol>: GenericProtocol {
	type Request: RequestType<'a>;
	type Response: ResponseType<'a>;
	type Error: Error + Send + Sync;

	fn handle_request(self: Arc<Self>, state: Arc<NodeState<Dt>>, request: Self::Request) -> Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
}

pub trait FastRequestProtocol<'a, Dt: TransportProtocol>: GenericProtocol {
	type Request: RequestType<'a>;
	type Response: ResponseType<'a>;
	type Error: Error + Send + Sync;
	type RequestHandleFuture: Future<Output = Result<Self::Response, Self::Error>>;

	fn handle_request_fast(self: Arc<Self>, state: Arc<NodeState<Dt>>, request: Self::Request) -> Self::RequestHandleFuture;
}