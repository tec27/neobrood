use std::ops::Index;

use anyhow::{bail, Context};
use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};

/// A bevy [AssetLoader] for TBL files.
#[derive(Debug, Default)]
pub struct TblAssetLoader {}

#[derive(Asset, Clone, Debug, Reflect)]
pub struct TblAsset {
    entries: Vec<String>,
}

#[allow(dead_code)]
impl TblAsset {
    pub fn get(&self, index: usize) -> Option<&str> {
        self.entries.get(index).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Index<usize> for TblAsset {
    type Output = str;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl AssetLoader for TblAssetLoader {
    type Asset = TblAsset;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let entries = load_tbl(&bytes)?;
        Ok(TblAsset { entries })
    }

    fn extensions(&self) -> &[&str] {
        &["tbl"]
    }
}

fn load_tbl(bytes: &[u8]) -> anyhow::Result<Vec<String>> {
    // A TBL file consists of:
    // - A u16 length stating the number of strings it contains
    // - A list of `length` u16 offsets into the string data (relative to the beginning of the
    //   file), one for each string
    // - null-terminated UTF-8 string data (as of SC:R, anyway)

    if bytes.len() < 2 {
        bail!("Malformed TBL file, does not contain a valid length");
    }

    let length = u16::from_le_bytes(bytes[0..2].try_into().unwrap()) as usize;

    let offsets = bytes
        .chunks_exact(2)
        .skip(1)
        .take(length)
        .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()) as usize);
    if offsets.len() != length {
        bail!(
            "Malformed TBL file, expected {} offsets but found {}",
            length,
            offsets.len()
        );
    }

    offsets
        .map(|o| {
            if o >= bytes.len() {
                bail!("Malformed TBL file, offset {o:#0x} is out of bounds");
            }

            let bytes = &bytes[o..];
            std::str::from_utf8(if let Some(end) = memchr::memchr(b'\0', bytes) {
                &bytes[..end]
            } else {
                bytes
            })
            .map(|s| s.to_string())
            .context("Malformed TBL file, invalid UTF-8 string")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};

    use super::*;

    #[test]
    fn load_normal_tbl() {
        let bytes = [
            0x02, 0x00, // length
            0x06, 0x00, // offset 1
            0x0a, 0x00, // offset 2
            b'a', b'b', b'c', 0x00, // string 1
            b'd', b'e', b'f', 0x00, // string 2
        ];

        let result = load_tbl(&bytes);
        let result = assert_ok!(result);
        assert_eq!(result, vec!["abc".to_string(), "def".to_string()]);
    }

    #[test]
    fn load_out_of_order_tbl() {
        let bytes = [
            0x02, 0x00, // length
            0x0a, 0x00, // offset 1
            0x06, 0x00, // offset 2
            b'd', b'e', b'f', 0x00, // string 1
            b'a', b'b', b'c', 0x00, // string 2
        ];

        let result = load_tbl(&bytes);
        let result = assert_ok!(result);
        assert_eq!(result, vec!["abc".to_string(), "def".to_string()]);
    }

    #[test]
    fn load_overlapping_tbl() {
        let bytes = [
            0x02, 0x00, // length
            0x06, 0x00, // offset 1
            0x09, 0x00, // offset 2
            b'a', b'b', b'c', // string 1
            b'd', b'e', b'f', 0x00, // string 2
        ];

        let result = load_tbl(&bytes);
        let result = assert_ok!(result);
        assert_eq!(result, vec!["abcdef".to_string(), "def".to_string()]);
    }

    #[test]
    fn invalid_length() {
        let bytes = [0x01, 0x00];
        let result = load_tbl(&bytes);
        assert_err!(result);
    }

    #[test]
    fn invalid_offset() {
        let bytes = [
            0x02, 0x00, // length
            0x06, 0x00, // offset 1
            0x0a, 0x00, // offset 2
            b'a', b'b', b'c', 0x00, // string 1
        ];

        let result = load_tbl(&bytes);
        assert_err!(result);
    }

    #[test]
    fn zero_length_string() {
        let bytes = [
            0x02, 0x00, // length
            0x06, 0x00, // offset 1
            0x09, 0x00, // offset 2
            b'a', b'b', b'c', 0x00, // string 1
            0x00, // string 2
        ];

        let result = load_tbl(&bytes);
        let result = assert_ok!(result);
        assert_eq!(result, vec!["abc".to_string(), "".to_string()]);
    }
}
