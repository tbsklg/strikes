use std::collections::HashMap;

use async_trait::async_trait;

use crate::tarnished::Tarnished;

#[async_trait]
pub trait StrikeClient {
    fn add_strike(&self, name: &str) -> HashMap<String, i8>;
    fn get_tarnished(&self) -> Vec<Tarnished>;
    fn clear_strikes(&self);
    async fn check_health(&self) -> Result<(), String>;
}
