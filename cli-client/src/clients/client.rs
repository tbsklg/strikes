use async_trait::async_trait;

use crate::tarnished::Tarnished;

#[async_trait]
pub trait StrikeClient {
    async fn add_strike(&self, name: &str) -> Result<u8, String>;
    async fn get_tarnished(&self) -> Result<Vec<Tarnished>, String>;
    fn clear_strikes(&self);
    async fn check_health(&self) -> Result<(), String>;
}
