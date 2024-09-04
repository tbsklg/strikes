use async_trait::async_trait;

use crate::tarnished::Tarnished;

#[async_trait]
pub trait StrikeClient {
    async fn add_strike(&self, name: &str) -> Result<u8, String>;
    async fn get_tarnished(&self) -> Result<Vec<Tarnished>, String>;
    async fn clear_strikes(&self) -> Result<(), String>;
    async fn check_health(&self) -> Result<(), String>;
}
