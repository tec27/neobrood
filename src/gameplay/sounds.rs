use std::ops::Index;

use bevy::{ecs::system::Command, prelude::*};

use crate::{
    gamedata::{BwSound, BwSoundId, SOUNDS},
    maps::position::Position,
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
}

struct PlaySoundCommand {
    sound_id: BwSoundId,
    #[allow(dead_code)]
    position: Option<Position>,
}

impl Command for PlaySoundCommand {
    fn apply(self, world: &mut World) {
        let assets = world.get_resource::<AssetServer>().unwrap();
        // TODO(tec27): Adjust volume + length + play based on priority/etc.
        // TODO(tec27): Allow picking audio quality
        world.spawn(AudioBundle {
            source: assets.load(format!("casc-extracted\\sound\\{}", self.sound_id.file())),
            ..default()
        });
    }
}

pub trait PlaySoundCommandsExt {
    fn play_sound(&mut self, sound_id: BwSoundId);
    fn play_sound_at(&mut self, sound_id: BwSoundId, position: Position);
}

impl<'w, 's> PlaySoundCommandsExt for Commands<'w, 's> {
    fn play_sound(&mut self, sound_id: BwSoundId) {
        self.add(PlaySoundCommand {
            sound_id,
            position: None,
        });
    }

    fn play_sound_at(&mut self, sound_id: BwSoundId, position: Position) {
        self.add(PlaySoundCommand {
            sound_id,
            position: Some(position),
        });
    }
}
