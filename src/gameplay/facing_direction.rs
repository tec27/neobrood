use bevy::prelude::*;

use crate::{gamedata::ConstructTypeId, math::FixedAngle};

use super::constructs::{ConstructImage, ConstructSprite};

/// Controls the direction that a construct is facing. Changes to this component will automatically
/// be applied to the sprites/images of the entity.
#[derive(Component, Debug, Copy, Clone, Default)]
pub struct FacingDirection(pub FixedAngle);

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
    q_sprites: Query<&Children, With<ConstructSprite>>,
    mut q_images: Query<&mut ConstructImage>,
) {
    for (sprites, facing) in q_changed.iter() {
        for images in q_sprites.iter_many(sprites) {
            let mut iter = q_images.iter_many_mut(images);
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
