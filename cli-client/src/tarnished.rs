use std::collections::HashMap;

use crate::clients::remote_client::StrikesResponse;

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

    pub fn from_map(db: HashMap<String, u8>) -> Vec<Tarnished> {
        db.iter()
            .map(|(name, strikes)| Tarnished {
                name: name.to_string(),
                strikes: *strikes,
            })
            .collect()
    }

    pub fn from_vec(sr: Vec<StrikesResponse>) -> Vec<Tarnished> {
        sr.iter()
            .map(|StrikesResponse { name, strike_count }| Tarnished {
                name: name.to_string(),
                strikes: *strike_count,
            })
            .collect()
    }
}
