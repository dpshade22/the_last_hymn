// main.rs
mod audio;
mod collectables;
mod player;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::*;

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
        .insert_resource(audio::CurrentBPM::default())
        .add_systems(
            Startup,
            (
                player::setup_player,
                audio::setup_audio,
                collectables::spawn_collectable_notes.after(player::setup_player),
            ),
        )
        .add_systems(Update, player::player_movement)
        .add_systems(Update, player::play_notes)
        .add_systems(Update, player::despawn_temporary_sprites)
        // .add_systems(Update, player::sync_player_camera)
        .add_systems(Update, collectables::collect_notes)
        .run();
}
