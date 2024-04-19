// tiles.rs
use crate::{
    player::CorruptedTileTexture,
    tiles::{TileType, TILE_SIZE},
    PotentiallyCorruptedTiles, TileMap,
};
use bevy::prelude::*;
use rand::prelude::SliceRandom;

use super::Tile;

#[derive(Resource, Clone)]
pub struct CorruptionTimer {
    pub timer: Timer,
}

pub fn corruption_system(
    mut commands: Commands,
    time: Res<Time>,
    mut corruption_timer: ResMut<CorruptionTimer>,
    mut potentially_corrupted_tiles: ResMut<PotentiallyCorruptedTiles>,
    tile_query: Query<(&mut TileType, &Tile)>,
    mut tile_map: ResMut<TileMap>,
    asset_server: Res<AssetServer>,
) {
    corruption_timer.timer.tick(time.delta());

    if corruption_timer.timer.just_finished() {
        if let Some(entity) = potentially_corrupted_tiles
            .tiles
            .choose(&mut rand::thread_rng())
        {
            if let Ok((tile_type, transform)) = tile_query.get(*entity) {
                if !tile_type.is_corrupted_tile() {
                    commands.entity(*entity).despawn();

                    let new_entity = commands
                        .spawn((
                            SpriteBundle {
                                texture: asset_server.load("corrupted_tile_1.png"),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    transform.x * TILE_SIZE,
                                    transform.y * TILE_SIZE,
                                    1.0,
                                ),
                                ..default()
                            },
                            TileType::Corruption {
                                png: "corrupted_tile_1.png".to_string(),
                            },
                        ))
                        .id();

                    tile_map
                        .tiles
                        .insert((transform.x as i32, transform.y as i32), new_entity);

                    info!("+1 corrupt tile");
                    for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        if let Some(&neighbor_entity) = tile_map
                            .tiles
                            .get(&(transform.x as i32 + dx, transform.y as i32 + dy))
                        {
                            info!("Found a neighbor");
                            potentially_corrupted_tiles.tiles.push(neighbor_entity);
                        }
                    }
                } else {
                    info!("Query failed");
                }
            }

            let old_timer = corruption_timer.timer.duration().mul_f32(0.9);

            corruption_timer.timer.set_duration(old_timer);
            corruption_timer.timer.reset();
        }
    }
}
