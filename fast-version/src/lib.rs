// #!no_std]
// #![feature(repr_simd)]
// #![feature(portable_simd)]

#![cfg_attr(not(feature = "serde"), no_std)]
#![cfg_attr(nightly, feature(repr_simd))]
#![cfg_attr(nightly, feature(portable_simd))]

extern crate alloc;


pub mod version;
pub mod version_req;

pub mod simd;

pub mod serde;

pub use version::Version;
pub use version_req::VersionReq;

