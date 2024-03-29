use anyhow::{anyhow, bail};
use byteorder::{LittleEndian, ReadBytesExt};
use proc_macro2::{Delimiter, Group, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use std::{env, ops::Range, path::Path};
use syn::Ident;

use crate::bytes::{ByteReadable, ReadByteArraysExt};

mod bytes;

fn main() -> Result<(), anyhow::Error> {
    let mut args = env::args();
    if args.len() < 2 {
        bail!("Usage: gen_rules /path/to/game/data/files");
    }

    let path_arg = args.nth(1).unwrap();
    let game_data_path = Path::new(&path_arg);

    {
        let path = game_data_path.join("arr/images.dat");
        let bytes = std::fs::read(path).expect("Couldn't read images.dat");
        let data = load_images_dat(&bytes)?;
        write_images(data)?;
    }

    {
        let path = game_data_path.join("arr/sprites.dat");
        let bytes = std::fs::read(path).expect("Couldn't read sprites.dat");
        let data = load_sprites_dat(&bytes)?;
        write_sprites(data)?;
    }

    {
        let path = game_data_path.join("arr/flingy.dat");
        let bytes = std::fs::read(path).expect("Couldn't read flingy.dat");
        let data = load_flingy_dat(&bytes)?;
        write_flingy(data)?;
    }

    {
        let path = game_data_path.join("arr/units.dat");
        let bytes = std::fs::read(path).expect("Couldn't read units.dat");
        let data = load_units_dat(&bytes)?;
        write_units(data)?;
    }

    Ok(())
}

/// Wrapper around [Option] that outputs it as `Some(value)` or `None` instead of as just the
/// contained value.
struct PreservedOption<T>(Option<T>);

impl<T: ToTokens> ToTokens for PreservedOption<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.0 {
            Some(inner) => {
                tokens.append(Ident::new("Some", Span::call_site()));
                tokens.append(Group::new(Delimiter::Parenthesis, inner.to_token_stream()));
            }
            None => {
                tokens.append(Ident::new("None", Span::call_site()));
            }
        }
    }
}

/// How many images are specified in the images.dat file.
const NUM_IMAGE_DATA: usize = 999;
/// How much data each image instance takes up in the images.dat file (in bytes).
const IMAGE_DATA_SIZE: usize = 38;

#[derive(Clone, Debug)]
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

