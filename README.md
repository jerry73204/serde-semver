# serde-semver: Serde-compatible version checker

The crate lets you build a version checker using `declare_version` macro.
It is useful to create a versioned configuration with a version number audited during deserialization.

## Usage

For example, declare a `MyVersion` checker with the semver requirement `^3.5.11`.

```rust
serde_semver::declare_version!(MyVersion, 3, 5, 11);
```

We can embed it in the configuration struct. In the following code, it audits the version number
in the JSON text.

```rust
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Declare the checker type that asserts '3.5.11'
serde_semver::declare_version!(MyVersion, 3, 5, 11);

// An example configuration with version tag
#[derive(Serialize, Deserialize)]
struct Config {
    pub version: MyVersion,
    pub input_file: PathBuf,
    pub output_file: PathBuf,
}

// The version number is audited during deserialization.
let config: Config = serde_json::from_str(
    r#"{
  "version": "3.5.12",
  "input_file": "input.txt",
  "output_file": "output.txt"
}"#,
)
.unwrap();

// The original version is recovered after serialization.
assert_eq!(
    serde_json::to_string_pretty(&config).unwrap(),
    r#"{
  "version": "3.5.12",
  "input_file": "input.txt",
  "output_file": "output.txt"
}"#,
);

// Besides deserialization, the version tag can also be created from scratch.
let my_ver = MyVersion::new(Version::new(3, 5, 11)).unwrap();
```

## License

MIT license. See [LICENSE.txt](LICENSE.txt) file.
