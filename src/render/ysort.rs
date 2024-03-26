use bevy::prelude::*;

/// Component that can be added to entities to make them sort based on their Y position. entities
/// will have their Z translation adjusted automatically. The singular parameter can be used to
// create layers of sorting (e.g. air units, ground units, etc.).
#[derive(Component, Debug, Default, Reflect)]
pub struct YSort(pub f32);

pub fn y_sort(mut query: Query<(&mut Transform, &YSort)>) {
    // Implementation suggested here:
    // https://twitter.com/JCatrambone/status/1562498519444836352?t=4m2NKRKNpAVBrGqS7tcxSw
    // Formula adjusted to better work with the range of Y values BW positions have.
    for (mut transform, ysort) in query.iter_mut() {
        let y = transform.translation.y;
        let new_z = ysort.0 - (1.0f32 / (1.0f32 + 2.0f32.powf(-0.0001 * y)));
        if new_z != transform.translation.z {
            transform.translation.z = new_z;
        }
    }
}