fn write_images(data: ImageData) -> anyhow::Result<()> {
    let mut entries = Vec::new();
    for i in 0..NUM_IMAGE_DATA {
        let id = i as u16;
        let grp = data.grp[i];
        let graphics_turns = data.graphics_turns[i];
        let clickable = data.clickable[i];
        let use_full_iscript = data.use_full_iscript[i];
        let draw_if_cloaked = data.draw_if_cloaked[i];
        let draw_function = data.draw_function[i];
        let remapping = data.remapping[i];
        let iscript = data.iscript[i];
        let shield_overlay = data.shield_overlay[i];
        let attack_overlay = data.attack_overlay[i];
        let damage_overlay = data.damage_overlay[i];
        let special_overlay = data.special_overlay[i];
        let landing_dust_overlay = data.landing_dust_overlay[i];
        let lift_off_dust_overlay = data.lift_off_dust_overlay[i];

        entries.push(quote! {
            BwImage {
                id: #id,
                grp: #grp,
                graphics_turns: #graphics_turns,
                clickable: #clickable,
                use_full_iscript: #use_full_iscript,
                draw_if_cloaked: #draw_if_cloaked,
                draw_function: #draw_function,
                remapping: #remapping,
                iscript: #iscript,
                shield_overlay: #shield_overlay,
                attack_overlay: #attack_overlay,
                damage_overlay: #damage_overlay,
                special_overlay: #special_overlay,
                landing_dust_overlay: #landing_dust_overlay,
                lift_off_dust_overlay: #lift_off_dust_overlay,
            }
        });
    }

    let tokens = quote! {
        use crate::gamedata::BwImage;

        /// Contains data for all images in the game.
        pub const IMAGES: [BwImage; #NUM_IMAGE_DATA] = [#(#entries,)*];
    };

    let src = syn::parse2(tokens).expect("Couldn't parse generated image.rs");
    let src = prettyplease::unparse(&src);
    std::fs::write("src/gamedata/generated/image.rs", src)
        .expect("Couldn't write generated/image.rs");

    Ok(())
}

const NUM_SPRITE_DATA: usize = 517;
const NUM_SELECTABLE_SPRITES: usize = 387;
const EXPECTED_SPRITES_DAT_SIZE: usize = 0xC9C;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SpriteData {
    pub image: [u16; NUM_SPRITE_DATA],
    pub health_bar: [u8; NUM_SELECTABLE_SPRITES],
    pub unknown_0: [u8; NUM_SPRITE_DATA],
    pub visible: [u8; NUM_SPRITE_DATA],
    pub selection_circle: [u8; NUM_SELECTABLE_SPRITES],
    pub selection_circle_offset: [u8; NUM_SELECTABLE_SPRITES],
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

fn write_sprites(data: SpriteData) -> anyhow::Result<()> {
    let mut entries = Vec::new();
    let first_selectable = NUM_SPRITE_DATA - NUM_SELECTABLE_SPRITES;
    for i in 0..NUM_SPRITE_DATA {
        let selectable_index = if i >= first_selectable {
            Some(i - first_selectable)
        } else {
            None
        };

        let id = i as u16;
        let image_id = data.image[i];
        let health_bar = PreservedOption(selectable_index.map(|i| data.health_bar[i]));
        let unknown_0 = data.unknown_0[i];
        let visible = data.visible[i];
        let selection_circle = PreservedOption(selectable_index.map(|i| data.selection_circle[i]));
        let selection_circle_offset =
            PreservedOption(selectable_index.map(|i| data.selection_circle_offset[i]));

        entries.push(quote! {
            BwSprite {
                id: #id,
                image_id: #image_id,
                health_bar: #health_bar,
                unknown_0: #unknown_0,
                visible: #visible,
                selection_circle: #selection_circle,
                selection_circle_offset: #selection_circle_offset,
            }
        });
    }

    let tokens = quote! {
        use crate::gamedata::BwSprite;

        /// Contains data for all sprites in the game.
        pub const SPRITES: [BwSprite; #NUM_SPRITE_DATA] = [#(#entries,)*];
    };

    let src = syn::parse2(tokens).expect("Couldn't parse generated sprite.rs");
    let src = prettyplease::unparse(&src);
    std::fs::write("src/gamedata/generated/sprite.rs", src)
        .expect("Couldn't write generated/sprite.rs");

    Ok(())
}

/// How many flingy types are specified in the flingy.dat file.
const NUM_FLINGY_DATA: usize = 209;
/// How much data each flingy instance takes up in the flingy.dat file (in bytes).
const FLINGY_DATA_SIZE: usize = 15;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FlingyData {
    pub sprite: [u16; NUM_FLINGY_DATA],
    pub speed: [u32; NUM_FLINGY_DATA],
    pub acceleration: [u16; NUM_FLINGY_DATA],
    pub halt_distance: [u32; NUM_FLINGY_DATA],
    pub turn_radius: [u8; NUM_FLINGY_DATA],
    pub unused: [u8; NUM_FLINGY_DATA],
    pub movement_control: [u8; NUM_FLINGY_DATA],
}

fn load_flingy_dat(mut bytes: &[u8]) -> anyhow::Result<FlingyData> {
    if bytes.len() < NUM_FLINGY_DATA * FLINGY_DATA_SIZE {
        return Err(anyhow!("flingy.dat file is too small: {}", bytes.len()));
    }

    Ok(FlingyData {
        sprite: bytes.read_u16_array::<NUM_FLINGY_DATA>()?,
        speed: bytes.read_u32_array::<NUM_FLINGY_DATA>()?,
        acceleration: bytes.read_u16_array::<NUM_FLINGY_DATA>()?,
        halt_distance: bytes.read_u32_array::<NUM_FLINGY_DATA>()?,
        turn_radius: bytes.read_u8_array::<NUM_FLINGY_DATA>()?,
        unused: bytes.read_u8_array::<NUM_FLINGY_DATA>()?,
        movement_control: bytes.read_u8_array::<NUM_FLINGY_DATA>()?,
    })
}

fn write_flingy(flingy_data: FlingyData) -> anyhow::Result<()> {
    let mut entries = Vec::new();
    for i in 0..NUM_FLINGY_DATA {
        let id = i as u8;
        let sprite_id = flingy_data.sprite[i];
        let speed = flingy_data.speed[i];
        let acceleration = flingy_data.acceleration[i];
        let halt_distance = flingy_data.halt_distance[i];
        let turn_radius = flingy_data.turn_radius[i];
        let movement_control = flingy_data.movement_control[i];

        entries.push(quote! {
            Flingy {
                id: #id,
                sprite_id: #sprite_id,
                speed: #speed,
                acceleration: #acceleration,
                halt_distance: #halt_distance,
                turn_radius: #turn_radius,
                movement_control: #movement_control,
            }
        });
    }

    let tokens = quote! {
        use crate::gamedata::Flingy;

        /// Contains data for all flingy types in the game.
        pub const FLINGIES: [Flingy; #NUM_FLINGY_DATA] = [#(#entries,)*];
    };

    let src = syn::parse2(tokens).expect("Couldn't parse generated flingy.rs");
    let src = prettyplease::unparse(&src);
    std::fs::write("src/gamedata/generated/flingy.rs", src)
        .expect("Couldn't write generated/flingy.rs");

    Ok(())
}

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

impl ToTokens for Point16 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let x = self.x;
        let y = self.y;
        let code = quote! { I16Vec2 { x: #x, y: #y } };
        code.to_tokens(tokens);
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Bounds16 {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

impl ByteReadable for Bounds16 {
    fn read(reader: &mut impl ReadBytesExt) -> anyhow::Result<Self> {
        Ok(Self {
            left: reader.read_i16::<LittleEndian>()?,
            top: reader.read_i16::<LittleEndian>()?,
            right: reader.read_i16::<LittleEndian>()?,
            bottom: reader.read_i16::<LittleEndian>()?,
        })
    }
}

impl ToTokens for Bounds16 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let left = self.left as i32;
        let top = self.top as i32;
        // NOTE(tec27): BW treats a bounds of (0, 0, 0, 0) as having size (1, 1), which feels fairly
        // odd and they often end up adding 1 to their bounds in various places to account for this
        // oddness. Instead of doing this haphazardly, we add 1 here and can adjust in the opposite
        // direction in the (fewer) places where it's not desired.
        let right = self.right as i32 + 1;
        let bottom = self.bottom as i32 + 1;
        let code = quote! {
            IBounds {
                left: #left,
                top: #top,
                right: #right,
                bottom: #bottom,
            }
        };
        code.to_tokens(tokens);
    }
}

/// How many things (units + buildings + other) are specified in the units.dat file.
const NUM_UNIT_DATA: usize = 228;
/// How many units are specified in the units.dat file (these are at the beginning).
const NUM_UNITS: usize = 106;
/// How many buildings are specified in the units.dat file (these are in the middle).
const NUM_BUILDINGS: usize = 96;
const UNITS_RANGE: Range<usize> = 0..NUM_UNITS;
const BUILDINGS_RANGE: Range<usize> = NUM_UNITS..NUM_UNITS + NUM_BUILDINGS;
const EXPECTED_UNITS_DAT_SIZE: usize = 0x4DA4;

#[allow(dead_code)]
#[derive(Clone, Debug)]
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
    pub bounds: [Bounds16; NUM_UNIT_DATA],
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

fn load_units_dat(mut bytes: &[u8]) -> anyhow::Result<UnitData> {
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
        bounds: bytes.read_array::<Bounds16, NUM_UNIT_DATA>()?,
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

fn write_units(data: UnitData) -> anyhow::Result<()> {
    let mut entries = Vec::new();
    for i in 0..NUM_UNIT_DATA {
        let id = i as u16;
        let flingy_id = data.flingy[i];
        let sub_unit_1 = data.sub_unit_1[i];
        let sub_unit_2 = data.sub_unit_2[i];
        let construction_image_id = data.construction_image[i];
        let unit_direction = data.unit_direction[i];
        let shield_enabled = data.shield_enabled[i];
        let shield_amount = data.shield_amount[i];
        let hit_points = data.hit_points[i];
        let elevation_level = data.elevation_level[i];
        let unknown_0 = data.unknown_0[i];
        let sub_label = data.sub_label[i];
        let computer_ai_idle = data.computer_ai_idle[i];
        let human_ai_idle = data.human_ai_idle[i];
        let return_to_idle = data.return_to_idle[i];
        let attack_unit = data.attack_unit[i];
        let attack_move = data.attack_move[i];
        let ground_weapon = data.ground_weapon[i];
        let max_ground_hits = data.max_ground_hits[i];
        let air_weapon = data.air_weapon[i];
        let max_air_hits = data.max_air_hits[i];
        let ai_internal = data.ai_internal[i];
        let special_ability_flags = data.special_ability_flags[i];
        let target_acquisition_range = data.target_acquisition_range[i];
        let sight_range = data.sight_range[i];
        let armor_upgrade = data.armor_upgrade[i];
        let unit_size = data.unit_size[i];
        let armor = data.armor[i];
        let right_click_action = data.right_click_action[i];
        let what_sound_start = data.what_sound_start[i];
        let what_sound_end = data.what_sound_end[i];
        let placebox_size = data.placebox_size[i];
        let bounds = data.bounds[i];
        let portrait = data.portrait[i];
        let mineral_cost = data.mineral_cost[i];
        let vespene_cost = data.vespene_cost[i];
        let build_time = data.build_time[i];
        let requirement_index = data.requirement_index[i];
        let star_edit_group_flags = data.star_edit_group_flags[i];
        let supply_provided = data.supply_provided[i];
        let supply_required = data.supply_required[i];
        let space_required = data.space_required[i];
        let space_provided = data.space_provided[i];
        let build_score = data.build_score[i];
        let destroy_score = data.destroy_score[i];
        let unit_map_string = data.unit_map_string[i];
        let is_brood_war = data.brood_war_unit_flag[i] != 0;
        let star_edit_availability_flag = data.star_edit_availability_flag[i];

        let kind = match (UNITS_RANGE.contains(&i), BUILDINGS_RANGE.contains(&i)) {
            (true, false) => {
                let i = i - UNITS_RANGE.start;
                let ready_sound = data.ready_sound[i];
                let piss_sound_start = data.piss_sound_start[i];
                let piss_sound_end = data.piss_sound_end[i];
                let yes_sound_start = data.yes_sound_start[i];
                let yes_sound_end = data.yes_sound_end[i];
                quote! {
                    ConstructKind::Unit(UnitData {
                        ready_sound: #ready_sound,
                        piss_sound_start: #piss_sound_start,
                        piss_sound_end: #piss_sound_end,
                        yes_sound_start: #yes_sound_start,
                        yes_sound_end: #yes_sound_end,
                    })
                }
            }
            (false, true) => {
                let i = i - BUILDINGS_RANGE.start;
                let infestation = data.infestation[i];
                let addon_size = data.addon_size[i];
                quote! {
                    ConstructKind::Building(BuildingData {
                        infestation: #infestation,
                        addon_size: #addon_size,
                    })
                }
            }
            _ => quote! { ConstructKind::Other },
        };

        entries.push(quote! {
            Construct {
                id: #id,
                flingy_id: #flingy_id,
                sub_unit_1: #sub_unit_1,
                sub_unit_2: #sub_unit_2,
                construction_image_id: #construction_image_id,
                unit_direction: #unit_direction,
                shield_enabled: #shield_enabled,
                shield_amount: #shield_amount,
                hit_points: #hit_points,
                elevation_level: #elevation_level,
                unknown_0: #unknown_0,
                sub_label: #sub_label,
                computer_ai_idle: #computer_ai_idle,
                human_ai_idle: #human_ai_idle,
                return_to_idle: #return_to_idle,
                attack_unit: #attack_unit,
                attack_move: #attack_move,
                ground_weapon: #ground_weapon,
                max_ground_hits: #max_ground_hits,
                air_weapon: #air_weapon,
                max_air_hits: #max_air_hits,
                ai_internal: #ai_internal,
                special_ability_flags: #special_ability_flags,
                target_acquisition_range: #target_acquisition_range,
                sight_range: #sight_range,
                armor_upgrade: #armor_upgrade,
                unit_size: #unit_size,
                armor: #armor,
                right_click_action: #right_click_action,
                what_sound_start: #what_sound_start,
                what_sound_end: #what_sound_end,
                placebox_size: #placebox_size,
                bounds: #bounds,
                portrait: #portrait,
                mineral_cost: #mineral_cost,
                vespene_cost: #vespene_cost,
                build_time: #build_time,
                requirement_index: #requirement_index,
                star_edit_group_flags: #star_edit_group_flags,
                supply_provided: #supply_provided,
                supply_required: #supply_required,
                space_required: #space_required,
                space_provided: #space_provided,
                build_score: #build_score,
                destroy_score: #destroy_score,
                unit_map_string: #unit_map_string,
                is_brood_war: #is_brood_war,
                star_edit_availability_flag: #star_edit_availability_flag,

                kind: #kind,
            }
        });
    }

    let num_entries = entries.len();

    let tokens = quote! {
        use crate::gamedata::{BuildingData, Construct, ConstructKind, UnitData};
        use crate::math::bounds::IBounds;
        use bevy::math::I16Vec2;

        /// Contains data for all units, buildings, and other constructs in the game.
        pub const CONSTRUCTS: [Construct; #num_entries] = [#(#entries,)*];
    };

    let src = syn::parse2(tokens).expect("Couldn't parse generated unit.rs");
    let src = prettyplease::unparse(&src);
    std::fs::write("src/gamedata/generated/unit.rs", src)
        .expect("Couldn't write generated/unit.rs");

    Ok(())
}
