use bevy::{ecs::system::Resource, math::Vec2};
use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(
    Resource,
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Default,
    Serialize,
    Deserialize,
)]
pub enum AssetQuality {
    /// Classic assets.
    Standard,
    /// High definition assets, suitable for resolutions below 4K.
    #[default]
    High,
    /// Extra high definition assets, suitable for 4K and above.
    ExtraHigh,
}

impl AssetQuality {
    pub const fn asset_path(&self) -> &'static str {
        match self {
            Self::Standard => "sd/",
            Self::High => "hd2/",
            Self::ExtraHigh => "",
        }
    }

    pub const fn tile_size(&self) -> Vec2 {
        match self {
            Self::Standard => Vec2::splat(32.0),
            Self::High => Vec2::splat(64.0),
            Self::ExtraHigh => Vec2::splat(128.0),
        }
    }
}

#[allow(unused)]
#[derive(
    Resource,
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    Default,
    Serialize,
    Deserialize,
)]
pub enum AssetPack {
    #[default]
    Standard,
    Carbot,
}

impl AssetPack {
    pub const fn asset_path(&self) -> &'static str {
        match self {
            Self::Standard => "",
            Self::Carbot => "carbot/",
        }
    }
}
