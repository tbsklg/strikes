use std::path::PathBuf;

use strikes::config::Config;

#[test]
fn parse_valid_config() {
    let config = Config::parse(PathBuf::from("tests/fixtures/valid_config"));
    assert_eq!(config.api_key, "abc");
    assert_eq!(config.base_url, "https://example.com");
}

#[test]
#[should_panic]
fn parse_invalid_config() {
    Config::parse(PathBuf::from("tests/fixtures/invalid_config"));
}
