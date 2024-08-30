use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;

use super::client::StrikeClient;
use crate::tarnished::Tarnished;

pub struct LocalClient {
    pub db_path: std::path::PathBuf,
}

#[async_trait]
impl StrikeClient for LocalClient {
    async fn add_strike(&self, name: &str) -> Result<u8, String> {
        let db_path = &self.db_path;
        if !db_path.exists() {
            std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        }

        let raw = std::fs::read_to_string(db_path).unwrap_or_else(|_| json!({}).to_string());
        let db: &mut HashMap<String, u8> = &mut serde_json::from_str(&raw).unwrap();
        let count = db.get(name).unwrap_or(&0);
        db.insert(name.to_string(), count + 1);

        std::fs::write(db_path, serde_json::to_string_pretty(&db).unwrap()).unwrap();

        Ok(*db.get(name).unwrap())
    }

    async fn get_tarnished(&self) -> Result<Vec<Tarnished>, String> {
        let db_path = &self.db_path;
        let raw = std::fs::read_to_string(db_path).unwrap_or_else(|_| json!({}).to_string());
        let db: HashMap<String, u8> = serde_json::from_str(&raw).unwrap_or(HashMap::new());

        Ok(Tarnished::sort_desc_by_strike(Tarnished::from_map(db))
            .into_iter()
            .collect())
    }

    fn clear_strikes(&self) {
        let db_path = &self.db_path;
        if db_path.exists() {
            std::fs::write(db_path, json!({}).to_string()).unwrap();
        }
    }

    async fn check_health(&self) -> Result<(), String> {
        println!("Checking health for local client");
        Ok(())
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[tokio::test]
    async fn it_should_add_some_strikes() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
        let client = LocalClient {
            db_path: file.to_path_buf(),
        };

        let _ = client.add_strike("guenther").await?;
        let _ = client.add_strike("guenther").await?;
        let strikes = client.add_strike("guenther").await?;

        assert_eq!(strikes, 3,);

        Ok(())
    }

    #[test]
    fn it_should_return_strikes_in_descending_order() {
        let raw = &mut [
            ("guenther".to_string(), 2),
            ("heinz".to_string(), 1),
            ("hans".to_string(), 3),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<String, u8>>();
        let tarnished = Tarnished::sort_desc_by_strike(Tarnished::from_map(raw.clone()));

        assert_eq!(
            tarnished,
            vec![
                Tarnished {
                    name: "hans".to_string(),
                    strikes: 3
                },
                Tarnished {
                    name: "guenther".to_string(),
                    strikes: 2
                },
                Tarnished {
                    name: "heinz".to_string(),
                    strikes: 1
                }
            ]
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::{
        clients::local_client::{LocalClient, StrikeClient as _},
        tarnished::Tarnished,
    };

    #[tokio::test]
    async fn it_should_add_a_strike() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
        let client = LocalClient {
            db_path: file.to_path_buf(),
        };

        let _ = client.add_strike("guenther").await?;
        let strikes = client.get_tarnished().await.unwrap();

        assert_eq!(
            strikes,
            vec![Tarnished {
                name: "guenther".to_string(),
                strikes: 1
            }]
        );
        Ok(())
    }

    #[tokio::test]
    async fn it_should_add_a_strike_to_an_existing_db() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
        let client = LocalClient {
            db_path: file.to_path_buf(),
        };

        let _ = client.add_strike("guenther").await?;
        let _ = client.add_strike("heinz").await?;
        let _ = client.add_strike("guenther").await?;

        let strikes = client.get_tarnished().await.unwrap();

        assert_eq!(
            strikes,
            vec![
                Tarnished {
                    name: "guenther".to_string(),
                    strikes: 2
                },
                Tarnished {
                    name: "heinz".to_string(),
                    strikes: 1
                }
            ]
        );

        Ok(())
    }

    #[tokio::test]
    async fn it_should_clear_strikes() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
        let client = LocalClient {
            db_path: file.to_path_buf(),
        };

        let _ = client.add_strike("guenther");
        let _ = client.add_strike("heinz");

        client.clear_strikes();

        let strikes = client.get_tarnished().await.unwrap();

        assert!(strikes.is_empty());

        Ok(())
    }
}
