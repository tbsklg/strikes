use std::path::PathBuf;

use strike::config::Config;

#[test]
fn parse_valid_config() {
    let config = Config::parse(PathBuf::from("tests/fixtures/valid_config"));
    assert_eq!(config.api_key, "abc");
}

#[test]
#[should_panic]
fn parse_invalid_config() {
    Config::parse(PathBuf::from("tests/fixtures/invalid_config"));
}
