// main.rs
mod audio;
mod collectables;
mod player;
mod tiles;

use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;
use tiles::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AudioPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .insert_resource(TileMap {
            tiles: HashMap::new(),
        })
        .insert_resource(PotentiallyCorruptedTiles { tiles: vec![] })
        .insert_resource(CorruptionTimer {
            timer: Timer::from_seconds(10.0, TimerMode::Repeating),
        })
        .insert_resource(audio::CurrentBPM::default())
        .add_systems(
            Startup,
            (
                tiles::setup_tiles.before(collectables::spawn_collectable_notes),
                tiles::generate_stage.after(tiles::setup_tiles),
                player::setup_player.after(tiles::setup_tiles),
                audio::setup_audio,
                collectables::spawn_collectable_notes.after(player::setup_player),
            ),
        )
        .add_systems(Update, tiles::corruption_system)
        .add_systems(Update, player::player_movement)
        .add_systems(Update, player::play_notes)
        .add_systems(Update, player::despawn_temporary_sprites)
        .add_systems(Update, player::sync_player_camera)
        .add_systems(Update, collectables::collect_notes)
        .run();
}
