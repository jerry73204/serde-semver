use serde_semver::SemverReq;
// use semver::Version;
// use serde_semver::declare_version;

// declare_version!(MyVersion, 3, 1, 4);

#[derive(SemverReq)]
#[version("3.1.4")]
struct MyVersion;

#[test]
fn const_version() {
    assert_eq!(MyVersion::version(), semver::Version::new(3, 1, 4));
}

#[test]
fn serialize_test() {
    let text = serde_json::to_string(&MyVersion).unwrap();
    assert_eq!(text, r#""3.1.4""#);
}

#[test]
fn deserialize_test() {
    assert!(serde_json::from_str::<MyVersion>(r#""3.1.5""#).is_err());
    assert!(serde_json::from_str::<MyVersion>(r#""3.1.4""#).is_ok());
    assert!(serde_json::from_str::<MyVersion>(r#""3.1.3""#).is_ok());
    assert!(serde_json::from_str::<MyVersion>(r#""2.1.7""#).is_err(),);
}
