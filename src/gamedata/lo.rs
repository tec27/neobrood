use std::{
    io::{Cursor, Read, Seek, SeekFrom},
    ops::Index,
};

use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct LoOffset {
    pub x: i8,
    pub y: i8,
}

#[derive(Asset, Clone, Debug, TypePath)]
pub struct LoAsset {
    /// The number of different frames this LO file specifies offsets for.
    pub frame_count: usize,
    /// The number of different offsets each frame in this LO file contains.
    pub offset_count: usize,
    offsets: Vec<LoOffset>,
}

impl LoAsset {
    /// Gets the offset for a given frame and overlay.
    pub fn get(&self, frame_index: usize, offset_index: usize) -> Option<&LoOffset> {
        let index = frame_index * self.offset_count + offset_index;
        self.offsets.get(index)
    }
}

impl Index<usize> for LoAsset {
    type Output = [LoOffset];

    fn index(&self, index: usize) -> &Self::Output {
        let index = index * self.offset_count;
        &self.offsets[index..index + self.offset_count]
    }
}

#[derive(Debug, Default)]
pub struct LoAssetLoader;

impl AssetLoader for LoAssetLoader {
    type Asset = LoAsset;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            load_lo(Cursor::new(&bytes))
        })
    }

    fn extensions(&self) -> &[&str] {
        &[
            "loa", "lob", "lod", "lof", "log", "lol", "loo", "los", "lou", "lox",
        ]
    }
}

fn load_lo<R: Read + Seek + Send>(mut r: R) -> anyhow::Result<LoAsset> {
    let frame_count = r.read_u32::<LittleEndian>()? as usize;
    let offset_count = r.read_u32::<LittleEndian>()? as usize;

    let mut def_offsets = vec![0; frame_count];
    r.read_u32_into::<LittleEndian>(&mut def_offsets)?;

    let mut offsets = vec![LoOffset::default(); frame_count * offset_count];
    for (i, &def) in def_offsets.iter().enumerate() {
        r.seek(SeekFrom::Start(def as u64))?;
        for o in 0..offset_count {
            let x = r.read_i8()?;
            let y = r.read_i8()?;
            offsets[i * offset_count + o] = LoOffset { x, y };
        }
    }

    Ok(LoAsset {
        frame_count,
        offset_count,
        offsets,
    })
}

#[cfg(test)]
mod tests {
    use claims::assert_ok;

    use super::*;

    #[test]
    fn load_normal_lo() {
        let bytes = [
            0x02, 0x00, 0x00, 0x00, // frame count
            0x04, 0x00, 0x00, 0x00, // overlay count
            0x18, 0x00, 0x00, 0x00, // offset of frame 0
            0x10, 0x00, 0x00, 0x00, // offset  of frame 1
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // second frame
            0xfe, 0xfd, 0xfc, 0xfb, 0xfa, 0xf9, 0xf8, 0xf7, // first frame
        ];

        let result = load_lo(Cursor::new(&bytes));
        let result = assert_ok!(result);

        assert_eq!(result.frame_count, 2);
        assert_eq!(result.offset_count, 4);
        assert_eq!(
            result.offsets,
            vec![
                LoOffset { x: -2, y: -3 },
                LoOffset { x: -4, y: -5 },
                LoOffset { x: -6, y: -7 },
                LoOffset { x: -8, y: -9 },
                LoOffset { x: 0x01, y: 0x02 },
                LoOffset { x: 0x03, y: 0x04 },
                LoOffset { x: 0x05, y: 0x06 },
                LoOffset { x: 0x07, y: 0x08 },
            ],
        );

        assert_eq!(result.get(1, 3), Some(&LoOffset { x: 0x07, y: 0x08 }));
    }
}
