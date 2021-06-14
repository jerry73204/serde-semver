//! Serde-compatible version checker.
//!
//! The crate lets you build a version checker using [`declare_version`](declare_version) macro.
//! It is useful to create a versioned configuration with a version number audited during deserialization.
//!
//! For example, declare a `MyVersion` checker with the semver requirement `^3.5.11`.
//!
//! ```rust
//! serde_semver::declare_version!(MyVersion, 3, 5, 11);
//! ```
//!
//! We can embed it in the configuration struct. In the following code, it audits the version number
//! in the JSON text.
//!
//! ```rust
//! use semver::Version;
//! use serde::{Deserialize, Serialize};
//! use std::path::PathBuf;
//!
//! // Declare the checker type that asserts '3.5.11'
//! serde_semver::declare_version!(MyVersion, 3, 5, 11);
//!
//! // An example configuration with version tag
//! #[derive(Serialize, Deserialize)]
//! struct Config {
//!     pub version: MyVersion,
//!     pub input_file: PathBuf,
//!     pub output_file: PathBuf,
//! }
//!
//! // The version number is audited during deserialization.
//! let config: Config = serde_json::from_str(
//!     r#"{
//!   "version": "3.5.12",
//!   "input_file": "input.txt",
//!   "output_file": "output.txt"
//! }"#,
//! )
//! .unwrap();
//!
//! // The original version is recovered after serialization.
//! assert_eq!(
//!     serde_json::to_string_pretty(&config).unwrap(),
//!     r#"{
//!   "version": "3.5.12",
//!   "input_file": "input.txt",
//!   "output_file": "output.txt"
//! }"#,
//! );
//!
//! // Besides deserialization, the version tag can also be created from scratch.
//! let my_ver = MyVersion::new(Version::new(3, 5, 11)).unwrap();
//! ```

pub use once_cell;
pub use semver;
pub use serde;

/// Declare a semver checker type with minimum version requirement.
///
/// The macro has the signature `declare_version(type_name, major, minor, patch)`.
/// For example,
///
/// ```rust
/// serde_semver::declare_version!(MyVersion, 3, 5, 11);
/// ```
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
