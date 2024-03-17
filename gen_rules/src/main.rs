use anyhow::{anyhow, bail};
use byteorder::{LittleEndian, ReadBytesExt};
use quote::quote;
use std::{env, path::Path, thread::AccessError};

use crate::bytes::{ByteReadable, ReadByteArraysExt};

mod bytes;

fn main() -> Result<(), anyhow::Error> {
    let args = env::args();
    if args.len() < 2 {
        bail!("Usage: gen_rules /path/to/game/data/files");
    }

    let path_arg = args.skip(1).next().unwrap();
    let game_data_path = Path::new(&path_arg);

    let flingy_path = game_data_path.join("arr/flingy.dat");
    let bytes = std::fs::read(&flingy_path).expect("Couldn't read flingy.dat");
    let flingy_data = load_flingy_dat(&bytes)?;
    write_flingy(flingy_data)?;

    Ok(())
}

fn write_flingy(flingy_data: FlingyData) -> anyhow::Result<()> {
    let mut entries = Vec::new();
    for i in 0..NUM_FLINGY_DATA {
        let sprite = flingy_data.sprite[i];
        let speed = flingy_data.speed[i];
        let acceleration = flingy_data.acceleration[i];
        let halt_distance = flingy_data.halt_distance[i];
        let turn_radius = flingy_data.turn_radius[i];
        let movement_control = flingy_data.movement_control[i];

        entries.push(quote! {
            Flingy {
                sprite: #sprite,
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

        pub const FLINGIES: &'static [Flingy] = &[#(#entries,)*];
    };

    let src = syn::parse2(tokens).expect("Couldn't parse generated flingy.rs");
    let src = prettyplease::unparse(&src);
    std::fs::write("src/gamedata/generated/flingy.rs", src)
        .expect("Couldn't write generated/flingy.rs");

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
