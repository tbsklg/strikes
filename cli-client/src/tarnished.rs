use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Tarnished {
    pub name: String,
    pub strikes: u8,
}

impl Tarnished {
    pub fn sort_desc_by_strike(tarnished: Vec<Tarnished>) -> Vec<Tarnished> {
        let mut tarnished = tarnished.clone();
        tarnished.sort_by(|a, b| b.strikes.partial_cmp(&a.strikes).unwrap());
        tarnished
    }

    pub fn as_tarnished(db: HashMap<String, u8>) -> Vec<Tarnished> {
        db.iter()
            .map(|(name, strikes)| Tarnished {
                name: name.to_string(),
                strikes: *strikes,
            })
            .collect()
    }
}

