use bevy::prelude::*;
use bevy::utils::HashMap;

// Constants and resources related to rendering
pub const SAFE_RADIUS: f32 = 360.0;

#[derive(Resource, Default)]
pub struct LoadedChunks {
    pub chunks: HashMap<IVec2, Entity>,
    pub corruption_tiles: HashMap<IVec2, Vec<(Entity, Transform)>>,
    pub corruption_neighbors: HashMap<IVec2, Vec<Entity>>,
}

// Systems for rendering tiles
pub fn setup_tiles(mut commands: Commands) {
    commands.insert_resource(LoadedChunks::default());
}
