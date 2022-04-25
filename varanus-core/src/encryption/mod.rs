pub mod plaintext;

use std::io::{Error, IoSlice};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite};
use crate::protocol::GenericProtocol;

use pin_project::pin_project;

pub trait EncryptionProtocol<T: AsyncWrite + AsyncRead + 'static>: GenericProtocol {
	type Connection: EncryptionConnection<T>;
}

pub(crate) trait InnerEncryptionProtocol: GenericProtocol {
	fn new_connection(&self, inner: Box<(dyn AsyncReadWrite + 'static)>);
}

pub trait EncryptionConnection<T: AsyncWrite + AsyncRead + 'static>: AsyncWrite + AsyncRead + Sync + Send + Unpin {
	fn new(inner: T) -> Self;
	fn inner_ref(&self) -> Rc<&T>;
}

pub(crate) trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin {}

impl<T: AsyncRead + AsyncWrite + Unpin> AsyncReadWrite for T {}

#[pin_project]
pub(crate) struct InnerEncryptionConnection {
	#[pin]
	inner: Box<(dyn AsyncReadWrite + 'static)>,
}

impl AsyncWrite for InnerEncryptionConnection {
	fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>> {
		let this = self.project();
		this.inner.poll_write(cx, buf)
	}

	fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
		let this = self.project();
		this.inner.poll_flush(cx)
	}

	fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
		let this = self.project();
		this.inner.poll_shutdown(cx)
	}

	fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>]) -> Poll<Result<usize, Error>> {
		let this = self.project();
		this.inner.poll_write_vectored(cx, bufs)
	}

	fn is_write_vectored(&self) -> bool {
		self.inner.is_write_vectored()
	}
}
