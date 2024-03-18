use bevy::math::{I16Vec2, IRect};

use super::{BwImage, Flingy};

/// A thing that can be constructed by a player (e.g. a unit, a building, etc.). Note that not all
/// of these are actually buildable by players in normal gameplay, it is just hard to come up with
/// a name to describe this group of stuff :)
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Construct {
    pub id: u16,
    pub flingy_id: u8,
    pub sub_unit_1: u16,
    pub sub_unit_2: u16,
    pub construction_image_id: u32,
    pub unit_direction: u8,
    pub shield_enabled: u8,
    pub shield_amount: i16,
    pub hit_points: i32,
    pub elevation_level: u8,
    pub unknown_0: u8,
    pub sub_label: u8,
    pub computer_ai_idle: u8,
    pub human_ai_idle: u8,
    pub return_to_idle: u8,
    pub attack_unit: u8,
    pub attack_move: u8,
    pub ground_weapon: u8,
    pub max_ground_hits: u8,
    pub air_weapon: u8,
    pub max_air_hits: u8,
    pub ai_internal: u8,
    pub special_ability_flags: u32,
    pub target_acquisition_range: u8,
    pub sight_range: u8,
    pub armor_upgrade: u8,
    pub unit_size: u8,
    pub armor: u8,
    pub right_click_action: u8,
    pub what_sound_start: u16,
    pub what_sound_end: u16,
    pub placebox_size: I16Vec2,
    pub unit_rect: IRect,
    pub portrait: u16,
    pub mineral_cost: u16,
    pub vespene_cost: u16,
    pub build_time: u16,
    pub requirement_index: u16,
    pub star_edit_group_flags: u8,
    pub supply_provided: u8,
    pub supply_required: u8,
    pub space_required: u8,
    pub space_provided: u8,
    pub build_score: u16,
    pub destroy_score: u16,
    pub unit_map_string: u16,
    pub is_brood_war: bool,
    pub star_edit_availability_flag: u16,

    /// Contains data specific to the underlying kind of this [Construct] (e.g. data that pertains
    /// only to buildings).
    pub kind: ConstructKind,
}

/// Describes the kind of a [Construct] and any data specific to that kind.
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum ConstructKind {
    Building(BuildingData),
    Unit(UnitData),
    Other,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct BuildingData {
    pub infestation: u16,
    pub addon_size: I16Vec2,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct UnitData {
    pub ready_sound: u16,
    pub piss_sound_start: u16,
    pub piss_sound_end: u16,
    pub yes_sound_start: u16,
    pub yes_sound_end: u16,
}

impl Construct {
    #[inline]
    pub fn flingy(&self) -> &'static Flingy {
        &super::FLINGIES[self.flingy_id as usize]
    }

    #[inline]
    pub fn construction_image(&self) -> &'static BwImage {
        &super::IMAGES[self.construction_image_id as usize]
    }
}
