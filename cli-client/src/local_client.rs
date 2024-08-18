use serde_json::{json, Value};

pub fn add_strike(name: &str, db_path: &std::path::PathBuf) -> Value {
    let db = std::fs::read_to_string(db_path).unwrap_or_else(|_| json!({}).to_string());
    let updated_db = update_strikes(name, serde_json::from_str(&db).unwrap());

    if !db_path.exists() {
        std::fs::create_dir_all(db_path.parent().unwrap()).unwrap();
    }

    std::fs::write(db_path, serde_json::to_string_pretty(&updated_db).unwrap()).unwrap();

    updated_db
}

fn update_strikes(name: &str, db: Value) -> Value {
    let mut db = db.as_object().unwrap().clone();
    let count = db.get(name).unwrap_or(&Value::Null).as_u64().unwrap_or(0);
    db.insert(name.to_string(), Value::from(count + 1));

    Value::Object(db)
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_adds_a_strike_for_a_new_name() {
        let db = update_strikes("guenther", json!({}));
        assert_eq!(db, json!({"guenther": 1}));
    }

    #[test]
    fn it_adds_a_strike_for_an_existing_name() {
        let db = update_strikes("guenther", json!({"guenther": 1}));
        assert_eq!(db, json!({"guenther": 2}));
    }

    #[test]
    fn it_adds_a_strike_for_an_existing_name_with_other_names() {
        let db = update_strikes("guenther", json!({"guenther": 1, "hans": 2}));
        assert_eq!(db, json!({"guenther": 2, "hans": 2}));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use serde_json::json;
    use std::path::PathBuf;

    #[test]
    fn it_adds_a_strike() {
        let db_path = PathBuf::from("tests/fixtures/db.json");

        let db = add_strike("guenther", &db_path);

        std::fs::remove_file(db_path).unwrap();

        assert_eq!(db, json!({"guenther": 1}));
    }

    #[test]
    fn it_adds_a_strike_to_an_existing_db() {
        let db_path = PathBuf::from("tests/fixtures/db_0.json");
        add_strike("guenther", &db_path);
        add_strike("heinz", &db_path);

        let db = add_strike("guenther", &db_path);

        std::fs::remove_file(db_path).unwrap();

        assert_eq!(db, json!({"guenther": 2, "heinz": 1}));
    }
}
