use anyhow::{anyhow, bail};
use byteorder::{LittleEndian, ReadBytesExt};
use proc_macro2::{Delimiter, Group, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use std::{
    arch::is_aarch64_feature_detected, collections::binary_heap::Drain, env, io::IoSlice,
    path::Path,
};
use syn::Ident;

use crate::bytes::{ByteReadable, ReadByteArraysExt};

mod bytes;

fn main() -> Result<(), anyhow::Error> {
    let args = env::args();
    if args.len() < 2 {
        bail!("Usage: gen_rules /path/to/game/data/files");
    }

    let path_arg = args.skip(1).next().unwrap();
    let game_data_path = Path::new(&path_arg);

    {
        let path = game_data_path.join("arr/images.dat");
        let bytes = std::fs::read(&path).expect("Couldn't read images.dat");
        let data = load_images_dat(&bytes)?;
        write_images(data)?;
    }

    {
        let path = game_data_path.join("arr/sprites.dat");
        let bytes = std::fs::read(&path).expect("Couldn't read sprites.dat");
        let data = load_sprites_dat(&bytes)?;
        write_sprites(data)?;
    }

    {
        let path = game_data_path.join("arr/flingy.dat");
        let bytes = std::fs::read(&path).expect("Couldn't read flingy.dat");
        let data = load_flingy_dat(&bytes)?;
        write_flingy(data)?;
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
        // GENERATED CODE, DO NOT MODIFY BY HAND
        use crate::gamedata::BwImage;

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
        let image = data.image[i] as usize;
        let health_bar = PreservedOption(selectable_index.map(|i| data.health_bar[i]));
        let unknown_0 = data.unknown_0[i];
        let visible = data.visible[i];
        let selection_circle = PreservedOption(selectable_index.map(|i| data.selection_circle[i]));
        let selection_circle_offset =
            PreservedOption(selectable_index.map(|i| data.selection_circle_offset[i]));

        entries.push(quote! {
            BwSprite {
                id: #id,
                image: &IMAGES[#image],
                health_bar: #health_bar,
                unknown_0: #unknown_0,
                visible: #visible,
                selection_circle: #selection_circle,
                selection_circle_offset: #selection_circle_offset,
            }
        });
    }

    let tokens = quote! {
        // GENERATED CODE, DO NOT MODIFY BY HAND
        use crate::gamedata::BwSprite;
        use super::image::IMAGES;

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
        let sprite = flingy_data.sprite[i] as usize;
        let speed = flingy_data.speed[i];
        let acceleration = flingy_data.acceleration[i];
        let halt_distance = flingy_data.halt_distance[i];
        let turn_radius = flingy_data.turn_radius[i];
        let movement_control = flingy_data.movement_control[i];

        entries.push(quote! {
            Flingy {
                id: #id,
                sprite: &SPRITES[#sprite],
                speed: #speed,
                acceleration: #acceleration,
                halt_distance: #halt_distance,
                turn_radius: #turn_radius,
                movement_control: #movement_control,
            }
        });
    }

    let tokens = quote! {
        // GENERATED CODE, DO NOT MODIFY BY HAND
        use crate::gamedata::Flingy;
        use super::sprite::SPRITES;

        pub const FLINGIES: [Flingy; #NUM_FLINGY_DATA] = [#(#entries,)*];
    };

    let src = syn::parse2(tokens).expect("Couldn't parse generated flingy.rs");
    let src = prettyplease::unparse(&src);
    std::fs::write("src/gamedata/generated/flingy.rs", src)
        .expect("Couldn't write generated/flingy.rs");

    Ok(())
}
