use std::collections::HashMap;

use crate::tarnished::Tarnished;

pub trait StrikeClient {
    fn add_strike(&self, name: &str) -> HashMap<String, i8>;
    fn get_tarnished(&self) -> Vec<Tarnished>;
    fn clear_strikes(&self);
}
