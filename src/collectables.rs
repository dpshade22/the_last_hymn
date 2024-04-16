use crate::audio::Note;
use crate::player::Player;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct CollectableNote;

pub fn spawn_collectable_notes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Res<Player>,
) {
    let notes_to_collect: Vec<Note> = player
        .current_song
        .0
        .iter()
        .filter_map(|&(note, _)| note)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    for (i, note) in notes_to_collect.iter().enumerate() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("tile_0029.png"),
                transform: Transform::from_xyz((i as f32) * 50.0, 0.0, 0.0),
                ..default()
            },
            RigidBody::Fixed,
            Collider::ball(8.0),
            CollectableNote,
            *note,
        ));
    }
}

pub fn collect_notes(
    mut commands: Commands,
    mut player: ResMut<Player>,
    mut collectable_notes_query: Query<(Entity, &Transform, &Note), With<CollectableNote>>,
    player_query: Query<(Entity, &Transform, &Collider), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok((player_entity, _, _)) = player_query.get_single() {
        for (note_entity, _, note) in collectable_notes_query.iter_mut() {
            if let Some(contact_pair) = rapier_context.contact_pair(player_entity, note_entity) {
                info!("Picked up {:?}", *note);
                if contact_pair.raw.has_any_active_contact {
                    player.current_notes.push(*note);
                    commands.entity(note_entity).despawn();
                }
            }
        }
    }
}
