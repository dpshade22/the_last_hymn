use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

// Constants and types related to tile generation
pub const TILE_SIZE: f32 = 8.0;
pub const STAGE_SIZE: (i32, i32) = (64, 64); // 256x256 tiles

#[derive(Component, Clone, PartialEq)]
pub enum TileType {
    Green { png: String },
    Grass { png: String },
    Flower { png: String },
    Sand { png: String },
    Corruption { png: String },
}

impl TileType {
    pub fn is_corrupted_tile(&self) -> bool {
        match self {
            TileType::Corruption { .. } => true,
            _ => false,
        }
    }
}

#[derive(Component, Clone)]
pub struct Tile {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource)]
pub struct TileMap {
    pub tiles: HashMap<(i32, i32), Entity>,
}

#[derive(Resource)]
pub struct PotentiallyCorruptedTiles {
    pub tiles: Vec<Entity>,
}

pub fn generate_stage(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tile_map: ResMut<TileMap>,
    mut potentially_corrupted_tiles: ResMut<PotentiallyCorruptedTiles>,
) {
    let mut rng = rand::thread_rng();
    let mut total_corrupted = 0;
    let max_corruption = 24;

    for y in 0..STAGE_SIZE.1 {
        for x in 0..STAGE_SIZE.0 {
            let tile_entity: Entity = match rng.gen_range(0.0..1.0) {
                a if a <= 0.05
                    && total_corrupted < max_corruption
                    && (x == 0 || y == 0 || x == STAGE_SIZE.0 - 1 || y == STAGE_SIZE.1 - 1) =>
                {
                    total_corrupted += 1;
                    info!("Hitting init corrupts: ");

                    spawn_tile(
                        &mut commands,
                        &asset_server,
                        &mut potentially_corrupted_tiles,
                        Tile {
                            x: x as f32,
                            y: y as f32,
                        },
                        TileType::Corruption {
                            png: "corrupted_tile_1.png".to_string(),
                        },
                        &tile_map,
                    )
                }
                a if a <= 0.08 => spawn_tile(
                    &mut commands,
                    &asset_server,
                    &mut potentially_corrupted_tiles,
                    Tile {
                        x: x as f32,
                        y: y as f32,
                    },
                    TileType::Grass {
                        png: "tile_0001.png".to_string(),
                    },
                    &tile_map,
                ),
                a if a <= 0.10 => spawn_tile(
                    &mut commands,
                    &asset_server,
                    &mut potentially_corrupted_tiles,
                    Tile {
                        x: x as f32,
                        y: y as f32,
                    },
                    TileType::Flower {
                        png: "tile_0002.png".to_string(),
                    },
                    &tile_map,
                ),
                a if a <= 0.12 => spawn_tile(
                    &mut commands,
                    &asset_server,
                    &mut potentially_corrupted_tiles,
                    Tile {
                        x: x as f32,
                        y: y as f32,
                    },
                    TileType::Sand {
                        png: "tile_0003.png".to_string(),
                    },
                    &tile_map,
                ),
                _ => spawn_tile(
                    &mut commands,
                    &asset_server,
                    &mut potentially_corrupted_tiles,
                    Tile {
                        x: x as f32,
                        y: y as f32,
                    },
                    TileType::Green {
                        png: "tile_0000.png".to_string(),
                    },
                    &tile_map,
                ),
            };

            tile_map.tiles.insert((x, y), tile_entity);
        }
    }
}

fn spawn_tile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    potentially_corrupted_tiles: &mut ResMut<PotentiallyCorruptedTiles>,
    tile: Tile,
    tile_type: TileType,
    tile_map: &ResMut<TileMap>,
) -> Entity {
    let png = match &tile_type {
        TileType::Green { png, .. } => png,
        TileType::Grass { png, .. } => png,
        TileType::Flower { png, .. } => png,
        TileType::Sand { png, .. } => png,
        TileType::Corruption { png } => png,
    };

    if tile_type.is_corrupted_tile() {
        find_and_push_neighbors(tile_map, &tile, potentially_corrupted_tiles);
    }

    let entity = commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load(png),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    tile.x * TILE_SIZE,
                    tile.y * TILE_SIZE,
                    0.0,
                )),
                ..default()
            },
            tile,
            tile_type,
        ))
        .id();

    entity
}

pub fn find_and_push_neighbors(
    tile_map: &ResMut<TileMap>,
    transform: &Tile,
    potentially_corrupted_tiles: &mut ResMut<PotentiallyCorruptedTiles>,
) {
    for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        if let Some(&neighbor_entity) = tile_map
            .tiles
            .get(&(transform.x as i32 + dx, transform.y as i32 + dy))
        {
            potentially_corrupted_tiles.tiles.push(neighbor_entity);
        }
    }
}
