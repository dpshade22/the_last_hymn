// tiles.rs
use crate::{
    find_and_push_neighbors,
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

    if corruption_timer.timer.just_finished() && !potentially_corrupted_tiles.tiles.is_empty() {
        loop {
            if let Some(entity) = potentially_corrupted_tiles
                .tiles
                .choose(&mut rand::thread_rng())
            {
                if let Ok((tile_type, old_tile)) = tile_query.get(*entity) {
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
                                        old_tile.x * TILE_SIZE,
                                        old_tile.y * TILE_SIZE,
                                        1.0,
                                    ),
                                    ..default()
                                },
                                old_tile.clone(),
                                TileType::Corruption {
                                    png: "corrupted_tile_1.png".to_string(),
                                },
                            ))
                            .id();

                        tile_map
                            .tiles
                            .insert((old_tile.x as i32, old_tile.y as i32), new_entity);

                        info!("+1 corrupt tile");
                        find_and_push_neighbors(
                            &tile_map,
                            &old_tile,
                            &mut potentially_corrupted_tiles,
                        );
                        break;
                    } else {
                        info!("Query failed");
                        if potentially_corrupted_tiles.tiles.is_empty() {
                            break;
                        }
                    }
                }
            }
        }

        let old_timer = corruption_timer.timer.duration().mul_f32(0.95);
        info!("{:?}", old_timer);

        corruption_timer.timer.set_duration(old_timer);
        corruption_timer.timer.reset();
    }
}
