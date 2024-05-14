use std::num::NonZeroU32;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BwImage {
    pub id: u16,
    pub grp: u32,
    pub has_directional_frames: bool,
    pub clickable: bool,
    /// If false, only init/death iscripts are present, otherwise there are more.
    pub use_full_iscript: bool,
    /// If true, the image is drawn even if the unit is cloaked.
    pub always_visible: bool,
    /// Modifies the way the image is drawn (effects).
    pub render_style: Option<RenderStyle>,
    pub color_shift: u8,
    pub iscript: u32,
    pub shield_overlay: Option<NonZeroU32>,
    pub attack_overlay: Option<NonZeroU32>,
    pub damage_overlay: Option<NonZeroU32>,
    pub special_overlay: Option<NonZeroU32>,
    pub landing_dust_overlay: Option<NonZeroU32>,
    pub lift_off_dust_overlay: Option<NonZeroU32>,
}

/// Describes a method of rendering a BwImage to change how it looks.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RenderStyle {
    OverlayOnTarget,
    EnemyUnitCloak,
    OwnUnitCloak,
    AllyUnitCloak,
    OwnUnitCloak2,
    OwnUnitCloakDrawOnly,
    // My favorite :)
    Crash,
    EmpShockwave,
    UseRemapping,
    Shadow,
    HpFloatDraw,
    WarpFlash,
    Outline,
    PlayerSide,
    BoundingRect,
    Hallucination,
    WarpFlash2,
}
