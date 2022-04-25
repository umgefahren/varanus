use std::any::Any;
pub use fast_version::{Version, VersionReq};

use self::{name::ProtocolName, identifier::ProtocolIdentifier};


pub mod name;
pub mod identifier;
pub mod request;

pub type DefaultVersionNumber = u64;

pub trait GenericProtocol: Any + Send + Sync {
    fn version() -> Version<DefaultVersionNumber>;
    fn version_req() -> VersionReq<DefaultVersionNumber>;
    fn name() -> ProtocolName;
    fn version_identifier() -> ProtocolIdentifier {
        let version = Self::version();
        let version_req = Self::version_req();
        let name = Self::name();
        ProtocolIdentifier::new(name, version, version_req)
    }
}

pub(crate) trait InternalGenericProtocol: Send + Sync {
    fn version(&self) -> Version<DefaultVersionNumber>;
    fn version_req(&self) -> VersionReq<DefaultVersionNumber>;
    fn name(&self) -> ProtocolName;
    fn version_identifier(&self) -> ProtocolIdentifier;
}

impl<T: GenericProtocol> InternalGenericProtocol for T {
    fn version(&self) -> Version<DefaultVersionNumber> {
        T::version()
    }

    fn version_req(&self) -> VersionReq<DefaultVersionNumber> {
        T::version_req()
    }

    fn name(&self) -> ProtocolName {
        T::name()
    }

    fn version_identifier(&self) -> ProtocolIdentifier {
        T::version_identifier()
    }
}
