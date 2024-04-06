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
    pub draw_function: u8,
    pub color_shift: u8,
    pub iscript: u32,
    pub shield_overlay: u32,
    pub attack_overlay: u32,
    pub damage_overlay: u32,
    pub special_overlay: u32,
    pub landing_dust_overlay: u32,
    pub lift_off_dust_overlay: u32,
}
