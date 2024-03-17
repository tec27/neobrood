use anyhow::anyhow;
use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext},
    reflect::{Reflect, TypePath},
    utils::BoxedFuture,
};
use byteorder::{LittleEndian, ReadBytesExt};

use crate::bytes::{ByteReadable, ReadByteArraysExt};

// TODO(tec27): Move this somewhere else
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Reflect)]
pub struct Point16 {
    pub x: i16,
    pub y: i16,
}

impl ByteReadable for Point16 {
    fn read(reader: &mut impl ReadBytesExt) -> anyhow::Result<Self> {
        Ok(Self {
            x: reader.read_i16::<LittleEndian>()?,
            y: reader.read_i16::<LittleEndian>()?,
        })
    }
}

// TODO(tec27): Move this somewhere else
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug, Reflect)]
pub struct Rect16 {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

impl ByteReadable for Rect16 {
    fn read(reader: &mut impl ReadBytesExt) -> anyhow::Result<Self> {
        Ok(Self {
            left: reader.read_i16::<LittleEndian>()?,
            top: reader.read_i16::<LittleEndian>()?,
            right: reader.read_i16::<LittleEndian>()?,
            bottom: reader.read_i16::<LittleEndian>()?,
        })
    }
}

#[derive(Debug, Default)]
pub struct DatAssetLoader {}

// NOTE(tec27): DAT files are just arbitrary binary data, but Bevy's asset loader is based (pretty
// sensibly) around file extensions. We use the file extension to know we need to load a DAT file,
// but then we have to do our own mapping to a proper loader/output type.
#[derive(Asset, Debug, TypePath)]
pub enum DatAsset {
    Images(Box<ImageData>),
    Sprites(Box<SpriteData>),
    Units(Box<UnitData>),
}

impl TryFrom<&DatAsset> for ImageData {
    type Error = anyhow::Error;

    fn try_from(value: &DatAsset) -> Result<Self, Self::Error> {
        match value {
            DatAsset::Images(data) => Ok((**data).clone()),
            _ => Err(anyhow!(
                "Tried to convert a non-images DatAsset to ImageData"
            )),
        }
    }
}

impl TryFrom<&DatAsset> for SpriteData {
    type Error = anyhow::Error;

    fn try_from(value: &DatAsset) -> Result<Self, Self::Error> {
        match value {
            DatAsset::Sprites(data) => Ok((**data).clone()),
            _ => Err(anyhow!(
                "Tried to convert a non-sprites DatAsset to SpriteData"
            )),
        }
    }
}

impl TryFrom<&DatAsset> for UnitData {
    type Error = anyhow::Error;

