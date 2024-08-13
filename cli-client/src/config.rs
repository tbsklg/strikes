use std::collections::HashMap;

pub struct Config {
    pub api_key: String,
    pub base_url: String,
}

impl Config {
    pub fn parse(config_path: std::path::PathBuf) -> Config {
        let config_content =
            std::fs::read_to_string(config_path).expect("could not read config file");

        let env = config_content.split('\n').filter(|x| !x.is_empty()).fold(
            HashMap::new(),
            |mut acc, curr| {
                let pair = curr.split("=").collect::<Vec<_>>();
                acc.insert(pair[0].to_lowercase(), pair[1]);
                acc
            },
        );

        let api_key = env.get("api_key").unwrap_or_else(|| {
            panic!("api_key not found in config file");
        });

        let url = env.get("base_url").unwrap_or_else(|| {
            panic!("url not found in config file");
        });

        Config {
            api_key: api_key.to_string(),
            base_url: url.to_string(),
        }
    }
}
