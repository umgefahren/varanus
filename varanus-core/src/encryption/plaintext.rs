use std::io::{Error, IoSlice};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use crate::encryption::EncryptionConnection;
use pin_project::pin_project;

#[pin_project]
pub struct PlainTextConnection<T: AsyncRead + AsyncWrite + 'static> {
	#[pin]
	inner: T,
}

impl<T: AsyncRead + AsyncWrite + 'static> PlainTextConnection<T> {
	#[inline]
	fn get_pin<'a>(self: Pin<&mut Self>) -> Pin<&mut T> {
		let this = self.project();
		this.inner
	}
}

impl<T: AsyncRead + AsyncWrite + 'static> AsyncWrite for PlainTextConnection<T> {
	fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize, Error>> {
		let pinned = self.get_pin();
		pinned.poll_write(cx, buf)
	}

	fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
		let pinned = self.get_pin();
		pinned.poll_flush(cx)
	}

	fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
		let pinned = self.get_pin();
		pinned.poll_shutdown(cx)
	}

	fn poll_write_vectored(self: Pin<&mut Self>, cx: &mut Context<'_>, bufs: &[IoSlice<'_>]) -> Poll<Result<usize, Error>> {
		let pinned = self.get_pin();
		pinned.poll_write_vectored(cx, bufs)
	}

	fn is_write_vectored(&self) -> bool {
		self.inner.is_write_vectored()
	}
}

impl<T: AsyncRead + AsyncWrite + 'static> AsyncRead for PlainTextConnection<T> {
	fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
		let pinned = self.get_pin();
		pinned.poll_read(cx, buf)
	}
}

impl<T: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static> EncryptionConnection<T> for PlainTextConnection<T> {
	fn new(inner: T) -> Self {
		Self {
			inner
		}
	}

	fn inner_ref(&self) -> Rc<&T> {
		let ret = Rc::new(&self.inner);
		ret
	}
}