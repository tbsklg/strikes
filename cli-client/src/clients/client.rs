use async_trait::async_trait;

use crate::tarnished::Tarnished;

#[async_trait]
pub trait StrikeClient {
    async fn add_strike(&self, name: &str) -> Result<i8, String>;
    fn get_tarnished(&self) -> Vec<Tarnished>;
    fn clear_strikes(&self);
    async fn check_health(&self) -> Result<(), String>;
}
