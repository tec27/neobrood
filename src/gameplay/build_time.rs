use bevy::prelude::*;

use crate::gamedata::ConstructTypeId;

/// Marks that a Construct is under construction and stores the time remaining until it is complete.
#[derive(Component, Debug, Copy, Clone, Default)]
#[component(storage = "SparseSet")]
pub struct UnderConstruction {
    pub time_remaining: u16,
}

impl UnderConstruction {
    pub fn for_type(c: ConstructTypeId) -> Self {
        Self {
            time_remaining: c.def().build_time,
        }
    }

    pub fn has_time_remaining(&self) -> bool {
        self.time_remaining > 0
    }
}
