use anyhow::{bail, Context, Result};
use bevy::asset::{LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::render::texture::{CompressedImageFormats, ImageType};
use bevy::utils::{HashMap, HashSet};
use bitflags::bitflags;
use broodmap::chk::terrain::TerrainTileIds;
use broodmap::chk::tileset::Tileset;
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct TilesetFilename(&'static str);

impl From<Tileset> for TilesetFilename {
    fn from(value: Tileset) -> Self {
        Self(match value {
            Tileset::Badlands => "badlands",
            Tileset::SpacePlatform => "platform",
            Tileset::Installation => "install",
            Tileset::Ashworld => "ashworld",
            Tileset::Jungle => "jungle",
            Tileset::Desert => "desert",
            Tileset::Arctic => "ice",
            Tileset::Twilight => "twilight",
        })
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AssetQuality {
    /// Classic assets.
    Standard,
    /// High definition assets, suitable for resolutions below 4K.
    High,
    /// Extra high definition assets, suitable for 4K and above.
    ExtraHigh,
}

impl AssetQuality {
    fn asset_path(&self) -> &'static str {
        match self {
            Self::Standard => "sd/",
            Self::High => "hd2/",
            Self::ExtraHigh => "",
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AssetPack {
    Standard,
    Carbot,
}

impl AssetPack {
    fn asset_path(&self) -> &'static str {
        match self {
            Self::Standard => "",
            Self::Carbot => "carbot/",
        }
    }
}

#[allow(unused)]
impl TilesetFilename {
    fn cv5_path(&self) -> String {
        format!("tileset/{}.cv5", self.0)
    }

    fn vf4_path(&self) -> String {
        format!("tileset/{}.vf4", self.0)
    }

    fn vr4_path(&self, quality: AssetQuality, pack: AssetPack) -> String {
        format!(
            "{}{}tileset/{}.dds.vr4",
            quality.asset_path(),
            pack.asset_path(),
            self.0
        )
    }

    fn grp_path(&self, quality: AssetQuality, pack: AssetPack) -> String {
        format!(
            "{}{}tileset/{}.dds.grp",
            quality.asset_path(),
            pack.asset_path(),
            self.0
        )
    }

    fn tmsk_path(&self, quality: AssetQuality, pack: AssetPack) -> String {
        format!(
            "{}{}tileset/{}.tmsk",
            quality.asset_path(),
            pack.asset_path(),
            self.0
        )
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MegaTileFlags: u16 {
        const WALKABLE = 0x0001;
        const UNWALKABLE = 0x0004;
        const PROVIDES_COVER = 0x0010;
        const HAS_CREEP = 0x0040;
        const UNBUILDABLE = 0x0080;
        const BLOCKS_VISION = 0x0100;
        /// This tile is higher than low, but lower than high ground.
        const LEVEL_MID = 0x0200;
        /// This tile is higher than mid and low ground.
        const LEVEL_HIGH = 0x0400;
        /// Unbuildable until a building on this tile gets removed.
        const OCCUPIED = 0x0800;
        const RECEDING_CREEP = 0x1000;
        const PARTIALLY_WALKABLE = 0x2000;
        /// Zerg can build here when combined with `HAS_CREEP`.
        const TEMPORARY_CREEP = 0x4000;
        const ALLOW_BEACONS_AND_START_LOCATIONS = 0x8000;
    }
}

impl MegaTileFlags {
    /// Returns the flags that are ignored when loading MegaTileFlags from a CV5 file. These flags
    /// are expected to be set based on the MiniTileFlags of the mini-tiles within each mega-tile.
    pub fn ignored_cv5_flags() -> Self {
        Self::WALKABLE
            | Self::UNWALKABLE
            | Self::BLOCKS_VISION
            | Self::LEVEL_MID
            | Self::LEVEL_HIGH
            | Self::PARTIALLY_WALKABLE
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileGroup {
    /// Flags that apply to all 16 mega tiles in this group, and will apply to tiles of this type
    /// when the game begins. Note that many of these flags will be added/removed based on the
    /// MiniTileFlags of all mini-tiles within the mega-tile, so the actual value for each tile
    /// may differ from what the group says.
    pub flags: MegaTileFlags,
    pub mega_tiles: [u16; 16],
}

fn parse_cv5(input: &[u8]) -> Vec<TileGroup> {
    let input = if input.len() > 0x7FF * 52 {
        warn!("CV5 file is too long, truncating");
        &input[..0x7FF * 52]
    } else {
        input
    };

    input
        .chunks_exact(52)
        .map(|entry_bytes| {
            // The first byte is the tilegroup type, which isn't used by the game
            let mut it = entry_bytes.chunks_exact(2).skip(1);
            let flags = u16::from_le_bytes(it.next().unwrap().try_into().unwrap());
            let mega_tiles = it
                // There are 2 Rect16s that are unused ingame
                .skip(8)
                .take(16)
                .map(|bytes| u16::from_le_bytes(bytes.try_into().unwrap()))
                .collect::<Vec<_>>();

            TileGroup {
                flags: MegaTileFlags::from_bits_truncate(flags),
                mega_tiles: mega_tiles.try_into().unwrap(),
            }
        })
        .collect()
}

async fn load_tile_groups(
    tileset: Tileset,
    load_context: &mut LoadContext<'_>,
) -> Result<Vec<TileGroup>> {
    let filename: TilesetFilename = tileset.into();
    let path = format!("casc-extracted/{}", filename.cv5_path());
    let data = load_context
        .read_asset_bytes(path)
        .await
        .context("Failed to load CV5 file")?;

    let tile_groups = parse_cv5(&data);

    Ok(tile_groups)
}

bitflags! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MiniTileFlags: u16 {
        /// This mini-tile allows units to walk on it/path through it.
        const WALKABLE = 0x0001;
        /// This mini-tile is higher than low, but lower than high ground.
        const LEVEL_MID = 0x0002;
        /// This mini-tile is higher than mid and low ground.
        const LEVEL_HIGH = 0x0004;
        /// This mini-tile blocks the view of tiles around it.
        const BLOCKS_VISION = 0x0008;
        /// This mini-tile is part of a ramp. (Usually appears in the middle of most ramps/stairs)
        const RAMP = 0x0010;
    }
}

async fn load_mini_tile_flags(
    tileset: Tileset,
    load_context: &mut LoadContext<'_>,
) -> Result<Vec<[MiniTileFlags; 16]>> {
    let filename: TilesetFilename = tileset.into();
    let path = format!("casc-extracted/{}", filename.vf4_path());
    let data = load_context
        .read_asset_bytes(path)
        .await
        .context("Failed to load VF4 file")?;

    let data = if data.len() > 0xFFFF * 32 {
        warn!("VF4 file is too long, truncating");
        &data[..0xFFFF * 32]
    } else {
        &data
    };

    Ok(data
        .chunks_exact(32)
        .map(|entry| {
            let mut iter = entry.chunks_exact(2);
            let flags: [MiniTileFlags; 16] = std::array::from_fn(|_| {
                let f = iter.next().unwrap();
                MiniTileFlags::from_bits_truncate(u16::from_le_bytes(f.try_into().unwrap()))
            });
            flags
        })
        .collect())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct MegaTileInfo {
    /// Flags that apply to any tiles of this type when the game begins. Note that there are flags
    /// that do *not* get transferred over, see `MegaTileFlags::ignored_cv5_flags`.
    pub flags: MegaTileFlags,
    /// The ID of the mega-tile. This can be used to look up other related tile assets, such as
    /// the VF4 mini-tile flags (already in this struct), or the texture for this tile from the
    /// VR4 file.
    pub id: u16,
    /// Flags for the mini-tiles that make up this mega-tile. Each mega-tile contains 8x8
    /// mini-tiles, which can have separate walkability flags that are used in pathfinding. With
    /// SC:R assets, mini-tiles are no longer used for rendering, just pathing (contrary to what
    /// documentation about this format may specify).
    pub mini_tiles: [MiniTileFlags; 16],
}

/// Calculate the flags for a mega-tile based on the flags of its mini-tiles and tile group.
fn get_mega_tile_flags(
    mini_tiles: &[MiniTileFlags; 16],
    group_flags: MegaTileFlags,
) -> MegaTileFlags {
    // Count the number of mini-tiles that meet certain criteria, and use that to decide the
    // resulting flags of the mega-tile.
    let mut walkable = 0;
    let mut mid = 0;
    let mut high = 0;
    let mut blocks_vision = 0;
    for t in mini_tiles {
        if t.contains(MiniTileFlags::WALKABLE) {
            walkable += 1;
        }
        if t.contains(MiniTileFlags::LEVEL_MID) {
            mid += 1;
        }
        if t.contains(MiniTileFlags::LEVEL_HIGH) {
            high += 1;
        }
        if t.contains(MiniTileFlags::BLOCKS_VISION) {
            blocks_vision += 1;
        }
    }

    let mut result = if walkable > 12 {
        MegaTileFlags::WALKABLE
    } else {
        MegaTileFlags::UNWALKABLE
    };

    if walkable > 0 && walkable != 16 {
        result |= MegaTileFlags::PARTIALLY_WALKABLE;
    }
    if high < 12 && mid + high >= 12 {
        result |= MegaTileFlags::LEVEL_MID;
    }
    if high >= 12 {
        result |= MegaTileFlags::LEVEL_HIGH;
    }
    if blocks_vision > 0 {
        result |= MegaTileFlags::BLOCKS_VISION;
    }

    result | (group_flags & !MegaTileFlags::ignored_cv5_flags())
}

pub async fn load_mega_tile_lookup(
    tileset: Tileset,
    terrain_mega_tiles: &TerrainTileIds,
    load_context: &mut LoadContext<'_>,
) -> Result<HashMap<u16, MegaTileInfo>> {
    let tile_groups = load_tile_groups(tileset, load_context).await?;
    let mini_tile_flags = load_mini_tile_flags(tileset, load_context).await?;
    let mut result = HashMap::new();

    for id in &terrain_mega_tiles.tiles {
        let entry_id = id.id();
        result.entry(entry_id).or_insert_with(|| {
            tile_groups.get(id.group_id() as usize).map_or(
                // TODO(tec27): Would probably be better not to insert in this case? Unsure what
                // MegaTile 0 is gonna correspond to here
                MegaTileInfo {
                    flags: MegaTileFlags::empty(),
                    id: 0,
                    mini_tiles: [MiniTileFlags::empty(); 16],
                },
                |group| {
                    let mega_tile = group.mega_tiles[id.tile_index() as usize];
                    let mini_tiles = mini_tile_flags
                        .get(mega_tile as usize)
                        .copied()
                        .unwrap_or_default();

                    MegaTileInfo {
                        flags: get_mega_tile_flags(&mini_tiles, group.flags),
                        id: mega_tile,
                        mini_tiles,
                    }
                },
            )
        });
    }

    Ok(result)
}

/// Loads the needed tile textures from the VR4 file for the given tileset/mega-tiles.
///
/// Returns a tuple of a vector of texture handles and a map from mega-tile IDs to indices into the
/// texture vector.
pub async fn load_tile_textures(
    tileset: Tileset,
    mega_tile_lookup: &HashMap<u16, MegaTileInfo>,
    load_context: &mut LoadContext<'_>,
    supported_compressed_formats: CompressedImageFormats,
) -> Result<(Vec<Handle<Image>>, HashMap<u16, usize>)> {
    let filename: TilesetFilename = tileset.into();
    let path = format!(
        "casc-extracted/{}",
        // TODO(tec27): Pass in asset quality + pack
        filename.vr4_path(AssetQuality::High, AssetPack::Standard)
    );
    let data = load_context
        .read_asset_bytes(path)
        .await
        .context("Failed to load VR4 file")?;

    if data.len() < 8 {
        bail!("VR4 file is too short");
    }

    let mega_tile_ids = mega_tile_lookup
        .values()
        .map(|info| info.id)
        .collect::<HashSet<_>>();

    let mut data = &data[4..];
    let frame_count = data.read_u16::<LittleEndian>()?;
    info!("frame count: {}", frame_count);
    let mut textures = Vec::with_capacity(mega_tile_ids.len());
    let mut texture_indices = HashMap::new();

    data = &data[2..];
    for i in 0..frame_count {
        data = &data[8..];
        let size = data.read_u32::<LittleEndian>()?;

        if mega_tile_ids.contains(&i) {
            let image = Image::from_buffer(
                &data[..size as usize],
                ImageType::Extension("dds"),
                supported_compressed_formats,
                true,
            )?;
            let handle = load_context
                .set_labeled_asset(format!("texture{}", i).as_str(), LoadedAsset::new(image));

            texture_indices.insert(i, textures.len());
            textures.push(handle);
        }

        if texture_indices.len() >= mega_tile_ids.len() {
            // We found a texture for every mega-tile, no need to keep reading
            break;
        }

        data = &data[size as usize..];
    }

    Ok((textures, texture_indices))
}
