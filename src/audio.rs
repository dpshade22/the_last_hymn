// audio.rs
use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_kira_audio::{Audio, AudioControl, AudioEasing, AudioSource, AudioTween};

pub const NOTES: &[&str] = &[
    "C3.wav",
    "Csharp3.wav",
    "D3.wav",
    "Dsharp3.wav",
    "E3.wav",
    "F3.wav",
    "Fsharp3.wav",
    "G3.wav",
    "Gsharp3.wav",
    "A3.wav",
    "Asharp3.wav",
    "B3.wav",
    "C4.wav",
    "Csharp4.wav",
    "D4.wav",
];
pub const BPM: f32 = 80.0; // Beats per minute
pub const EIGHTH_NOTE_DURATION: f32 = 60.0 / BPM / 2.0; // Duration of an eighth note in seconds

#[derive(Resource)]
pub struct CurrentBPM {
    pub bpm: f32,
    pub eighth_note_duration: f32,
}

impl Default for CurrentBPM {
    fn default() -> Self {
        CurrentBPM {
            bpm: BPM,
            eighth_note_duration: EIGHTH_NOTE_DURATION,
        }
    }
}

#[derive(Clone)]
pub struct Song(pub Vec<(Option<Note>, f32)>);

impl Song {
    pub fn new(notes: &[(Option<usize>, f32)]) -> Self {
        Song(
            notes
                .iter()
                .map(|&(i, d)| (i.map(Note), d * EIGHTH_NOTE_DURATION))
                .collect(),
        )
    }
}

#[derive(Clone, Copy, Component, PartialEq, Eq, Debug, Hash)]
pub struct Note(pub usize);

#[derive(Resource)]
pub struct NoteAudioHandles {
    pub handles: Vec<Handle<AudioSource>>,
}

pub fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let handles = NOTES.iter().map(|file| asset_server.load(*file)).collect();
    commands.insert_resource(NoteAudioHandles { handles });

    // Load and play the soundscape
    let soundscape = asset_server.load("D-soundscape.wav");
    audio
        .play(soundscape)
        .loop_from(0.5)
        .with_volume(0.15)
        .fade_in(AudioTween::new(
            Duration::from_secs(2),
            AudioEasing::OutPowi(2),
        ))
        .reverse();
}
