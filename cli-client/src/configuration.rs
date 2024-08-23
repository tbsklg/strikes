use std::path::PathBuf;

use crate::cli::Cli;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub remote: Option<RemoteSettings>,
    pub local: Option<LocalSettings>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RemoteSettings {
    pub api_key: String,
    pub base_url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct LocalSettings {
    pub db_path: std::path::PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
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

pub fn get_configuration(args: &Cli) -> Settings {
    let home = &std::env::var("HOME").unwrap();
    let config_path = args
        .config_path
        .clone()
        .unwrap_or_else(|| PathBuf::from(home).join(".strikes/configuration.yaml"));

    let settings = config::Config::builder().add_source(config::File::new(
        config_path.to_str().unwrap(),
        config::FileFormat::Yaml,
    ));

    match settings.build() {
        Ok(settings) => settings.try_deserialize().map_or_else(
            |_| Settings::default(),
            |settings: Settings| match (&settings.remote, &settings.local) {
                (None, None) => Settings::default(),
                _ => settings,
            },
        ),
        Err(_) => Settings::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_valid_config() {
        let args = Cli {
            config_path: Some(PathBuf::from("tests/fixtures/valid_config.yaml")),
            command: None,
        };
        let configuration = get_configuration(&args);
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
        let args = Cli {
            config_path: Some(PathBuf::from("tests/fixtures/empty_config.yaml")),
            command: None,
        };

        let configuration = get_configuration(&args);

        assert_eq!(
            configuration.local.unwrap().db_path,
            PathBuf::from("/home/user/.strikes/db.json")
        );
    }

    #[test]
    fn parse_invalid_config() {
        let args = Cli {
            config_path: Some(PathBuf::from("tests/fixtures/invalid_config.yaml")),
            command: None,
        };

        let configuration = get_configuration(&args);

        assert_eq!(
            configuration.local.unwrap().db_path,
            PathBuf::from("/home/user/.strikes/db.json")
        )
    }
}