    fn try_from(value: &DatAsset) -> Result<Self, Self::Error> {
        match value {
            DatAsset::Units(data) => Ok((**data).clone()),
            _ => Err(anyhow!("Tried to convert a non-units DatAsset to UnitData")),
        }
    }
}

/// How many images are specified in the images.dat file.
const NUM_IMAGE_DATA: usize = 999;
/// How much data each image instance takes up in the images.dat file (in bytes).
const IMAGE_DATA_SIZE: usize = 38;

#[allow(dead_code)]
#[derive(Clone, Debug, Reflect)]
pub struct ImageData {
    pub grp: [u32; NUM_IMAGE_DATA],
    pub graphics_turns: [u8; NUM_IMAGE_DATA],
    pub clickable: [u8; NUM_IMAGE_DATA],
    pub use_full_iscript: [u8; NUM_IMAGE_DATA],
    pub draw_if_cloaked: [u8; NUM_IMAGE_DATA],
    pub draw_function: [u8; NUM_IMAGE_DATA],
    pub remapping: [u8; NUM_IMAGE_DATA],
    pub iscript: [u32; NUM_IMAGE_DATA],
    pub shield_overlay: [u32; NUM_IMAGE_DATA],
    pub attack_overlay: [u32; NUM_IMAGE_DATA],
    pub damage_overlay: [u32; NUM_IMAGE_DATA],
    pub special_overlay: [u32; NUM_IMAGE_DATA],
    pub landing_dust_overlay: [u32; NUM_IMAGE_DATA],
    pub lift_off_dust_overlay: [u32; NUM_IMAGE_DATA],
}

const NUM_SPRITE_DATA: usize = 517;
const NUM_SELECTABLE_SPRITES: usize = 387;
const EXPECTED_SPRITES_DAT_SIZE: usize = 0xC9C;

#[allow(dead_code)]
#[derive(Clone, Debug, Reflect)]
pub struct SpriteData {
    pub image: [u16; NUM_SPRITE_DATA],
    pub health_bar: [u8; NUM_SELECTABLE_SPRITES],
    pub unknown_0: [u8; NUM_SPRITE_DATA],
    pub visible: [u8; NUM_SPRITE_DATA],
    pub selection_circle: [u8; NUM_SELECTABLE_SPRITES],
    pub selection_circle_offset: [u8; NUM_SELECTABLE_SPRITES],
}

/// How many things (units + buildings + other) are specified in the units.dat file.
const NUM_UNIT_DATA: usize = 228;
/// How many units are specified in the units.dat file (these are at the beginning).
const NUM_UNITS: usize = 106;
/// How many buildings are specified in the units.dat file (these are in the middle).
const NUM_BUILDINGS: usize = 96;
const EXPECTED_UNITS_DAT_SIZE: usize = 0x4DA4;

#[allow(dead_code)]
#[derive(Clone, Debug, Reflect)]
pub struct UnitData {
    pub flingy: [u8; NUM_UNIT_DATA],
    pub sub_unit_1: [u16; NUM_UNIT_DATA],
    pub sub_unit_2: [u16; NUM_UNIT_DATA],
    pub infestation: [u16; NUM_BUILDINGS],
    pub construction_image: [u32; NUM_UNIT_DATA],
    pub unit_direction: [u8; NUM_UNIT_DATA],
    pub shield_enabled: [u8; NUM_UNIT_DATA],
    pub shield_amount: [i16; NUM_UNIT_DATA],
    pub hit_points: [i32; NUM_UNIT_DATA],
    pub elevation_level: [u8; NUM_UNIT_DATA],
    pub unknown_0: [u8; NUM_UNIT_DATA],
    pub sub_label: [u8; NUM_UNIT_DATA],
    pub computer_ai_idle: [u8; NUM_UNIT_DATA],
    pub human_ai_idle: [u8; NUM_UNIT_DATA],
    pub return_to_idle: [u8; NUM_UNIT_DATA],
    pub attack_unit: [u8; NUM_UNIT_DATA],
    pub attack_move: [u8; NUM_UNIT_DATA],
    pub ground_weapon: [u8; NUM_UNIT_DATA],
    pub max_ground_hits: [u8; NUM_UNIT_DATA],
    pub air_weapon: [u8; NUM_UNIT_DATA],
    pub max_air_hits: [u8; NUM_UNIT_DATA],
    pub ai_internal: [u8; NUM_UNIT_DATA],
    pub special_ability_flags: [u32; NUM_UNIT_DATA],
    pub target_acquisition_range: [u8; NUM_UNIT_DATA],
    pub sight_range: [u8; NUM_UNIT_DATA],
    pub armor_upgrade: [u8; NUM_UNIT_DATA],
    pub unit_size: [u8; NUM_UNIT_DATA],
    pub armor: [u8; NUM_UNIT_DATA],
    pub right_click_action: [u8; NUM_UNIT_DATA],
    pub ready_sound: [u16; NUM_UNITS],
    pub what_sound_start: [u16; NUM_UNIT_DATA],
    pub what_sound_end: [u16; NUM_UNIT_DATA],
    pub piss_sound_start: [u16; NUM_UNITS],
    pub piss_sound_end: [u16; NUM_UNITS],
    pub yes_sound_start: [u16; NUM_UNITS],
    pub yes_sound_end: [u16; NUM_UNITS],
    pub placebox_size: [Point16; NUM_UNIT_DATA],
    pub addon_size: [Point16; NUM_BUILDINGS],
    pub unit_rect: [Rect16; NUM_UNIT_DATA],
    pub portrait: [u16; NUM_UNIT_DATA],
    pub mineral_cost: [u16; NUM_UNIT_DATA],
    pub vespene_cost: [u16; NUM_UNIT_DATA],
    pub build_time: [u16; NUM_UNIT_DATA],
    pub requirement_index: [u16; NUM_UNIT_DATA],
    pub star_edit_group_flags: [u8; NUM_UNIT_DATA],
    pub supply_provided: [u8; NUM_UNIT_DATA],
    pub supply_required: [u8; NUM_UNIT_DATA],
    pub space_required: [u8; NUM_UNIT_DATA],
    pub space_provided: [u8; NUM_UNIT_DATA],
    pub build_score: [u16; NUM_UNIT_DATA],
    pub destroy_score: [u16; NUM_UNIT_DATA],
    pub unit_map_string: [u16; NUM_UNIT_DATA],
    pub brood_war_unit_flag: [u8; NUM_UNIT_DATA],
    pub star_edit_availability_flag: [u16; NUM_UNIT_DATA],
}

impl AssetLoader for DatAssetLoader {
    type Asset = DatAsset;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let asset = match load_context
                .path()
                .file_stem()
                .map(|name| name.to_ascii_lowercase().to_string_lossy().to_string())
                .as_deref()
            {
                Some("images") => {
                    let mut bytes = Vec::new();
                    reader.read_to_end(&mut bytes).await?;
                    DatAsset::Images(Box::new(load_images_dat(&bytes)?))
                }
                Some("sprites") => {
                    let mut bytes = Vec::new();
                    reader.read_to_end(&mut bytes).await?;
                    DatAsset::Sprites(Box::new(load_sprites_dat(&bytes)?))
                }
                Some("units") => {
                    let mut bytes = Vec::new();
                    reader.read_to_end(&mut bytes).await?;
                    DatAsset::Units(Box::new(load_units_dat(&bytes)?))
                }
                t => return Err(anyhow!("Unknown dat file type: {t:?}",)),
            };

            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["dat"]
    }
}

