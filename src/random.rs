// This implementation matches BW's random number generator, with convenience methods to get values
// in the desired type.

use bevy::{ecs::system::Resource, reflect::Reflect};

/// A Linear Congruential Generator (LCG) with Borland C++ constants.
#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect)]
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
