use bevy::prelude::*;

use self::ysort::YSort;

pub mod ysort;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<YSort>()
            // TODO(tec27): Add a schedule/ordering for this such that it runs after other position
            // adjustments we perform
            .add_systems(PostUpdate, ysort::y_sort);
    }
}
