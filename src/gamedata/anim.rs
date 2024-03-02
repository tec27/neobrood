// Useful links for .anim stuff:
// http://www.staredit.net/topic/17217/0/
// http://www.staredit.net/topic/17590/
// https://github.com/neivv/animosity/blob/master/src/anim.rs
// https://github.com/alexpineda/titan-reactor/blob/974f3b30a5a19c14dfcf03f85c82f869342ac76d/src/renderer/image/formats/parse-anim.ts

use std::io::{Cursor, Read, Seek, SeekFrom};

use bevy::{
    asset::{io::Reader, Asset, AssetLoader, AsyncReadExt, Handle, LoadContext},
    log::{error, warn},
    math::{Rect, Vec2},
    reflect::TypePath,
    render::{render_asset::RenderAssetUsages, texture::Image},
    sprite::{Anchor, TextureAtlasLayout},
    utils::hashbrown::HashMap,
};
use byteorder::{LittleEndian, ReadBytesExt};
use image::io::Reader as ImageReader;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnimError {
    #[error("anim header is invalid")]
    InvalidHeader,
    #[error("unknown anim type: {0}")]
    UnknownType(u8),
    #[error("SD anims are not yet supported")]
    SdAnimsUnsupported,
    #[error("HD anims with multiple entries are not supported")]
    MultipleHdEntries,
    #[error("image {0} references entry {1} which is out of bounds")]
    OutOfBoundsReference(u16, u16),
    #[error("inline sprite references are not yet supported")]
    InlineSpriteReferencesUnsupported,
    #[error("failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to decode image: {0}")]
    Image(#[from] image::ImageError),
}

#[derive(Debug, Default)]
pub struct AnimAssetLoader {}

impl AssetLoader for AnimAssetLoader {
    type Asset = AnimAsset;
    type Settings = ();
    type Error = AnimError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let file = load_anim(Cursor::new(&bytes))?;
            let AnimFile {
                sprite:
                    AnimFileSprite {
                        frames,
                        layers,
                        width,
                        height,
                    },
                scale,
            } = file;

            let mut layer_handles = HashMap::with_capacity(layers.len());
            for (name, texture) in layers.iter() {
                let texture_cursor = Cursor::new(
                    &bytes[texture.offset as usize..(texture.offset + texture.size) as usize],
                );
                let dyn_image = ImageReader::new(texture_cursor)
                    .with_guessed_format()?
                    .decode()?;
                let handle = load_context.labeled_asset_scope(format!("layer_{name}"), |_| {
                    Image::from_dynamic(dyn_image, true, RenderAssetUsages::RENDER_WORLD)
                });
                layer_handles.insert(name.clone(), handle);
            }

            let mut layout = TextureAtlasLayout::new_empty(Vec2::new(width as f32, height as f32));
            let mut offsets = Vec::with_capacity(frames.len());
            let scale = FRAME_SCALE as f32 / scale as f32;
            let use_offsets = if width == 0 && height == 0 {
                // TODO(tec27): We should use the GRP sizes instead in this case
                warn!(
                    "anim at {:?} has no specified size, not using offsets",
                    load_context.path()
                );
                false
            } else {
                true
            };

            let width = width as f32 / scale;
            let height = height as f32 / scale;
            for frame in frames {
                let rect = Rect::new(
                    frame.texture_x as f32 / scale,
                    frame.texture_y as f32 / scale,
                    (frame.texture_x + frame.width) as f32 / scale,
                    (frame.texture_y + frame.height) as f32 / scale,
                );
                // Convert offsets into Bevy Anchors. Offsets are expressed as pixel values relative
                // to the top-left corner of the render position, whereas Bevy anchors are expressed
                // relative to the center (0,0), and with +Y being up instead of down.
                if use_offsets {
                    let offset_x = frame.offset_x as f32 / scale;
                    let offset_y = frame.offset_y as f32 / scale;
                    let anchor_point = Vec2::new(width / 2.0 - offset_x, height / 2.0 - offset_y);
                    offsets.push(Anchor::Custom(Vec2::new(
                        anchor_point.x / rect.width() - 0.5,
                        0.5 - anchor_point.y / rect.height(),
                    )));
                }
                layout.add_texture(rect);
            }

            let layout = load_context.labeled_asset_scope("layout".to_string(), |_| layout);

            Ok(AnimAsset {
                size: Vec2::new(width, height),
                layout,
                offsets,
                layers: layer_handles,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["anim"]
    }
}

#[derive(Asset, Debug, TypePath)]
pub struct AnimAsset {
    pub size: Vec2,
    pub layout: Handle<TextureAtlasLayout>,
    /// How much to offset the frame from the sprite's origin during rendering (from the top left?).
    /// These are expressed as a fraction of the total size of the sprite.
    pub offsets: Vec<Anchor>,
    pub layers: HashMap<String, Handle<Image>>,
}

#[derive(Debug, Clone)]
pub struct AnimFile {
    sprite: AnimFileSprite,
    scale: u8,
}

#[derive(Debug, Clone)]
pub struct AnimFileSprite {
    width: u16,
    height: u16,
    frames: Vec<AnimFrame>,
    layers: HashMap<String, AnimTexture>,
}

#[derive(Debug, Copy, Clone)]
pub struct AnimFrame {
    /// X position of this frame's area in the layer texture
    pub texture_x: u16,
    /// Y position of this frame's area in the layer texture
    pub texture_y: u16,
    /// How much to offset the frame from the sprite's origin during rendering in the X direction
    pub offset_x: i16,
    /// How much to offset the frame from the sprite's origin during rendering in the Y direction
    pub offset_y: i16,
    /// Width of the frame's area in the layer texture
    pub width: u16,
    /// Height of the frame's area in the layer texture
    pub height: u16,
    pub unknown: u32,
}

#[derive(Debug, Copy, Clone)]
struct AnimTexture {
    offset: u32,
    size: u32,
    width: u16,
    height: u16,
}

const ANIM_MAGIC: u32 = 0x4d494e41; // "ANIM"
const TYPE_SD: u8 = 0x01;
const TYPE_HD: u8 = 0x02;

/// The scale that frames are specified in. (HD frames are still specified in 4K coordinates).
const FRAME_SCALE: u16 = 4;

fn load_anim<R: Read + Seek + Send>(mut r: R) -> Result<AnimFile, AnimError> {
    let magic = r.read_u32::<LittleEndian>()?;
    if magic != ANIM_MAGIC {
        return Err(AnimError::InvalidHeader);
    }

    let scale = r.read_u8()?;
    let ty = r.read_u8()?;

    if ty != TYPE_SD && ty != TYPE_HD {
        return Err(AnimError::UnknownType(ty));
    }

    if ty == TYPE_SD {
        return Err(AnimError::SdAnimsUnsupported);
    }

    let _unknown = r.read_u16::<LittleEndian>()?;
    let num_layers = r.read_u16::<LittleEndian>()?;
    let num_entries = r.read_u16::<LittleEndian>()?;

    if ty != TYPE_SD && num_entries != 1 {
        return Err(AnimError::MultipleHdEntries);
    }

    let mut layer_names = Vec::with_capacity(num_layers as usize);
    for i in 0..num_layers {
        if i < 10 {
            let mut buf = [0u8; 0x20];
            std::io::Read::read_exact(&mut r, &mut buf)?;
            layer_names.push(
                String::from_utf8_lossy(if let Some(end) = memchr::memchr(b'\0', &buf) {
                    &buf[..end]
                } else {
                    &buf
                })
                .into_owned(),
            );
        } else {
            layer_names.push(format!("Layer{i}"));
        }
    }

    if ty == TYPE_SD {
        r.seek(SeekFrom::Start(0x14c + 999 * 4))?;
    } else {
        r.seek(SeekFrom::Start(0x14c))?;
    }

    let frame_count = r.read_u16::<LittleEndian>()?;
    let ref_id = r.read_u16::<LittleEndian>()?;
    let width = r.read_u16::<LittleEndian>()?;
    let height = r.read_u16::<LittleEndian>()?;
    let frame_arr_offset = r.read_u32::<LittleEndian>()?;

    if ref_id != u16::MAX {
        if ref_id > num_entries {
            return Err(AnimError::OutOfBoundsReference(0, ref_id));
        }

        // TODO(tec27): Support this for SD anims
        Err(AnimError::InlineSpriteReferencesUnsupported)
    } else {
        let layers = read_layer_textures(&mut r, &layer_names)?;
        r.seek(SeekFrom::Start(frame_arr_offset as u64))?;
        let frames = read_frames(&mut r, frame_count as usize)?;

        Ok(AnimFile {
            sprite: AnimFileSprite {
                width,
                height,
                frames,
                layers,
            },
            scale,
        })
    }
}

fn read_layer_textures<R: Read>(
    mut r: R,
    layer_names: &[String],
) -> Result<HashMap<String, AnimTexture>, AnimError> {
    let mut result = HashMap::with_capacity(layer_names.len());
    for layer_name in layer_names.iter() {
        let texture = AnimTexture {
            offset: r.read_u32::<LittleEndian>()?,
            size: r.read_u32::<LittleEndian>()?,
            width: r.read_u16::<LittleEndian>()?,
            height: r.read_u16::<LittleEndian>()?,
        };
        if texture.offset != 0 {
            result.insert(layer_name.clone(), texture);
        }
    }
    Ok(result)
}

fn read_frames<R: Read>(mut r: R, count: usize) -> Result<Vec<AnimFrame>, AnimError> {
    (0..count)
        .map(|_| {
            Ok(AnimFrame {
                texture_x: r.read_u16::<LittleEndian>()?,
                texture_y: r.read_u16::<LittleEndian>()?,
                offset_x: r.read_i16::<LittleEndian>()?,
                offset_y: r.read_i16::<LittleEndian>()?,
                width: r.read_u16::<LittleEndian>()?,
                height: r.read_u16::<LittleEndian>()?,
                unknown: r.read_u32::<LittleEndian>()?,
            })
        })
        .collect()
}
