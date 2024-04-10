use bitflags::bitflags;
use std::{num::NonZeroU16, ops::Range};

use bevy::reflect::Reflect;

use crate::random::{LcgRand, LcgRandGen};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BwSound {
    pub id: BwSoundId,
    pub file: &'static str,
    pub priority: u8,
    pub flags: BwSoundFlags,
    pub length_adjustment: u16,
    pub min_volume: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
pub struct BwSoundId(pub NonZeroU16);

impl BwSoundId {
    #[inline]
    pub fn new(id: u16) -> Option<Self> {
        NonZeroU16::new(id).map(Self)
    }

    /// Creates a [BwSoundId] without checking if the id is zero.
    ///
    /// # Safety
    /// The id must be non-zero.
    #[inline]
    pub const unsafe fn new_unchecked(id: u16) -> Self {
        Self(NonZeroU16::new_unchecked(id))
    }

    #[inline]
    pub const fn get(&self) -> u16 {
        self.0.get()
    }
}

/// A range of sound IDs that can be played for a given event.
///
/// NOTE(tec27): This is not using `Range` because it cannot be `Copy` and has a number of issues
/// when stored.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Reflect)]
pub struct BwSoundRange {
    start: BwSoundId,
    end: BwSoundId,
}

impl BwSoundRange {
    #[inline]
    pub const fn new(start: BwSoundId, end: BwSoundId) -> Self {
        Self { start, end }
    }

    #[inline]
    pub const fn contains(&self, id: BwSoundId) -> bool {
        id.get() >= self.start.get() && id.get() < self.end.get()
    }
}

impl From<BwSoundRange> for Range<BwSoundId> {
    fn from(range: BwSoundRange) -> Self {
        range.start..range.end
    }
}

impl LcgRandGen for BwSoundRange {
    type Output = BwSoundId;

    fn gen_random(&self, rng: &mut LcgRand) -> Self::Output {
        BwSoundId::new(rng.in_range_u16(self.start.get(), self.end.get() - 1)).unwrap()
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BwSoundFlags: u8 {
        const PRELOAD = 1;
        const UNIT_SPEECH = 2;
        const ONE_AT_A_TIME = 16;
        const NEVER_PREEMPT = 32;
    }
}
