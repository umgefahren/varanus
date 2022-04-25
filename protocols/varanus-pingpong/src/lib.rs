use std::error::Error;
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};
use fast_version::version_req::{VersionRegCompType, VersionRegType};
use varanus_core::node::NodeState;
use varanus_core::protocol::{DefaultVersionNumber, GenericProtocol, Version, VersionReq};
use varanus_core::protocol::name::ProtocolName;
use varanus_core::protocol::request::{FastRequestProtocol, RequestProtocol, RequestType, ResponseType};
use varanus_core::transport::TransportProtocol;

lazy_static::lazy_static! {
	static ref PING_PONG_VERSION: Version<DefaultVersionNumber> = {
		Version::new(1, 1, 1).unwrap()
	};
	static ref PING_PONG_VERSION_REQ: VersionReq<DefaultVersionNumber> = {
		let init_version = Version::new(1, 1, 1).unwrap();
		let type_version_req = VersionRegType::Strict(init_version);
		VersionReq::try_from(VersionRegCompType::Pure(type_version_req)).unwrap()
	};
	static ref PING_PONG_PROTOCOL_NAME: ProtocolName = {
		ProtocolName::new("PingPong".to_string()).unwrap()
	};
}

#[derive(Serialize, Deserialize)]
pub enum PingPongRequest {
	Roundtrip(Box<PingPongRoundtripRequest>),
	Counter,
}

impl RequestType<'_> for PingPongRequest {}

#[derive(Serialize, Deserialize)]
pub struct PingPongRoundtripRequest {
	#[serde(with = "serde_millis")]
	instant: Instant,
}

#[derive(Serialize, Deserialize)]
pub enum PingPongResponse {
	Roundtrip(Box<PingPongRoundtripResponse>),
	Counter(usize)
}

impl ResponseType<'_> for PingPongResponse {}

#[derive(Serialize, Deserialize)]
pub struct PingPongRoundtripResponse {
	#[serde(with = "serde_millis")]
	request_instant: Instant,
	#[serde(with = "serde_millis")]
	instant: Instant,
}

impl PingPongRoundtripResponse {
	pub fn calculate_duration(&self) -> Duration {
		self.instant.duration_since(self.request_instant)
	}
}

#[derive(Debug)]
pub enum PingPongError {}


impl Display for PingPongError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("PingPongError")
	}
}

impl Error for PingPongError {}

pub struct PingPongRequestHandleFuture {
	this: Arc<PingPongProtocol>,
	request: PingPongRequest,
}

impl Future for PingPongRequestHandleFuture {
	type Output = Result<PingPongResponse, PingPongError>;

	fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
		let res = match &self.request {
			PingPongRequest::Roundtrip(r) => {
				let request_instant = r.instant;
				let response_instant = Instant::now();
				let inner_response = PingPongRoundtripResponse {
					request_instant,
					instant: response_instant
				};
				let response = PingPongResponse::Roundtrip(Box::new(inner_response));
				self.this.counter.fetch_add(1, Ordering::Relaxed);
				Ok(response)
			}
			PingPongRequest::Counter => {
				let count = self.this.counter.fetch_add(1, Ordering::Acquire);
				let response = PingPongResponse::Counter(count);
				Ok(response)
			}
		};
		Poll::Ready(res)
	}
}

pub struct PingPongProtocol {
	counter: AtomicUsize,
}

impl PingPongProtocol {
	pub fn new() -> Self {
		PingPongProtocol {
			counter: AtomicUsize::new(0)
		}
	}
}

impl GenericProtocol for PingPongProtocol {
	fn version() -> Version<varanus_core::protocol::DefaultVersionNumber> {
		*PING_PONG_VERSION
	}

	fn version_req() -> VersionReq<varanus_core::protocol::DefaultVersionNumber> {
		*PING_PONG_VERSION_REQ
	}

	fn name() -> ProtocolName {
		PING_PONG_PROTOCOL_NAME.clone()
	}
}

async fn process_request<Dt: TransportProtocol>(this: Arc<PingPongProtocol>, _: Arc<NodeState<Dt>>, request: PingPongRequest) -> Result<PingPongResponse, PingPongError> {
	match request {
		PingPongRequest::Roundtrip(r) => {
			let request_instant = r.instant;
			let response_instant = Instant::now();
			let inner_response = PingPongRoundtripResponse {
				request_instant,
				instant: response_instant
			};
			let response = PingPongResponse::Roundtrip(Box::new(inner_response));
			this.counter.fetch_add(1, Ordering::Relaxed);
			Ok(response)
		}
		PingPongRequest::Counter => {
			let count = this.counter.fetch_add(1, Ordering::Acquire);
			let response = PingPongResponse::Counter(count);
			Ok(response)
		}
	}
}


impl<'a, Dt: TransportProtocol> RequestProtocol<'a, Dt> for PingPongProtocol {
	type Request = PingPongRequest;
	type Response = PingPongResponse;
	type Error = PingPongError;

	fn handle_request(self: Arc<Self>, state: Arc<NodeState<Dt>>, request: Self::Request) -> Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>> {
		Box::pin(process_request(self, state, request))
	}
}

impl<'a, Dt: TransportProtocol> FastRequestProtocol<'a, Dt> for PingPongProtocol {
	type Request = PingPongRequest;
	type Response = PingPongResponse;
	type Error = PingPongError;
	type RequestHandleFuture = PingPongRequestHandleFuture;

	fn handle_request_fast(self: Arc<Self>, _: Arc<NodeState<Dt>>, request: Self::Request) -> Self::RequestHandleFuture {
		let this = self;
		PingPongRequestHandleFuture {
			this,
			request
		}
	}
}

impl PingPongProtocol {
	pub fn roundtrip_request() -> PingPongRequest {
		let instant = Instant::now();
		let inner_request = PingPongRoundtripRequest {
			instant
		};
		let boxed_request = Box::new(inner_request);
		PingPongRequest::Roundtrip(boxed_request)
	}

	pub fn counter_request() -> PingPongRequest {
		PingPongRequest::Counter
	}
}