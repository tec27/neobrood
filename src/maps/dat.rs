use anyhow::anyhow;
use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext},
    reflect::TypePath,
    utils::BoxedFuture,
};
use byteorder::{LittleEndian, ReadBytesExt};

use crate::bytes::{ByteReadable, ReadByteArraysExt};

// TODO(tec27): Move this somewhere else
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
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
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
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
    Flingy,
    Images,
    Sprites,
    Units(Box<UnitData>),
}

/// How many things (units + buildings + other) are specified in the units.dat file.
const NUM_UNIT_DATA: usize = 228;
/// How many units are specified in the units.dat file (these are at the beginning).
const NUM_UNITS: usize = 106;
/// How many buildings are specified in the units.dat file (these are in the middle).
const NUM_BUILDINGS: usize = 96;
const EXPECTED_UNITS_DAT_SIZE: usize = 0x4DA4;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct UnitData {
    flingy: [u8; NUM_UNIT_DATA],
    sub_unit_1: [u16; NUM_UNIT_DATA],
    sub_unit_2: [u16; NUM_UNIT_DATA],
    infestation: [u16; NUM_BUILDINGS],
    construction_animation: [u32; NUM_UNIT_DATA],
    unit_direction: [u8; NUM_UNIT_DATA],
    shield_enabled: [u8; NUM_UNIT_DATA],
    shield_amount: [i16; NUM_UNIT_DATA],
    hit_points: [i32; NUM_UNIT_DATA],
    elevation_level: [u8; NUM_UNIT_DATA],
    unknown_0: [u8; NUM_UNIT_DATA],
    sub_label: [u8; NUM_UNIT_DATA],
    computer_ai_idle: [u8; NUM_UNIT_DATA],
    human_ai_idle: [u8; NUM_UNIT_DATA],
    return_to_idle: [u8; NUM_UNIT_DATA],
    attack_unit: [u8; NUM_UNIT_DATA],
    attack_move: [u8; NUM_UNIT_DATA],
    ground_weapon: [u8; NUM_UNIT_DATA],
    max_ground_hits: [u8; NUM_UNIT_DATA],
    air_weapon: [u8; NUM_UNIT_DATA],
    max_air_hits: [u8; NUM_UNIT_DATA],
    ai_internal: [u8; NUM_UNIT_DATA],
    special_ability_flags: [u32; NUM_UNIT_DATA],
    target_acquisition_range: [u8; NUM_UNIT_DATA],
    sight_range: [u8; NUM_UNIT_DATA],
    armor_upgrade: [u8; NUM_UNIT_DATA],
    unit_size: [u8; NUM_UNIT_DATA],
    armor: [u8; NUM_UNIT_DATA],
    right_click_action: [u8; NUM_UNIT_DATA],
    ready_sound: [u16; NUM_UNITS],
    what_sound_start: [u16; NUM_UNIT_DATA],
    what_sound_end: [u16; NUM_UNIT_DATA],
    piss_sound_start: [u16; NUM_UNIT_DATA],
    piss_sound_end: [u16; NUM_UNIT_DATA],
    yes_sound_start: [u16; NUM_UNIT_DATA],
    yes_sound_end: [u16; NUM_UNIT_DATA],
    placebox_size: [Point16; NUM_UNIT_DATA],
    addon_size: [Point16; NUM_BUILDINGS],
    unit_rect: [Rect16; NUM_UNIT_DATA],
    portrait: [u16; NUM_UNIT_DATA],
    mineral_cost: [u16; NUM_UNIT_DATA],
    vespene_cost: [u16; NUM_UNIT_DATA],
    build_time: [u16; NUM_UNIT_DATA],
    requirement_index: [u16; NUM_UNIT_DATA],
    star_edit_group_flags: [u8; NUM_UNIT_DATA],
    supply_provided: [u8; NUM_UNIT_DATA],
    supply_required: [u8; NUM_UNIT_DATA],
    space_required: [u8; NUM_UNIT_DATA],
    space_provided: [u8; NUM_UNIT_DATA],
    build_score: [u16; NUM_UNIT_DATA],
    destroy_score: [u16; NUM_UNIT_DATA],
    unit_map_string: [u16; NUM_UNIT_DATA],
    brood_war_unit_flag: [u8; NUM_UNIT_DATA],
    star_edit_availability_flag: [u16; NUM_UNIT_DATA],
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
                Some("flingy") => DatAsset::Flingy,
                Some("images") => DatAsset::Images,
                Some("sprites") => DatAsset::Sprites,
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
        construction_animation: bytes.read_u32_array::<NUM_UNIT_DATA>()?,
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
        piss_sound_start: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        piss_sound_end: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        yes_sound_start: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
        yes_sound_end: bytes.read_u16_array::<NUM_UNIT_DATA>()?,
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