fn load_units_dat(mut bytes: &[u8]) -> anyhow::Result<UnitData> {
    // TODO(tec27): At some point in the future we could likely support extended units.dat files
    if bytes.len() < EXPECTED_UNITS_DAT_SIZE {
        return Err(anyhow!("units.dat file is too small: {}", bytes.len()));
    }

    Ok(UnitData {
        flingy: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        sub_unit_1: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        sub_unit_2: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        infestation: bytes.read_u16_array::<NUM_BUILDINGS>()?,
        construction_image: bytes.read_u32_array::<NUM_UNIT_DATA>()?,
        unit_direction: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        shield_enabled: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        shield_amount: bytes.read_i16_array::<NUM_UNIT_DATA>()?,
        hit_points: bytes.read_i32_array::<NUM_UNIT_DATA>()?,
        elevation_level: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        unknown_0: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        sub_label: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        computer_ai_idle: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        human_ai_idle: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        return_to_idle: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        attack_unit: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        attack_move: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        ground_weapon: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        max_ground_hits: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        air_weapon: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        max_air_hits: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        ai_internal: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        special_ability_flags: bytes.read_u32_array::<NUM_UNIT_DATA>()?,
        target_acquisition_range: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        sight_range: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        armor_upgrade: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        unit_size: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        armor: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        right_click_action: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        ready_sound: bytes.read_u16_array::<NUM_UNITS>()?,
        what_sound_start: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        what_sound_end: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        piss_sound_start: bytes.read_u16_array::<NUM_UNITS>()?,
        piss_sound_end: bytes.read_u16_array::<NUM_UNITS>()?,
        yes_sound_start: bytes.read_u16_array::<NUM_UNITS>()?,
        yes_sound_end: bytes.read_u16_array::<NUM_UNITS>()?,
        placebox_size: bytes.read_array::<Point16, NUM_UNIT_DATA>()?,
        addon_size: bytes.read_array::<Point16, NUM_BUILDINGS>()?,
        unit_rect: bytes.read_array::<Rect16, NUM_UNIT_DATA>()?,
        portrait: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        mineral_cost: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        vespene_cost: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        build_time: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        requirement_index: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        star_edit_group_flags: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        supply_provided: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        supply_required: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        space_required: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        space_provided: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        build_score: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        destroy_score: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        unit_map_string: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        brood_war_unit_flag: bytes.read_u8_array::<NUM_UNIT_DATA>()?,
        star_edit_availability_flag: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
    })
}

fn load_images_dat(mut bytes: &[u8]) -> anyhow::Result<ImageData> {
    if bytes.len() < NUM_IMAGE_DATA * IMAGE_DATA_SIZE {
        return Err(anyhow!("images.dat file is too small: {}", bytes.len()));
    }

    Ok(ImageData {
        grp: bytes.read_u32_array::<NUM_IMAGE_DATA>()?,
        graphics_turns: bytes.read_u8_array::<NUM_IMAGE_DATA>()?,
        clickable: bytes.read_u8_array::<NUM_IMAGE_DATA>()?,
        use_full_iscript: bytes.read_u8_array::<NUM_IMAGE_DATA>()?,
        draw_if_cloaked: bytes.read_u8_array::<NUM_IMAGE_DATA>()?,
        draw_function: bytes.read_u8_array::<NUM_IMAGE_DATA>()?,
        remapping: bytes.read_u8_array::<NUM_IMAGE_DATA>()?,
        iscript: bytes.read_u32_array::<NUM_IMAGE_DATA>()?,
        shield_overlay: bytes.read_u32_array::<NUM_IMAGE_DATA>()?,
        attack_overlay: bytes.read_u32_array::<NUM_IMAGE_DATA>()?,
        damage_overlay: bytes.read_u32_array::<NUM_IMAGE_DATA>()?,
        special_overlay: bytes.read_u32_array::<NUM_IMAGE_DATA>()?,
        landing_dust_overlay: bytes.read_u32_array::<NUM_IMAGE_DATA>()?,
        lift_off_dust_overlay: bytes.read_u32_array::<NUM_IMAGE_DATA>()?,
    })
}

fn load_sprites_dat(mut bytes: &[u8]) -> anyhow::Result<SpriteData> {
    if bytes.len() < EXPECTED_SPRITES_DAT_SIZE {
        return Err(anyhow!("sprites.dat file is too small: {}", bytes.len()));
    }

    Ok(SpriteData {
        image: bytes.read_u16_array::<NUM_SPRITE_DATA>()?,
        health_bar: bytes.read_u8_array::<NUM_SELECTABLE_SPRITES>()?,
        unknown_0: bytes.read_u8_array::<NUM_SPRITE_DATA>()?,
        visible: bytes.read_u8_array::<NUM_SPRITE_DATA>()?,
        selection_circle: bytes.read_u8_array::<NUM_SELECTABLE_SPRITES>()?,
        selection_circle_offset: bytes.read_u8_array::<NUM_SELECTABLE_SPRITES>()?,
    })
}
