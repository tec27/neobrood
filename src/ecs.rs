use bevy::prelude::*;

/// Generic system that despawns every entity with the specified component.
pub fn despawn_all<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
