// tiles.rs
use crate::{
    player::CorruptedTileTexture,
    tiles::{TileType, TILE_SIZE},
    CorruptionRateAdder,
};
use bevy::prelude::*;
use rand::Rng;

use super::Tile;

pub const SAFE_RADIUS: f32 = 360.0; // Declare the safe radius constant

pub const CORRUPTION_RATE_ADDER: f32 = 0.01; // Declare the safe radius constant

#[derive(Component)]
pub struct CorruptedTile {
    pub spread_timer: Timer,
    pub spread_rate: f32, // Rate at which the corruption spreads
}
