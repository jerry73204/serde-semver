//! Serde-compatible version checker.
//!
//! The crate lets you build a version checker using [`declare_version`](declare_version) macro.
//! It is useful to create a versioned configuration with a version number audited during deserialization.
//!
//! For example, declare a `MyVersion` checker with the semver requirement `^3.1.4`.
//!
//! ```rust
//! # use serde_semver::SemverReq;
//! #[derive(SemverReq)]
//! #[version("3.1.4")]
//! struct MyVersion;
//! ```
//!
//! We can embed it in the configuration struct. In the following code, it audits the version number
//! in the JSON text.
//!
//! ```rust
//! use semver::Version;
//! use serde::{Deserialize, Serialize};
//! use serde_semver::SemverReq;
//! use std::path::PathBuf;
//!
//! // Declare the checker type that asserts '3.1.4'
//! #[derive(SemverReq)]
//! #[version("3.1.4")]
//! struct MyVersion;
//!
//! // An example configuration with version tag
//! #[derive(Serialize, Deserialize)]
//! struct Config {
//!     pub version: MyVersion,
//!     pub input_file: PathBuf,
//!     pub output_file: PathBuf,
//! }
//!
//! // The version is audited during deserialization.
//! let config: Config = serde_json::from_str(
//!     r#"{
//!   "version": "3.1.4",
//!   "input_file": "input.txt",
//!   "output_file": "output.txt"
//! }"#,
//! )
//! .unwrap();
//!
//! // The version is recovered after serialization.
//! assert_eq!(
//!     serde_json::to_string_pretty(&config).unwrap(),
//!     r#"{
//!   "version": "3.1.4",
//!   "input_file": "input.txt",
//!   "output_file": "output.txt"
//! }"#,
//! );
//!
//! // It accepts compatible version during deserialization, such as "3.1.3".
//! let config: Config = serde_json::from_str(
//!     r#"{
//!   "version": "3.1.3",
//!   "input_file": "input.txt",
//!   "output_file": "output.txt"
//! }"#,
//! )
//! .unwrap();
//!
//! // The version is updated after serialization.
//! assert_eq!(
//!     serde_json::to_string_pretty(&config).unwrap(),
//!     r#"{
//!   "version": "3.1.4",
//!   "input_file": "input.txt",
//!   "output_file": "output.txt"
//! }"#,
//! );
//! ```

pub use semver;
pub use serde;
pub use serde_semver_derive::SemverReq;
