use std::collections::HashMap;

use serde_json::json;

#[derive(Debug, PartialEq)]
pub struct Tarnished {
    pub name: String,
    pub strikes: u8,
}

pub fn add_strike(name: &str, db_path: &std::path::PathBuf) -> HashMap<String, i8> {
    let raw = std::fs::read_to_string(db_path).unwrap_or_else(|_| json!({}).to_string());
    let db = update_strikes(name, &mut serde_json::from_str(&raw).unwrap());

    if !db_path.exists() {
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
    }

    std::fs::write(db_path, serde_json::to_string_pretty(&db).unwrap()).unwrap();

    db
}

pub fn get_tarnished(db_path: &std::path::PathBuf) -> Vec<Tarnished> {
    let raw = std::fs::read_to_string(db_path).unwrap_or_else(|_| json!({}).to_string());
    let db: HashMap<String, u8> = serde_json::from_str(&raw).unwrap_or(HashMap::new());

    sort_desc_by_strike(as_tarnished(db)).into_iter().rev().collect()
}

fn update_strikes(name: &str, db: &mut HashMap<String, i8>) -> HashMap<String, i8> {
    let count = db.get(name).unwrap_or(&0);
    db.insert(name.to_string(), count + 1);

    db.clone()
}

fn sort_desc_by_strike(tarnished: Vec<Tarnished>) -> Vec<Tarnished> {
    let mut tarnished = tarnished;
    tarnished.sort_by(|a, b| b.strikes.cmp(&a.strikes));
    tarnished
}

fn as_tarnished(db: HashMap<String, u8>) -> Vec<Tarnished> {
    db.iter()
        .map(|(name, strikes)| Tarnished {
            name: name.to_string(),
            strikes: *strikes as u8,
        })
        .collect()
}

#[cfg(test)]
mod unit_tests {

    use super::*;

    #[test]
    fn it_adds_a_strike_for_a_new_name() {
        let db = update_strikes("guenther", &mut HashMap::new());
        assert_eq!(db, [("guenther".to_string(), 1)].iter().cloned().collect());
    }

    #[test]
    fn it_adds_a_strike_for_an_existing_name() {
        let db = update_strikes("guenther", &mut HashMap::new());
        assert_eq!(db, [("guenther".to_string(), 1)].iter().cloned().collect());
    }

    #[test]
    fn it_adds_a_strike_for_an_existing_name_with_other_names() {
        let db = update_strikes("guenther", &mut HashMap::from([("hans".to_string(), 2)]));
        assert_eq!(
            db,
            [("guenther".to_string(), 1), ("hans".to_string(), 2)]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn it_should_read_strikes_as_tarnished() {
        let raw = &mut [("guenther".to_string(), 2), ("hans".to_string(), 3)]
            .iter()
            .cloned()
            .collect::<HashMap<String, u8>>();
        let tarnished = sort_desc_by_strike(as_tarnished(raw.clone()));

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
            ]
        );
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn it_adds_a_strike() {
        let db_path = PathBuf::from("tests/fixtures/db.json");

        let db = add_strike("guenther", &db_path);

        std::fs::remove_file(db_path).unwrap();

        assert_eq!(db, [("guenther".to_string(), 1)].iter().cloned().collect());
    }

    #[test]
    fn it_adds_a_strike_to_an_existing_db() {
        let db_path = PathBuf::from("tests/fixtures/db_0.json");
        add_strike("guenther", &db_path);
        add_strike("heinz", &db_path);

        let db = add_strike("guenther", &db_path);

        std::fs::remove_file(db_path).unwrap();

        assert_eq!(
            db,
            [("guenther".to_string(), 2), ("heinz".to_string(), 1)]
                .iter()
                .cloned()
                .collect()
        );
    }
}
