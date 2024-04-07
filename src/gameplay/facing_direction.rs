use bevy::prelude::*;

use crate::{gamedata::ConstructTypeId, math::FixedAngle};

use super::constructs::{ConstructImage, ConstructSprite};

/// Controls the direction that a construct is facing. Changes to this component will automatically
/// be applied to the sprites/images of the entity.
#[derive(Component, Debug, Copy, Clone, Default)]
pub struct FacingDirection(pub FixedAngle);

/// Returns the index of the frame in a sprite sheet that corresponds to a given direction. Assumes
/// there are sprites for 32 possible directions.
fn direction_to_frame_index(direction: FixedAngle) -> usize {
    (direction.to_bits() as usize + 4) / 8
}

pub fn apply_facing_to_images(
    q_changed: Query<
        (&Children, &FacingDirection),
        (With<ConstructTypeId>, Changed<FacingDirection>),
    >,
    q_sprites: Query<&Children, With<ConstructSprite>>,
    mut q_images: Query<(&ConstructImage, &mut TextureAtlas, &mut Sprite)>,
) {
    for (sprites, facing) in q_changed.iter() {
        for images in q_sprites.iter_many(sprites) {
            let mut iter = q_images.iter_many_mut(images);
            while let Some((ty, mut atlas, mut sprite)) = iter.fetch_next() {
                if ty.def().has_directional_frames {
                    let mut frame = direction_to_frame_index(facing.0);
                    let mut flip_x = false;
                    if frame > 16 {
                        frame = 32 - frame;
                        flip_x = true;
                    }

                    // TODO(tec27): Handle frame offsets from special states (iscript?)

                    if atlas.index != frame {
                        atlas.index = frame;
                    }
                    if sprite.flip_x != flip_x {
                        sprite.flip_x = flip_x;
                    }
                }
            }
        }
    }
}
