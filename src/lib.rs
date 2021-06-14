//! Checker

pub use once_cell;
pub use semver;
pub use serde;

#[macro_export]
macro_rules! declare_version {
    ($name:ident, $major:expr, $minor:expr, $patch:expr) => {
        #[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $name(::serde_semver::semver::Version);

        impl $name {
            pub const MIN_VERSION: ::serde_semver::semver::Version =
                ::serde_semver::semver::Version::new($major, $minor, $patch);

            pub fn version_req() -> &'static ::serde_semver::semver::VersionReq {
                use ::serde_semver::{once_cell::sync::OnceCell, semver::VersionReq};

                static ONCE: OnceCell<VersionReq> = OnceCell::new();

                ONCE.get_or_init(|| VersionReq::parse(&Self::MIN_VERSION.to_string()).unwrap())
            }

            pub fn new(version: ::serde_semver::semver::Version) -> Option<Self> {
                let req = Self::version_req();
                req.matches(&version).then(|| Self(version))
            }
        }

        impl ::serde_semver::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde_semver::serde::Serializer,
            {
                self.0.serialize(serializer)
            }
        }

        impl<'de> ::serde_semver::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde_semver::serde::Deserializer<'de>,
            {
                use ::serde_semver::{
                    semver::{Version, VersionReq},
                    serde::de::Error,
                };

                let version = Version::deserialize(deserializer)?;
                let req = Self::version_req();
                if !req.matches(&version) {
                    return Err(D::Error::custom(format!(
                        "the version '{}' does not satisfy the requirement '{}'",
                        version, req
                    )));
                }
                Ok(Self(version))
            }
        }
    };
}
