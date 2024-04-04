use bevy::{prelude::*, transform::TransformSystem};

use crate::maps::position::apply_position_to_transform;

use self::ysort::YSort;

pub mod ysort;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<YSort>().add_systems(
            PostUpdate,
            ysort::y_sort
                .before(TransformSystem::TransformPropagate)
                // TODO(tec27): Add a custom schedule for this instead
                .after(apply_position_to_transform),
        );
    }
}
