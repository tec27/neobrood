use std::ops::Index;

use bevy::{
    audio::{PlaybackMode, Volume},
    ecs::system::Command,
    prelude::*,
};

use crate::{
    gamedata::{BwSound, BwSoundFlags, BwSoundId, ConstructTypeId, SOUNDS},
    maps::position::Position,
    settings::GameSettings,
};

impl Index<BwSoundId> for [BwSound; 1144] {
    type Output = BwSound;

    #[inline]
    fn index(&self, index: BwSoundId) -> &Self::Output {
        &self[index.get() as usize]
    }
}

impl BwSoundId {
    #[inline]
    pub fn def(&self) -> &BwSound {
        &SOUNDS[*self]
    }

    #[inline]
    pub fn file(&self) -> &'static str {
        self.def().file
    }

    #[inline]
    pub fn is_unit_speech(&self) -> bool {
        self.def().flags.contains(BwSoundFlags::UNIT_SPEECH)
    }

    #[inline]
    pub fn is_never_preempt(&self) -> bool {
        self.def().flags.contains(BwSoundFlags::NEVER_PREEMPT)
    }

    #[inline]
    pub fn is_one_at_a_time(&self) -> bool {
        self.def().flags.contains(BwSoundFlags::ONE_AT_A_TIME)
    }
}

/// Marker component on audio played via a `PlaySoundCommand`.
#[derive(Component, Copy, Clone, Debug, Reflect)]
pub struct PlayingSound;

/// Marker component on playing audio that is unit speech. Only one unit speech sound is allowed to
/// play at any given time per construct type.
#[derive(Component, Copy, Clone, Debug, Reflect)]
pub struct UnitSpeech(pub ConstructTypeId);

/// Marker component for playing audio that shouldn't be preempted by new sound playback requests.
/// Preemption will only happen for sounds of the same type (e.g. unit speech).
#[derive(Component, Copy, Clone, Debug, Reflect)]
pub struct NeverPreempt;

#[derive(Debug, Copy, Clone)]
struct PlaySoundCommand {
    sound: BwSoundId,
    construct_type: Option<ConstructTypeId>,
    #[allow(dead_code)]
    position: Option<Position>,
}

impl Default for PlaySoundCommand {
    fn default() -> Self {
        Self {
            sound: BwSoundId::new(1).unwrap(),
            construct_type: Default::default(),
            position: Default::default(),
        }
    }
}

impl Command for PlaySoundCommand {
    fn apply(self, world: &mut World) {
        // TODO(tec27): Adjust volume + length + play based on priority/etc.
        // TODO(tec27): Handle one-at-a-time, seems to be for sound effects that would be annoying
        // if duplicated (e.g. repair sounds)?
        let mut to_despawn = Vec::new();
        if self.sound.is_unit_speech() {
            let construct_type = self.construct_type.unwrap_or_default();
            let mut playing_sounds = world.query::<(Entity, &UnitSpeech, Has<NeverPreempt>)>();

            for (s, unit, has_never_preempt) in playing_sounds.iter(world) {
                if unit.0 == construct_type {
                    if has_never_preempt {
                        // We won't preempt this sound to play a new one, so we're done
                        return;
                    } else {
                        // We're replacing this sound, so despawn it
                        to_despawn.push(s);
                    }
                }
            }
        }

        for s in to_despawn {
            world.despawn(s);
        }

        let settings = world.get_resource::<GameSettings>().unwrap();
        let assets = world.get_resource::<AssetServer>().unwrap();
        let mut entity = world.spawn((
            AudioBundle {
                source: assets.load(format!(
                    "casc-extracted/{}{}",
                    settings.audio_quality.asset_path(),
                    self.sound.file()
                )),
                settings: PlaybackSettings {
                    volume: Volume::new(settings.volumes.sound_effects),
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
            },
            PlayingSound,
        ));
        if self.sound.is_unit_speech() {
            if let Some(construct_type) = self.construct_type {
                entity.insert(UnitSpeech(construct_type));
            } else {
                warn!(
                    "Playing unit speech sound {:?} without a construct type",
                    self.sound
                )
            }
        }
        if self.sound.is_never_preempt() {
            entity.insert(NeverPreempt);
        }
    }
}

pub trait PlaySoundCommandsExt {
    fn play_sound(&mut self, sound_id: BwSoundId);
    fn play_sound_from(
        &mut self,
        sound_id: BwSoundId,
        construct: ConstructTypeId,
        position: Position,
    );
    fn play_sound_at(&mut self, sound_id: BwSoundId, position: Position);
}

impl<'w, 's> PlaySoundCommandsExt for Commands<'w, 's> {
    fn play_sound(&mut self, sound_id: BwSoundId) {
        self.add(PlaySoundCommand {
            sound: sound_id,
            ..default()
        });
    }

    fn play_sound_from(
        &mut self,
        sound_id: BwSoundId,
        construct: ConstructTypeId,
        position: Position,
    ) {
        self.add(PlaySoundCommand {
            sound: sound_id,
            construct_type: Some(construct),
            position: Some(position),
        });
    }

    fn play_sound_at(&mut self, sound_id: BwSoundId, position: Position) {
        self.add(PlaySoundCommand {
            sound: sound_id,
            position: Some(position),
            ..default()
        });
    }
}
