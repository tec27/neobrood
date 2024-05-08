use bevy::prelude::*;

/// [Component] that describes how many of a particular resource type an entity contains.
#[derive(Component, Debug, Copy, Clone, Eq, PartialEq, Reflect)]
pub enum ResourceAmount {
    Minerals(u32),
    Gas(u32),
}
