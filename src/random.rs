// This implementation matches BW's random number generator, with convenience methods to get values
// in the desired type.

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<LockedLcgRand>()
        .add_systems(FixedFirst, insert_lcg_rand)
        .add_systems(FixedLast, remove_lcg_rand);
}

/// A resource that holds the current [LcgRand] but does not allow access to it. The LcgRand will be
/// inserted only for the duration of the Fixed schedule.
#[derive(Resource, Debug, Default, Clone)]
pub struct LockedLcgRand(LcgRand);

impl LockedLcgRand {
    /// Reseeds the internal [LcgRand] with the specified seed. This should *ONLY* be called during
    /// game initialization!!!
    pub fn i_know_what_im_doing_please_reseed(&mut self, seed: u32) {
        self.0.reseed(seed);
        info!("Seeded RNG with {seed}");
    }
}

fn insert_lcg_rand(mut commands: Commands, mut locked_lcg_rand: ResMut<LockedLcgRand>) {
    let lcg_rand = std::mem::take(&mut locked_lcg_rand.0);
    commands.insert_resource(lcg_rand);
}

fn remove_lcg_rand(
    mut commands: Commands,
    mut locked_lcg_rand: ResMut<LockedLcgRand>,
    lcg_rand: Res<LcgRand>,
) {
    locked_lcg_rand.0 = lcg_rand.clone();
    commands.remove_resource::<LcgRand>();
}

/// A Linear Congruential Generator (LCG) with Borland C++ constants.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect, Default)]
pub struct LcgRand {
    state: u32,
}

#[allow(dead_code)]
impl LcgRand {
    /// Creates a new [LcgRand] with the specified seed.
    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    /// Resets the state of the generator to the specified seed.
    pub fn reseed(&mut self, seed: u32) {
        self.state = seed;
    }

    /// Generates a new random number as a [u32]. This value will be in the range `[0, 0x7fff]`.
    pub fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(22695477).wrapping_add(1);
        self.state >> 16 & 0x7fff
    }

    /// Generates a new random number as an [i32]. This value will be in the range `[0, 0x7fff]`.
    #[inline]
    pub fn next_i32(&mut self) -> i32 {
        self.next_u32() as i32
    }

    /// Generates a new random number as a [usize]. This value will be in the range `[0, 0x7fff]`.
    #[inline]
    pub fn next_usize(&mut self) -> usize {
        self.next_u32() as usize
    }

    /// Generates a new random number as [u8]. This value will be in the range `[0, 0xff]`.
    pub fn next_u8(&mut self) -> u8 {
        self.next_u32() as u8
    }

    // NOTE(tec27): These range implementations match the behavior of the game, but I make no
    // guarantees about the quality of their output :)

    /// Generates a new random number as a [u32] in the range `[min, max]`.
    #[inline]
    pub fn in_range_u32(&mut self, min: u32, max: u32) -> u32 {
        min + (self.next_u32().saturating_mul(max - min + 1) >> 15)
    }

    /// Generates a new random number as an [i32] in the range `[min, max]`.
    #[inline]
    pub fn in_range_i32(&mut self, min: i32, max: i32) -> i32 {
        min + (self.next_i32().saturating_mul(max - min + 1) >> 15)
    }

    /// Generates a new random number as a [usize] in the range `[min, max]`.
    #[inline]
    pub fn in_range_usize(&mut self, min: usize, max: usize) -> usize {
        min + (self.next_usize().saturating_mul(max - min + 1) >> 15)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut lcg = LcgRand::new(42);
        let val = lcg.next_u32();
        assert_eq!(val, 14544);

        let val = lcg.next_i32();
        assert_eq!(val, 24056);

        let val = lcg.next_usize();
        assert_eq!(val, 29925);
    }

    #[test]
    fn u32_range() {
        let mut lcg = LcgRand::new(42);
        let val = lcg.in_range_u32(0, 100);
        assert_eq!(val, 44);

        for _ in 0..10000 {
            let val = lcg.in_range_u32(5, 1000);
            assert!(val >= 5);
            assert!(val <= 1000);
        }
    }

    #[test]
    fn i32_range() {
        let mut lcg = LcgRand::new(42);
        let val = lcg.in_range_i32(0, 100);
        assert_eq!(val, 44);

        for _ in 0..10000 {
            let val = lcg.in_range_i32(5, 1000);
            assert!(val >= 5);
            assert!(val <= 1000);
        }
    }

    #[test]
    fn usize_range() {
        let mut lcg = LcgRand::new(42);
        let val = lcg.in_range_usize(0, 100);
        assert_eq!(val, 44);

        for _ in 0..10000 {
            let val = lcg.in_range_usize(5, 1000);
            assert!(val >= 5);
            assert!(val <= 1000);
        }
    }

    #[test]
    fn reseed() {
        let mut lcg = LcgRand::new(42);
        let val = lcg.next_u32();
        assert_eq!(val, 14544);

        lcg.reseed(42);
        let val = lcg.next_u32();
        assert_eq!(val, 14544);
    }
}
