# serde-semver: Serde-compatible version checker

[API doc](https://docs.rs/serde-semver/)

This crate provides a derive macro `SemverReq` to build a _versioned_ marker type.
For example, it assocates the type with version "3.1.4".

```rust
#[derive(SemverReq)]
#[version("3.1.4")]
struct MyVersion;
```
The marker type works as a version checker during deserialization. In this example, the marker verifies whether the deserialized JSON text is is compatible with version "3.1.4". For example, "3.1.3" and "3.1.0" are valid versions, but "3.2.0" and "2.7.0" are not.


```rust
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use serde_semver::SemverReq;

#[derive(SemverReq)]
#[version("3.1.4")]
struct MyVersion;

// An example configuration with version tag
#[derive(Serialize, Deserialize)]
struct Config {
    pub version: MyVersion,
    pub input_file: PathBuf,
    pub output_file: PathBuf,
}

// The version is audited during deserialization.
let config: Config = serde_json::from_str(
    r#"{
  "version": "3.1.4",
  "input_file": "input.txt",
  "output_file": "output.txt"
}"#,
)
.unwrap();

// The version is recovered after serialization.
assert_eq!(
    serde_json::to_string_pretty(&config).unwrap(),
    r#"{
  "version": "3.1.4",
  "input_file": "input.txt",
  "output_file": "output.txt"
}"#,
);
```

## License

MIT license. See [LICENSE.txt](LICENSE.txt) file.
