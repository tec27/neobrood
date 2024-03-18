#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BwImage {
    pub id: u16,
    pub grp: u32,
    pub graphics_turns: u8,
    pub clickable: u8,
    pub use_full_iscript: u8,
    pub draw_if_cloaked: u8,
    pub draw_function: u8,
    pub remapping: u8,
    pub iscript: u32,
    pub shield_overlay: u32,
    pub attack_overlay: u32,
    pub damage_overlay: u32,
    pub special_overlay: u32,
    pub landing_dust_overlay: u32,
    pub lift_off_dust_overlay: u32,
}
