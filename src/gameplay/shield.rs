use bevy::prelude::*;

use crate::{gamedata::ConstructTypeId, math::FixedPoint};

#[derive(Component, Debug, Copy, Clone, Default)]
pub struct Shield {
    pub max: FixedPoint,
    pub current: FixedPoint,
}

impl Shield {
    /// Creates the initial shield component for a given construct type.
    pub fn initial(c: ConstructTypeId) -> Option<Self> {
        c.shield_points().map(|s| Self {
            max: s,
            current: if c.is_building() { s / 10 } else { s },
        })
    }
}
