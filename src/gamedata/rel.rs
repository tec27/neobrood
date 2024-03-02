use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, LoadContext},
    reflect::TypePath,
    utils::BoxedFuture,
};

#[derive(Debug, Default)]
pub struct RelAssetLoader {}

impl AssetLoader for RelAssetLoader {
    type Asset = RelAsset;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let entries = bytes
                .chunks_exact(8)
                .map(|chunk| {
                    let rel_type = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    let ref_image = u32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);

                    RelEntry {
                        rel_type,
                        ref_image: if ref_image == 0xFFFFFFFF {
                            None
                        } else {
                            Some(ref_image)
                        },
                    }
                })
                .collect();

            Ok(RelAsset { entries })
        })
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RelEntry {
    // TODO(tec27): Use bitflags for these values?
    pub rel_type: u32,
    pub ref_image: Option<u32>,
}

impl RelEntry {
    pub fn is_image_reference(&self) -> bool {
        self.rel_type & 0x200 != 0
    }
}

#[derive(Asset, Debug, TypePath, Clone)]
pub struct RelAsset {
    pub entries: Vec<RelEntry>,
}
