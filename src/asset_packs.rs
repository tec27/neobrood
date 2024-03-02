#[allow(unused)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
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
    pub fn asset_path(&self) -> &'static str {
        match self {
            Self::Standard => "sd/",
            Self::High => "hd2/",
            Self::ExtraHigh => "",
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
pub enum AssetPack {
    #[default]
    Standard,
    Carbot,
}

impl AssetPack {
    pub fn asset_path(&self) -> &'static str {
        match self {
            Self::Standard => "",
            Self::Carbot => "carbot/",
        }
    }
}
