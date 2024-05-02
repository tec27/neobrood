use fixed::{types::extra::U8, FixedI32, FixedU8};

pub mod bounds;

/// The fixed-point format used by BW when fractional values are needed. This has 24 integer bits
/// and 8 fractional bits.
pub type FixedPoint = FixedI32<U8>;

/// Specifies an angle in fixed-point format. BW uses this for specifying the direction a unit is
/// facing.
pub type FixedAngle = FixedU8<U8>;
