use bevy::prelude::*;

use crate::{gamedata::ConstructTypeId, math::FixedAngle};

use super::constructs::{ConstructImage, ConstructSprite};

/// Controls the direction that a construct is facing. Changes to this component will automatically
/// be applied to the sprites/images of the entity.
#[derive(Component, Debug, Copy, Clone, Default)]
pub struct FacingDirection(pub FixedAngle);

/// The amount that each successive frame of a sprite turns.
const ANGLE_PER_SPRITE: FixedAngle = FixedAngle::from_bits(8);

impl FacingDirection {
    /// Sets the angle of the facing direction given a 0-31 direction value (which corresponds to
    /// a particular unique frame of the sprite).
    pub fn set_angle_by_direction(&mut self, direction: u8) {
        debug_assert!(direction < 32);
        self.0 = ANGLE_PER_SPRITE * direction;
    }

    /// Turns clockwise by a number of sprite frames worth of angle.
    pub fn turn_clockwise(&mut self, num_sprites: u8) {
        self.0 = self.0.wrapping_add(ANGLE_PER_SPRITE * num_sprites);
    }

    /// Turns counter-clockwise by a number of sprite frames worth of angle.
    pub fn turn_counter_clockwise(&mut self, num_sprites: u8) {
        self.0 = self.0.wrapping_sub(ANGLE_PER_SPRITE * num_sprites);
    }
}

/// Returns the index of the frame in a sprite sheet that corresponds to a given direction. Assumes
/// there are sprites for 32 possible directions.
fn direction_to_frame_index(direction: FixedAngle) -> u16 {
    (direction.to_bits() as u16 + 4) / 8
}

pub fn apply_facing_to_images(
    q_changed: Query<
        (&Children, &FacingDirection),
        (With<ConstructTypeId>, Changed<FacingDirection>),
    >,
    q_sprites: Query<&ConstructSprite>,
    mut q_images: Query<&mut ConstructImage>,
) {
    for (sprites, facing) in q_changed.iter() {
        for sprite in q_sprites.iter_many(sprites) {
            let mut iter = q_images.iter_many_mut(sprite.images.iter());
            while let Some(mut image) = iter.fetch_next() {
                if image.def().has_directional_frames {
                    let mut frame = direction_to_frame_index(facing.0);
                    let mut flip_x = false;
                    if frame > 16 {
                        frame = 32 - frame;
                        flip_x = true;
                    }

                    if image.frame_offset != frame || image.flip_x != flip_x {
                        image.frame_offset = frame;
                        image.flip_x = flip_x;
                    }
                }
            }
        }
    }
}
