use fast_version::serde::{VersionSerde, VersionReqSerde, VersionSerdeError};
use fast_version::{Version, VersionReq};
use thiserror::Error;

use super::DefaultVersionNumber;

use super::name::{ProtocolName, ProtocolNameError};

#[derive(Debug, Hash)]
pub struct ProtocolIdentifier {
    pub name: ProtocolName,
    pub version: Version<DefaultVersionNumber>,
    pub version_req: VersionReq<DefaultVersionNumber>,
}

impl ProtocolIdentifier {
    pub fn new(name: ProtocolName, version: Version<DefaultVersionNumber>, version_req: VersionReq<DefaultVersionNumber>) -> Self {
        Self {
            name,
            version,
            version_req
        }
    }
}

#[derive(Error, Debug)]
pub enum ProtocolIdentifierSerdeError {
    #[error("Error in version conversion")]
    VersionSerde(#[from] VersionSerdeError),
    #[error("Error in protocl name conversion")]
    ProtocolNameError(#[from] ProtocolNameError),
    #[error("Error in version requirenment conversion")]
    VersionReqSerde,
}

pub struct ProtocolIdentifierSerde {
    name: String,
    version: VersionSerde,
    version_req: VersionReqSerde
}

impl From<ProtocolIdentifier> for ProtocolIdentifierSerde {
    fn from(pi: ProtocolIdentifier) -> Self {
        let name = pi.name.to_string();
        let version = VersionSerde::from(pi.version);
        let version_req = VersionReqSerde::from(pi.version_req);
        Self {
            name,
            version,
            version_req
        }
    }
}

impl TryFrom<ProtocolIdentifierSerde> for ProtocolIdentifier {
    type Error = ProtocolIdentifierSerdeError;

    fn try_from(value: ProtocolIdentifierSerde) -> Result<Self, Self::Error> {
       let name = ProtocolName::new(value.name).map_err(|e| ProtocolIdentifierSerdeError::ProtocolNameError(e))?;
       let version = Version::<DefaultVersionNumber>::try_from(value.version).map_err(ProtocolIdentifierSerdeError::VersionSerde)?;
       let version_req = VersionReq::try_from(value.version_req).map_err(|_| ProtocolIdentifierSerdeError::VersionReqSerde)?;

       let ret = Self {
           name,
           version,
           version_req
       };

       Ok(ret)
    }
}
