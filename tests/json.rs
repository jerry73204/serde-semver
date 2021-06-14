use semver::Version;
use serde_semver::declare_version;

declare_version!(MyVersion, 3, 1, 4);

#[test]
fn const_version() {
    assert_eq!(MyVersion::MIN_VERSION, semver::Version::new(3, 1, 4));
}

#[test]
fn create_version() {
    assert!(MyVersion::new(Version::new(3, 1, 5)).is_some());
    assert!(MyVersion::new(Version::new(3, 1, 4)).is_some());
    assert!(MyVersion::new(Version::new(3, 1, 2)).is_none());
}

#[test]
fn serialize_test() {
    let version = MyVersion::new(Version::new(3, 1, 5)).unwrap();
    let text = serde_json::to_string(&version).unwrap();
    assert_eq!(text, r#""3.1.5""#);
}

#[test]
fn deserialize_test() {
    serde_json::from_str::<MyVersion>(r#""3.1.5""#).unwrap();
    serde_json::from_str::<MyVersion>(r#""3.1.4""#).unwrap();
    assert!(matches!(
        serde_json::from_str::<MyVersion>(r#""2.1.7""#),
        Err(_)
    ));
}
