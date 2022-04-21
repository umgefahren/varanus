#[cfg(feature = "serde")]
use serde::{Serialize, de::DeserializeOwned};

#[cfg(feature = "serde")]
pub trait SerdeNumber: Serialize + DeserializeOwned {}

#[cfg(not(feature = "serde"))]
pub trait SerdeNumber {}

impl SerdeNumber for u8 {}

impl SerdeNumber for u16 {}

impl SerdeNumber for u32 {}

impl SerdeNumber for u64 {}

impl SerdeNumber for usize {}

impl SerdeNumber for i8 {}

impl SerdeNumber for i16 {}

impl SerdeNumber for i32 {}

impl SerdeNumber for i64 {}

impl SerdeNumber for isize {}
