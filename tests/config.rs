use raex::RaExConfig;

#[test]
fn check_file() {
    let config = RaExConfig::new("tests/raex").unwrap();

    assert_eq!(config.global_addr, "0.0.0.0".to_string());
    assert_eq!(config.local_addr, "1.1.1.1".to_string());
}
