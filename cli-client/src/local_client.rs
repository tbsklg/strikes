use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Tarnished {
    pub name: String,
    pub strikes: u8,
}

impl Tarnished {
    fn sort_desc_by_strike(tarnished: Vec<Tarnished>) -> Vec<Tarnished> {
        let mut tarnished = tarnished.clone();
        tarnished.sort_by(|a, b| b.strikes.partial_cmp(&a.strikes).unwrap());
        tarnished
    }

    fn as_tarnished(db: HashMap<String, u8>) -> Vec<Tarnished> {
        db.iter()
            .map(|(name, strikes)| Tarnished {
                name: name.to_string(),
                strikes: *strikes,
            })
            .collect()
    }
}

pub trait StrikeClient {
    fn add_strike(&self, name: &str) -> HashMap<String, i8>;
    fn get_tarnished(&self) -> Vec<Tarnished>;
    fn clear_strikes(&self);
}

pub struct LocalClient {
    pub db_path: std::path::PathBuf,
}

impl StrikeClient for LocalClient {
    fn add_strike(&self, name: &str) -> HashMap<String, i8> {
        let db_path = &self.db_path;
        if !db_path.exists() {
            std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
        }

        let raw = std::fs::read_to_string(db_path).unwrap_or_else(|_| json!({}).to_string());
        let db: &mut HashMap<String, i8> = &mut serde_json::from_str(&raw).unwrap();
        let count = db.get(name).unwrap_or(&0);
        db.insert(name.to_string(), count + 1);

        std::fs::write(db_path, serde_json::to_string_pretty(&db).unwrap()).unwrap();

        db.clone()
    }

    fn get_tarnished(&self) -> Vec<Tarnished> {
        let db_path = &self.db_path;
        let raw = std::fs::read_to_string(db_path).unwrap_or_else(|_| json!({}).to_string());
        let db: HashMap<String, u8> = serde_json::from_str(&raw).unwrap_or(HashMap::new());

        Tarnished::sort_desc_by_strike(Tarnished::as_tarnished(db))
            .into_iter()
            .collect()
    }

    fn clear_strikes(&self) {
        let db_path = &self.db_path;
        if db_path.exists() {
            std::fs::write(db_path, json!({}).to_string()).unwrap();
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn it_should_add_a_strike_for_an_existing_name() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
        let client = LocalClient {
            db_path: file.to_path_buf(),
        };

        client.add_strike("guenther");
        let db = client.add_strike("heinz");

        assert_eq!(
            db,
            [("guenther".to_string(), 1), ("heinz".to_string(), 1)]
                .iter()
                .cloned()
                .collect()
        );

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
        let tarnished = Tarnished::sort_desc_by_strike(Tarnished::as_tarnished(raw.clone()));

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
    use crate::local_client::{LocalClient, StrikeClient};

    #[test]
    fn it_should_add_a_strike() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
        let client = LocalClient {
            db_path: file.to_path_buf(),
        };

        let db = client.add_strike("guenther");
        assert_eq!(db, [("guenther".to_string(), 1)].iter().cloned().collect());

        Ok(())
    }

    #[test]
    fn it_should_add_a_strike_to_an_existing_db() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
        let client = LocalClient {
            db_path: file.to_path_buf(),
        };
        client.add_strike("guenther");
        client.add_strike("heinz");

        let db = client.add_strike("guenther");

        assert_eq!(
            db,
            [("guenther".to_string(), 2), ("heinz".to_string(), 1)]
                .iter()
                .cloned()
                .collect()
        );

        Ok(())
    }

    #[test]
    fn it_should_clear_strikes() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("./tests/fixtures/db.json")?;
        let client = LocalClient {
            db_path: file.to_path_buf(),
        };
        client.add_strike("guenther");
        client.add_strike("heinz");

        client.clear_strikes();

        assert!(client.get_tarnished().is_empty());

        Ok(())
    }
}
