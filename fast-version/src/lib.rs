#![no_std]
// #![feature(repr_simd)]
// #![feature(portable_simd)]


#![cfg_attr(nightly, feature(repr_simd))]
#![cfg_attr(nightly, feature(portable_simd))]


pub mod version;
pub mod version_req;

pub mod simd;

pub use version::Version;

