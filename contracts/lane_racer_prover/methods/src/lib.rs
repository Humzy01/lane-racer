include!(concat!(env!("OUT_DIR"), "/methods.rs"));

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInput {
    pub seed: u64,
    pub actions: Vec<u8>,
    pub player_address: String,
    pub game_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameResult {
    pub player_address: String,
    pub game_id: u64,
    pub score: u32,
    pub obstacles_dodged: u32,
    pub gems_collected: u32,
    pub speed_reached: u32,
    pub collision_occurred: bool,
}