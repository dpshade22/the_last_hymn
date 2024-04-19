// player.rs

use crate::{
    audio::{CurrentBPM, Note, NoteAudioHandles, Song, EIGHTH_NOTE_DURATION},
    TILE_SIZE,
};
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_kira_audio::{Audio, AudioControl, AudioEasing, AudioPlugin, AudioSource, AudioTween};
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
pub struct CorruptedTileTexture(pub Handle<Image>);

#[derive(Resource, Component, Clone)]
pub struct Player {
    pub current_notes: Vec<Note>,
    pub current_song: Song,
    pub note_index: usize,
    pub timer: MyTimer,
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = Player {
        current_notes: vec![],
        current_song: Song::new(&[
            (Some(2), 1.0),  // Eighth note
            (Some(6), 1.0),  // Eighth note
            (Some(9), 3.0),  // Half note (sustained)
            (Some(11), 1.0), // Quarter note (sustained)
            (Some(9), 1.0),  // Half note (sustained)
            (Some(6), 1.0),
            (Some(2), 4.0),
            (Some(2), 2.0),
            (Some(4), 3.0),
            (Some(4), 1.0),
            (Some(2), 1.0),
            (Some(4), 1.0),
            (Some(6), 4.0),
            (Some(2), 1.0),
            (Some(6), 1.0),
            (Some(9), 3.0),
            (Some(11), 1.0),
            (Some(9), 1.0),
            (Some(6), 1.0),
            (Some(2), 4.0),
            (Some(2), 1.0),
            (Some(4), 1.0),
            (Some(6), 3.0),
            (Some(7), 1.0),
            (Some(6), 1.0),
            (Some(4), 1.0),
            (Some(2), 6.0),
            (Some(9), 3.0),
            (Some(11), 1.0),
            (Some(9), 1.0),
            (Some(6), 1.0),
            (Some(14), 6.0),
            (Some(9), 3.0),
            (Some(11), 1.0),
            (Some(9), 1.0),
            (Some(6), 1.0),
            (Some(4), 6.0),
            (Some(9), 3.0),
            (Some(11), 1.0),
            (Some(9), 1.0),
            (Some(6), 1.0),
            (Some(14), 1.0),
            (Some(13), 1.0),
            (Some(11), 2.0),
            (Some(14), 2.0),
            (Some(6), 3.0),
            (Some(7), 1.0),
            (Some(6), 1.0),
            (Some(4), 1.0),
            (Some(2), 4.0),
        ]),
        timer: MyTimer {
            timer: Timer::from_seconds(EIGHTH_NOTE_DURATION, TimerMode::Repeating),
            duration: EIGHTH_NOTE_DURATION,
        },
        note_index: 0,
    };

    commands.insert_resource(CorruptedTileTexture(
        asset_server.load("corrupted_tile_1.png"),
    ));

    // Camera
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.25; // Zoom in by a factor of 2
    commands.spawn(camera);

    // Player
    commands.insert_resource(player.clone());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("tile_0088.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(128.0 * TILE_SIZE, 128.0 * TILE_SIZE, 100.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(4.0),
        ActiveEvents::COLLISION_EVENTS,
        ActiveCollisionTypes::all(),
        player,
    ));
}

pub fn play_notes(
    keyboard: Res<ButtonInput<KeyCode>>,
    audio: Res<Audio>,
    note_handles: Res<NoteAudioHandles>,
    mut player: ResMut<Player>,
    time: Res<Time>,
    commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    if keyboard.pressed(KeyCode::KeyA)
        || keyboard.pressed(KeyCode::KeyS)
        || keyboard.pressed(KeyCode::KeyW)
        || keyboard.pressed(KeyCode::KeyD)
    {
        player.timer.timer.tick(time.delta());

        if player.timer.timer.finished() {
            while let Some(&(note, duration)) = player.current_song.0.get(player.note_index) {
                if let Some(note) = note {
                    if player.current_notes.contains(&note) {
                        let note_handle = note_handles.handles[note.0].clone();
                        audio.play(note_handle).with_volume(5.0);

                        if let Ok(player_transform) = player_query.get_single() {
                            spawn_temporary_sprite(
                                commands,
                                asset_server,
                                player_transform,
                                duration,
                            );
                        }

                        player.timer.duration = duration;
                        player
                            .timer
                            .timer
                            .set_duration(Duration::from_secs_f32(duration));
                        player.note_index += 1;
                        break;
                    }
                }
                player.note_index += 1;
            }

            if player.note_index >= player.current_song.0.len() {
                player.note_index = 0;
            }
        }
    } else {
        player.timer.timer.reset();
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    current_bpm: Res<CurrentBPM>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * current_bpm.bpm * time.delta_seconds();
    }
}

pub fn sync_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera.get_single_mut() else {
        return;
    };

    camera_transform.translation = player_transform.translation;
}

#[derive(Component)]
pub struct TemporarySprite;

#[derive(Component, Clone)]
pub struct MyTimer {
    pub timer: Timer,
    pub duration: f32,
}

fn spawn_temporary_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_transform: &Transform,
    duration: f32,
) {
    let transform = player_transform.clone();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("tile_0088.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(8.0, 8.0)),
                ..default()
            },
            transform,
            ..default()
        },
        TemporarySprite,
        MyTimer {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            duration,
        },
    ));
}

pub fn despawn_temporary_sprites(
    mut commands: Commands,
    time: Res<Time>,
    player: Res<Player>,
    mut sprite_query: Query<(Entity, &mut Sprite, &mut MyTimer), With<TemporarySprite>>,
) {
    for (entity, mut sprite, mut timer) in sprite_query.iter_mut() {
        timer.timer.tick(time.delta());

        // Get the duration of the current note from the player's song
        let current_note_duration = player
            .note_index
            .checked_sub(1)
            .and_then(|index| player.current_song.0.get(index))
            .map(|&(_, duration)| duration)
            .unwrap_or(0.0);

        // Calculate the remaining time ratio based on the current note duration
        let remaining_time_ratio =
            timer.timer.fraction_remaining() * current_note_duration / timer.duration;

        // Fade out the sprite based on the remaining time
        sprite.color.set_a(remaining_time_ratio);

        if timer.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
