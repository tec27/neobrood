use bevy::{prelude::*, window::WindowMode};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, Reflect)]
#[serde(rename_all = "camelCase")]
pub enum NeobroodWindowMode {
    Windowed,
    #[default]
    BorderlessFullscreen,
    ExclusiveFullscreen,
}

impl From<NeobroodWindowMode> for WindowMode {
    fn from(value: NeobroodWindowMode) -> Self {
        match value {
            NeobroodWindowMode::Windowed => WindowMode::Windowed,
            NeobroodWindowMode::BorderlessFullscreen => WindowMode::BorderlessFullscreen,
            NeobroodWindowMode::ExclusiveFullscreen => WindowMode::Fullscreen,
        }
    }
}

#[derive(
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
    Reflect,
)]
#[serde(rename_all = "camelCase")]
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

    pub const fn scale(&self) -> f32 {
        match self {
            Self::Standard => 1.0,
            Self::High => 2.0,
            Self::ExtraHigh => 4.0,
        }
    }
}

#[derive(
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
    Reflect,
)]
#[serde(rename_all = "camelCase")]
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

const fn default_volume() -> f32 {
    1.0
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct Volumes {
    #[serde(default = "default_volume")]
    pub global: f32,
    #[serde(default = "default_volume")]
    pub music: f32,
    #[serde(default = "default_volume")]
    pub sound_effects: f32,
}

impl Default for Volumes {
    fn default() -> Self {
        Self {
            global: default_volume(),
            music: default_volume(),
            sound_effects: default_volume(),
        }
    }
}

#[derive(
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
    Reflect,
)]
#[serde(rename_all = "camelCase")]
pub enum AudioQuality {
    Classic,
    #[default]
    Remastered,
}

impl AudioQuality {
    pub const fn asset_path(&self) -> &'static str {
        match self {
            Self::Classic => "SD/sound/",
            Self::Remastered => "sound/",
        }
    }
}

// TODO(tec27): Write a way to configure these ingame and save them to the file
#[derive(Resource, Clone, Copy, Debug, Default, Serialize, Deserialize, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct GameSettings {
    #[serde(default)]
    pub window_mode: NeobroodWindowMode,
    pub window_size: Option<(u32, u32)>,
    #[serde(default)]
    pub asset_quality: AssetQuality,
    #[serde(default)]
    pub asset_pack: AssetPack,

    #[serde(default)]
    pub volumes: Volumes,
    #[serde(default)]
    pub audio_quality: AudioQuality,
}
