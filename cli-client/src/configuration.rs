use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub use_remote: bool,
    pub remote: Option<RemoteSettings>,
    pub local: Option<LocalSettings>,
}

#[derive(serde::Deserialize)]
pub struct RemoteSettings {
    pub api_key: String,
    pub base_url: String,
}

#[derive(serde::Deserialize)]
pub struct LocalSettings {
    pub db_path: std::path::PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            use_remote: false,
            remote: None,
            local: {
                Some(LocalSettings {
                    db_path: std::env::var("HOME")
                        .map(|home| PathBuf::from(home).join(".strikes/db.json"))
                        .unwrap(),
                })
            },
        }
    }
}

pub fn get_configuration(path: std::path::PathBuf) -> Settings {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            path.to_str().unwrap(),
            config::FileFormat::Yaml,
        ))
        .build();

    match settings {
        Ok(settings) => settings.try_deserialize::<Settings>().unwrap_or_default(),
        Err(_) => Settings::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_valid_config() {
        let configuration =
            get_configuration(PathBuf::from("tests/fixtures/valid_config.yaml"));
        assert_eq!(configuration.use_remote, true);
        assert_eq!(configuration.remote.as_ref().unwrap().api_key, "abc");
        assert_eq!(
            configuration.remote.as_ref().unwrap().base_url,
            "https://example.com"
        );
        assert_eq!(
            configuration.local.unwrap().db_path,
            PathBuf::from("/home/user/.strikes")
        );
    }

    #[test]
    fn parse_default_config() {
        std::env::set_var("HOME", "/home/user");

        let configuration = get_configuration(PathBuf::from("tests/fixtures/empty_config.yaml"));

        assert_eq!(
            configuration.local.unwrap().db_path,
            PathBuf::from("/home/user/.strikes/db.json")
        );
    }

    #[test]
    fn parse_invalid_config() {
        let configuration = get_configuration(PathBuf::from("tests/fixtures/invalid_config.yaml"));

        assert_eq!(configuration.use_remote, false);
    }
}
