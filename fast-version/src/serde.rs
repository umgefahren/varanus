#[cfg(feature = "serde")]
use serde::{Serialize, de::DeserializeOwned};

#[cfg(feature = "serde")]
use crate::{Version, VersionReq, version_req::{VersionComperatorUpper, VersionComperatorLower}};


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

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VersionSerde {
    pub major: u64,
    pub minor: u64,
    pub patch: u64
}

#[cfg(feature = "serde")]
impl From<Version<u64>> for VersionSerde {
    fn from(v: Version<u64>) -> Self {
        Self {
            major: v.major,
            minor: v.minor,
            patch: v.patch
        }
    }
}

#[cfg(feature = "serde")]
impl From<VersionSerde> for Version<u64> {
    fn from(v: VersionSerde) -> Self {
        Self {
            major: v.major,
            minor: v.minor,
            patch: v.patch
        }
    }
}

#[cfg(feature = "serde")]
impl From<Version<u32>> for VersionSerde {
    fn from(v: Version<u32>) -> Self {
        let major = v.major as u64;
        let minor = v.minor as u64;
        let patch = v.patch as u64;

        Self {
            major,
            minor,
            patch
        }
    }
}

#[cfg(feature = "serde")]
pub enum VersionSerdeError {
    NumberToBig
}

#[cfg(feature = "serde")]
fn convert_to_u32(input: u64) -> Result<u32, VersionSerdeError> {
    const U32_MAX: u64 = {
        u32::MAX as u64
    };

    if input > U32_MAX {
        return Err(VersionSerdeError::NumberToBig);
    }
    Ok(input as u32)
}

#[cfg(feature = "serde")]
impl TryFrom<VersionSerde> for Version<u32> {
    type Error = VersionSerdeError;
    fn try_from(value: VersionSerde) -> Result<Self, Self::Error> {
        let major = convert_to_u32(value.major)?;
        let minor = convert_to_u32(value.minor)?;
        let patch = convert_to_u32(value.patch)?;

        Ok(Self { major, minor, patch })
    }
}

#[cfg(feature = "serde")]
impl From<Version<usize>> for VersionSerde {
    fn from(v: Version<usize>) -> Self {
       let major = v.major as u64; 
       let minor = v.minor as u64;
       let patch = v.patch as u64;

       Self { major, minor, patch }
    }
}

#[cfg(feature = "serde")]
fn convert_to_usize(input: u64) -> Result<usize, VersionSerdeError> {
    const USIZE_MAX: u64 = {
        usize::MAX as u64 
    };

    if input > USIZE_MAX {
        return Err(VersionSerdeError::NumberToBig);
    }

    Ok(input as usize)
}

#[cfg(feature = "serde")]
impl TryFrom<VersionSerde> for Version<usize> {
    type Error = VersionSerdeError;

    fn try_from(value: VersionSerde) -> Result<Self, Self::Error> {
        let major = convert_to_usize(value.major)?;
        let minor = convert_to_usize(value.minor)?;
        let patch = convert_to_usize(value.patch)?;

        Ok(Self { major, minor, patch })
    }
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct VersionReqSerde {
    comperator_lower: VersionComperatorLower,
    comperator_higher: VersionComperatorUpper,
    major_lower: u64,
    minor_lower: u64,
    patch_lower: u64,
    major_upper: u64,
    minor_upper: u64,
    patch_upper: u64
}

#[cfg(feature = "serde")]
impl From<VersionReq<u64>> for VersionReqSerde {
   fn from(v: VersionReq<u64>) -> Self {
       Self {
           comperator_lower: v.comperator_lower,
           comperator_higher: v.comperator_higher,
           major_lower: v.major_lower,
           minor_lower: v.minor_lower,
           patch_lower: v.patch_lower,
           major_upper: v.major_upper,
           minor_upper: v.minor_upper,
           patch_upper: v.patch_upper
       }
   }
}

#[cfg(feature = "serde")]
impl From<VersionReqSerde> for VersionReq<u64> {
    fn from(v: VersionReqSerde) -> Self {
        Self {
            comperator_lower: v.comperator_lower,
            comperator_higher: v.comperator_higher,
            major_lower: v.major_lower,
            minor_lower: v.minor_lower,
            patch_lower: v.patch_lower,
            major_upper: v.major_upper,
            minor_upper: v.minor_upper,
            patch_upper: v.patch_upper
        }
    }
}
