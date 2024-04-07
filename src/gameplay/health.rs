use bevy::prelude::*;

use crate::{gamedata::ConstructTypeId, math::FixedPoint};

#[derive(Component, Debug, Copy, Clone, Default)]
pub struct Health {
    pub max: FixedPoint,
    pub current: FixedPoint,
}

impl Health {
    /// Creates the initial (e.g. unfinished) health component for a given construct type.
    pub fn initial(c: ConstructTypeId) -> Self {
        Self {
            max: c.max_health(),
            current: FixedPoint::ONE,
        }
    }
}
