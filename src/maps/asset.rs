use anyhow::anyhow;
use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::CompressedImageFormats;
use bevy::utils::HashMap;
use broodmap::chk::placed_units::PlacedUnit;
use broodmap::chk::terrain::TerrainTileIds;
use broodmap::chk::tileset::Tileset;
use serde::{Deserialize, Serialize};

use crate::maps::tileset::{load_mega_tile_lookup, load_tile_textures, MegaTileInfo};
use crate::settings::{AssetPack, AssetQuality};

/// A bevy [AssetLoader] for SCM and SCX files.
#[derive(Debug)]
pub struct MapAssetLoader {
    supported_compressed_formats: CompressedImageFormats,
}

impl FromWorld for MapAssetLoader {
    fn from_world(world: &mut World) -> Self {
        let supported_compressed_formats = match world.get_resource::<RenderDevice>() {
            Some(render_device) => CompressedImageFormats::from_features(render_device.features()),
            None => CompressedImageFormats::all(),
        };

        Self {
            supported_compressed_formats,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct MapAssetSettings {
    pub quality: AssetQuality,
    pub pack: AssetPack,
}

#[derive(Asset, Debug, TypePath)]
pub struct MapAsset {
    /// The name of the map.
    pub name: String,
    /// Width of the map in tiles.
    pub width: u32,
    /// Height of the map in tiles.
    pub height: u32,
    /// The map's tileset.
    pub tileset: Tileset,
    /// The map's terrain, as a 2D vector of tile IDs (can be converted to mega-tiles via
    /// `mega_tile_lookup`).
    pub terrain: TerrainTileIds,
    /// A hashmap of tile IDs to their mega-tile info.
    pub mega_tile_lookup: HashMap<u16, MegaTileInfo>,
    /// A Vec of handles to textures for each mega-tile.
    pub tile_textures: Vec<Handle<Image>>,
    /// A map of mega-tile IDs -> an index into `tile_textures`.
    pub tile_texture_indices: HashMap<u16, usize>,
    /// Units that were placed on the map during editing.
    pub placed_units: Vec<PlacedUnit>,
    /// Sprites that were placed on the map during editing.
    pub sprites: Vec<broodmap::chk::sprites::Sprite>,
}

impl AssetLoader for MapAssetLoader {
    type Asset = MapAsset;
    type Settings = MapAssetSettings;
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            // TODO(tec27): At some point we'll need the MPQ to be able to load other assets
            // (for UMS), but I don't want to deal with the lifetimes for now, so we just drop it
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let (chk, _mpq) = broodmap::extract_chk_from_map(&bytes, None, None)?;
            let tileset = chk.tileset();
            let Ok(terrain) = chk.terrain() else {
                return Err(anyhow!("Could not load map's terrain"));
            };

            let Ok(placed_units) = chk.placed_units() else {
                return Err(anyhow!("Could not load map's placed units"));
            };
            let Ok(sprites) = chk.sprites() else {
                return Err(anyhow!("Could not load map's sprites"));
            };

            // TODO(tec27): Use load_context.labeled_asset_scope for these so that they get properly
            // marked as dependencies (and probably load the files out of the casc via a loader
            // call instead of doing it directly in these functions?)
            info!("Loading mega tile lookup...");
            let mega_tile_lookup = load_mega_tile_lookup(tileset, terrain, load_context).await?;
            info!("Mega tile lookup has {} entries", mega_tile_lookup.len());

            info!("Loading tileset textures...");
            let (tile_textures, tile_texture_indices) = load_tile_textures(
                tileset,
                &mega_tile_lookup,
                settings.quality,
                settings.pack,
                load_context,
                self.supported_compressed_formats,
            )
            .await?;
            info!("Loaded {} tile textures", tile_textures.len());

            Ok(MapAsset {
                name: chk
                    .scenario_props()
                    .ok()
                    .map(|p| p.name.clone())
                    .flatten()
                    .unwrap_or_default(),
                width: chk.width() as u32,
                height: chk.height() as u32,
                tileset,
                terrain: terrain.clone(),
                mega_tile_lookup,
                tile_textures,
                tile_texture_indices,
                placed_units: placed_units.clone(),
                sprites: sprites.clone(),
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["scx", "scm"]
    }
}
