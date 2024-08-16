#[derive(serde::Deserialize)]
pub struct Settings {
    pub api_key: String,
    pub base_url: String,
}

pub fn get_configuration(path: std::path::PathBuf) -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            path.to_str().unwrap(),
            config::FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize::<Settings>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_valid_config() {
        let configuration =
            get_configuration(PathBuf::from("tests/fixtures/valid_config.yaml")).unwrap();
        assert_eq!(configuration.api_key, "abc");
        assert_eq!(configuration.base_url, "https://example.com");
    }

    #[test]
    #[should_panic]
    fn parse_invalid_config() {
        get_configuration(PathBuf::from("tests/fixtures/invalid_config.yaml")).unwrap();
    }
}
