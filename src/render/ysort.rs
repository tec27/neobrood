use bevy::prelude::*;

/// Component that can be added to entities to make them sort based on their Y position. entities
/// will have their Z translation adjusted automatically. The singular parameter can be used to
// create layers of sorting (e.g. air units, ground units, etc.).
#[derive(Component, Debug, Default, Reflect)]
pub struct YSort(pub f32);

pub fn y_sort(mut query: Query<(&mut Transform, &YSort)>) {
    // Implementation suggested here:
    // https://twitter.com/JCatrambone/status/1562498519444836352?t=4m2NKRKNpAVBrGqS7tcxSw
    for (mut transform, ysort) in query.iter_mut() {
        // FIXME: this function doesn't work well for values exceeding ~3500 in either direction
        transform.translation.z =
            ysort.0 - (1.0f32 / (1.0f32 + (2.0f32.powf(-0.01 * transform.translation.y))));
    }
}
