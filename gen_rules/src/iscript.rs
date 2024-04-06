use anyhow::{anyhow, Context};
use num_enum::TryFromPrimitive;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::{
    collections::HashMap,
    io::{Cursor, Read, Seek, SeekFrom},
    num::NonZeroU16,
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::PreservedOption;

const ISCRIPT_MAGIC: u32 = u32::from_le_bytes(*b"SCPE");
pub struct IscriptBinData {
    pub iscripts: HashMap<u16, Vec<u16>>,
    pub anim_blocks: HashMap<u16, Vec<IscriptCommand>>,
    pub anim_ref_count: HashMap<u16, u32>,
}

pub fn load_iscript_bin(bytes: &[u8]) -> anyhow::Result<IscriptBinData> {
    let mut iscripts = HashMap::new();
    let mut anim_blocks = HashMap::new();
    let mut anim_ref_count = HashMap::new();

    let mut next_offset = u16::from_le_bytes([bytes[0], bytes[1]]) as usize;
    while next_offset + 4 < bytes.len() {
        let mut cursor = Cursor::new(&bytes[next_offset..]);
        let index = cursor.read_u16::<LittleEndian>()?;
        if iscripts.contains_key(&index) {
            return Err(anyhow!("Duplicate iscript index: {}", index));
        }

        let offset = cursor.read_u16::<LittleEndian>()?;
        if index == u16::MAX && offset == 0 {
            break;
        }

        let mut cursor = Cursor::new(&bytes[offset as usize..]);
        let magic = cursor.read_u32::<LittleEndian>()?;
        if magic != ISCRIPT_MAGIC {
            return Err(anyhow!("Invalid iscript magic: 0x{:08X}", magic));
        }

        let ty = cursor.read_u8()?;
        cursor.seek(SeekFrom::Current(3))?;
        let anim_len = type_to_animation_block_len(ty)?;
        let anim_offsets = (0..anim_len)
            .map(|_| cursor.read_u16::<LittleEndian>())
            .collect::<Result<Vec<_>, _>>()?;

        iscripts.insert(index, anim_offsets.clone());

        let mut load_anim_block = |o: u16| -> anyhow::Result<Vec<IscriptLabel>> {
            if o == 0 {
                return Ok(vec![]);
            } else if anim_blocks.contains_key(&o) {
                anim_ref_count.entry(o).and_modify(|e| *e += 1);
                return Ok(vec![]);
            }

            let cursor = Cursor::new(&bytes[o as usize..]);
            let (block, references) = load_animation_block(cursor)?;
            anim_blocks.insert(o, block);
            anim_ref_count.insert(o, 1);

            Ok(references)
        };

        for o in anim_offsets {
            let mut references = load_anim_block(o)?;

            while !references.is_empty() {
                let mut new_references = Vec::new();
                for o in references {
                    new_references.extend(load_anim_block(o.0)?);
                }
                references = new_references;
            }
        }

        next_offset += 4;
    }

    Ok(IscriptBinData {
        iscripts,
        anim_blocks,
        anim_ref_count,
    })
}

fn load_animation_block<R: Read>(
    mut r: R,
) -> anyhow::Result<(Vec<IscriptCommand>, Vec<IscriptLabel>)> {
    let mut commands = Vec::new();
    let mut referenced = Vec::new();

    loop {
        let op: IscriptOp = r.read_u8()?.try_into()?;
        let command = read_command(&mut r, op)?;

        match command {
            IscriptCommand::End | IscriptCommand::Return => break,
            IscriptCommand::Goto(label) => {
                referenced.push(label);
                break;
            }
            IscriptCommand::RandomConditionalJump { label, .. }
            | IscriptCommand::Call(label)
            | IscriptCommand::PowerupConditionalJump(label)
            | IscriptCommand::TriggerTargetRangeConditionalJump { label, .. }
            | IscriptCommand::TriggerTargetCConditionalJump { label, .. }
            | IscriptCommand::CurrentDirectionConditionalJump { label, .. }
            | IscriptCommand::LiftOffConditionalJump(label) => {
                referenced.push(label);
            }
            _ => {}
        }

        commands.push(command);
    }

    Ok((commands, referenced))
}

struct IscriptId(NonZeroU16);

impl ToTokens for IscriptId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0.get();
        let code = quote! { IscriptId(NonZeroU16::new_unchecked(#id)) };
        code.to_tokens(tokens);
    }
}

pub fn write_iscripts(data: IscriptBinData) -> anyhow::Result<()> {
    let mut sorted_anim_blocks = data.anim_blocks.into_iter().collect::<Vec<_>>();
    // Sort the most referenced blocks to the beginning (I don't know if this is the best way to
    // sort these but it does maybe make them easier to navigate in the generated code)
    sorted_anim_blocks.sort_by_key(|(k, _)| u32::MAX - *data.anim_ref_count.get(k).unwrap_or(&0));
    // Add an empty anim block at the beginning so we can optimize the Option<IscriptId> storage
    sorted_anim_blocks.insert(0, (0, Vec::new()));

    // Create a mapping from the original anim block offset to a compact ID
    let mut anim_block_ids = HashMap::new();
    for (i, (k, _)) in sorted_anim_blocks.iter().enumerate() {
        anim_block_ids.insert(*k, i as u16);
    }

    let num_iscripts = data.iscripts.keys().max().unwrap() + 1;
    let mut entries = Vec::new();
    for id in 0..num_iscripts {
        if let Some(s) = data.iscripts.get(&id) {
            let mut anims = s
                .iter()
                .map(|&a| {
                    if a == 0 {
                        None
                    } else {
                        Some(anim_block_ids[&a])
                    }
                })
                .collect::<Vec<_>>();
            // Truncate any trailing nulls for minor space savings
            if let Some(i) = anims.iter().rposition(|x| x.is_some()) {
                let new_len = i + 1;
                anims.truncate(new_len);
            }
            let anims = anims
                .iter()
                .map(|&a| PreservedOption(a.map(|a| IscriptId(NonZeroU16::new(a).unwrap()))))
                .collect::<Vec<_>>();

            entries.push(quote! {
                IscriptCollection {
                    id: #id,
                    scripts: &[#(#anims),*],
                }
            });
        } else {
            entries.push(quote! {
                IscriptCollection {
                    id: #id,
                    scripts: &[],
                }
            });
        }
    }

    for (_, v) in sorted_anim_blocks.iter_mut() {
        for c in v.iter_mut() {
            use IscriptCommand::*;
            // Rewrite every command involving a label to point to our new label IDs
            match c {
                Goto(IscriptLabel(label)) => {
                    *c = Goto(IscriptLabel(
                        *anim_block_ids
                            .get(label)
                            .context("Unknown label: {label}")?,
                    ));
                }
                RandomConditionalJump {
                    chance,
                    label: IscriptLabel(label),
                } => {
                    *c = RandomConditionalJump {
                        chance: *chance,
                        label: IscriptLabel(
                            *anim_block_ids
                                .get(label)
                                .context("Unknown label: {label}")?,
                        ),
                    };
                }
                Call(IscriptLabel(label)) => {
                    *c = Call(IscriptLabel(
                        *anim_block_ids
                            .get(label)
                            .context("Unknown label: {label}")?,
                    ));
                }
                PowerupConditionalJump(IscriptLabel(label)) => {
                    *c = PowerupConditionalJump(IscriptLabel(
                        *anim_block_ids
                            .get(label)
                            .context("Unknown label: {label}")?,
                    ));
                }
                TriggerTargetRangeConditionalJump {
                    distance,
                    label: IscriptLabel(label),
                } => {
                    *c = TriggerTargetRangeConditionalJump {
                        distance: *distance,
                        label: IscriptLabel(
                            *anim_block_ids
                                .get(label)
                                .context("Unknown label: {label}")?,
                        ),
                    };
                }
                TriggerTargetCConditionalJump {
                    angle1,
                    angle2,
                    label: IscriptLabel(label),
                } => {
                    *c = TriggerTargetCConditionalJump {
                        angle1: *angle1,
                        angle2: *angle2,
                        label: IscriptLabel(
                            *anim_block_ids
                                .get(label)
                                .context("Unknown label: {label}")?,
                        ),
                    };
                }
                CurrentDirectionConditionalJump {
                    angle1,
                    angle2,
                    label: IscriptLabel(label),
                } => {
                    *c = CurrentDirectionConditionalJump {
                        angle1: *angle1,
                        angle2: *angle2,
                        label: IscriptLabel(
                            *anim_block_ids
                                .get(label)
                                .context("Unknown label: {label}")?,
                        ),
                    };
                }
                LiftOffConditionalJump(IscriptLabel(label)) => {
                    *c = LiftOffConditionalJump(IscriptLabel(
                        *anim_block_ids
                            .get(label)
                            .context("Unknown label: {label}")?,
                    ));
                }
                _ => {}
            }
        }
    }

    let num_iscripts = num_iscripts as usize;

    let anim_entries = sorted_anim_blocks
        .iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    let num_anim_entries = anim_entries.len();

    let tokens = quote! {
        use crate::gamedata::{
            FlipState, FrameId, FrameIdByte, FrameSet, GasOverlay, ImageId, IscriptCollection,
            IscriptCommand as IC, IscriptId, IscriptLabel, OverlayId, SignalId, SoundId, Speed, SpriteId,
            WeaponId, WeaponType,
        };
        use std::num::NonZeroU16;

        /// Contains all the iscript collections referenced by images in the game.
        pub const ISCRIPTS: [IscriptCollection<'static>; #num_iscripts] = unsafe { [#(#entries,)*] };

        pub const ISCRIPT_ANIMS: [&[IC]; #num_anim_entries] = [
            #(
                &[
                    #(#anim_entries,)*
                ],
            )*
        ];
    };

    let src = syn::parse2(tokens).expect("Couldn't parse generated iscript.rs");
    let src = prettyplease::unparse(&src);
    std::fs::write("src/gamedata/generated/iscript.rs", src)
        .expect("Couldn't write generated/iscript.rs");

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, TryFromPrimitive)]
#[repr(u8)]
pub enum IscriptOp {
    PlayFrame = 0,
    PlayFrameTile = 1,
    SetHorizontalPosition = 2,
    SetVerticalPosition = 3,
    SetPosition = 4,
    Wait = 5,
    WaitRandom = 6,
    Goto = 7,
    ImageOverlay = 8,
    ImageUnderlay = 9,
    ImageOverlayOriginal = 10,
    SwitchUnderlay = 11,
    // 12 is unused
    ImageOverlayUseLo = 13,
    ImageUnderlayUseLo = 14,
    SpriteOverlay = 15,
    HighSpriteOverlay = 16,
    LowSpriteUnderlay = 17,
    UnusedFlingyUnstable = 18,
    SpriteUnderlayUseLo = 19,
    SpriteUnderlay = 20,
    SpriteOverlayUseLo = 21,
    End = 22,
    SetFlipState = 23,
    PlaySound = 24,
    PlaySoundRandom = 25,
    PlaySoundBetween = 26,
    DoMissileDamage = 27,
    AttackMelee = 28,
    FollowMainGraphic = 29,
    RandomConditionalJump = 30,
    TurnCounterClockwise = 31,
    TurnClockwise = 32,
    TurnOnceClockwise = 33,
    TurnRandom = 34,
    SetSpawnFrame = 35,
    SignalOrder = 36,
    AttackWith = 37,
    Attack = 38,
    CastSpell = 39,
    UseWeapon = 40,
    Move = 41,
    GotoRepeatAttack = 42,
    EngFrame = 43,
    EngSet = 44,
    Unknown45 = 45,
    NoBreakCodeStart = 46,
    NoBreakCodeEnd = 47,
    IgnoreRest = 48,
    AttackShiftProjectiles = 49,
    TempRemoveGraphicStart = 50,
    TempRemoveGraphicEnd = 51,
    SetFlingyDirection = 52,
    Call = 53,
    Return = 54,
    SetFlingySpeed = 55,
    CreateGasOverlays = 56,
    PowerupConditionalJump = 57,
    TriggerTargetRangeConditionalJump = 58,
    TriggerTargetCConditionalJump = 59,
    CurrentDirectionConditionalJump = 60,
    ImageUnderlayNextId = 61,
    // 62 is unused
    LiftOffConditionalJump = 63,
    WarpOverlay = 64,
    OrderDone = 65,
    GroundSpriteOverlay = 66,
    Unknown67 = 67,
    DoGroundDamage = 68,
}

// NOTE(tec27): These are not shared with the game so we have a bit more freedom and the game
// code can derive things on them without needing newtypes

#[derive(Debug, Clone, Copy)]
pub struct FrameId(u16);
#[derive(Debug, Clone, Copy)]
pub struct FrameIdByte(u8);
#[derive(Debug, Clone, Copy)]
pub struct FrameSet(u8);
#[derive(Debug, Clone, Copy)]
pub struct IscriptLabel(u16);
#[derive(Debug, Clone, Copy)]
pub struct ImageId(u16);
#[derive(Debug, Clone, Copy)]
pub struct SpriteId(u16);
#[derive(Debug, Clone, Copy)]
pub struct OverlayId(u8);
#[derive(Debug, Clone, Copy)]
pub struct FlipState(u8);
#[derive(Debug, Clone, Copy)]
pub struct SoundId(u16);
#[derive(Debug, Clone, Copy)]
pub struct SignalId(u8);
#[derive(Debug, Clone, Copy)]
pub struct WeaponId(u8);
#[derive(Debug, Clone, Copy)]
pub struct Speed(u16);
#[derive(Debug, Clone, Copy)]
pub struct GasOverlay(u8);

impl ToTokens for FrameId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { FrameId(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for FrameIdByte {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { FrameIdByte(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for FrameSet {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { FrameSet(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for IscriptLabel {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { IscriptLabel(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for ImageId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { ImageId(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for SpriteId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { SpriteId(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for OverlayId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { OverlayId(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for FlipState {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { FlipState(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for SoundId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { SoundId(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for SignalId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { SignalId(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for WeaponId {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { WeaponId(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for Speed {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { Speed(#id) };
        code.to_tokens(tokens);
    }
}

impl ToTokens for GasOverlay {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = self.0;
        let code = quote! { GasOverlay(#id) };
        code.to_tokens(tokens);
    }
}

#[derive(Debug, Clone)]
pub enum IscriptCommand {
    PlayFrame {
        frame: FrameId,
    },
    PlayFrameTile {
        frame: FrameId,
    },
    SetHorizontalPosition(i8),
    SetVerticalPosition(i8),
    SetPosition {
        x: i8,
        y: i8,
    },
    Wait(u8),
    WaitRandom {
        min: u8,
        max: u8,
    },
    Goto(IscriptLabel),
    ImageOverlay {
        image: ImageId,
        x: i8,
        y: i8,
    },
    ImageUnderlay {
        image: ImageId,
        x: i8,
        y: i8,
    },
    ImageOverlayOriginal(ImageId),
    SwitchUnderlay(ImageId),
    ImageOverlayUseLo {
        image: ImageId,
        x: i8,
        y: i8,
    },
    ImageUnderlayUseLo {
        image: ImageId,
        x: i8,
        y: i8,
    },
    SpriteOverlay {
        sprite: SpriteId,
        x: i8,
        y: i8,
    },
    HighSpriteOverlay {
        sprite: SpriteId,
        x: i8,
        y: i8,
    },
    LowSpriteUnderlay {
        sprite: SpriteId,
        x: i8,
        y: i8,
    },
    UnusedFlingyUnstable(u16),
    SpriteUnderlayUseLo {
        sprite: SpriteId,
        x: i8,
        y: i8,
    },
    SpriteUnderlay {
        sprite: SpriteId,
        x: i8,
        y: i8,
    },
    SpriteOverlayUseLo {
        sprite: SpriteId,
        overlay: OverlayId,
    },
    End,
    SetFlipState(FlipState),
    PlaySound(SoundId),
    PlaySoundRandom {
        num_sounds: u8,
        sounds: Vec<SoundId>,
    },
    PlaySoundBetween {
        min: SoundId,
        max: SoundId,
    },
    DoMissileDamage,
    AttackMelee {
        num_sounds: u8,
        sounds: Vec<SoundId>,
    },
    FollowMainGraphic,
    RandomConditionalJump {
        chance: u8,
        label: IscriptLabel,
    },
    TurnCounterClockwise(u8),
    TurnClockwise(u8),
    TurnOnceClockwise,
    TurnRandom(u8),
    SetSpawnFrame(u8),
    SignalOrder(SignalId),
    AttackWith(u8),
    Attack,
    CastSpell,
    UseWeapon(WeaponId),
    Move(u8),
    GotoRepeatAttack,
    EngineFrame(FrameIdByte),
    EngineSet(FrameSet),
    Unknown45,
    NoBreakCodeStart,
    NoBreakCodeEnd,
    IgnoreRest,
    AttackShiftProjectiles(u8),
    TempRemoveGraphicStart,
    TempRemoveGraphicEnd,
    SetFlingyDirection(u8),
    Call(IscriptLabel),
    Return,
    SetFlingySpeed(Speed),
    CreateGasOverlays(GasOverlay),
    PowerupConditionalJump(IscriptLabel),
    TriggerTargetRangeConditionalJump {
        distance: u16,
        label: IscriptLabel,
    },
    TriggerTargetCConditionalJump {
        angle1: u16,
        angle2: u16,
        label: IscriptLabel,
    },
    CurrentDirectionConditionalJump {
        angle1: u16,
        angle2: u16,
        label: IscriptLabel,
    },
    ImageUnderlayNextId {
        x: i8,
        y: i8,
    },
    LiftOffConditionalJump(IscriptLabel),
    WarpOverlay(FrameId),
    OrderDone(SignalId),
    GroundSpriteOverlay {
        sprite: SpriteId,
        x: i8,
        y: i8,
    },
    Unknown67,
    DoGroundDamage,
}

impl ToTokens for IscriptCommand {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use IscriptCommand::*;

        let code = match self {
            PlayFrame { frame } => {
                quote! { IC::PlayFrame { frame: #frame } }
            }
            PlayFrameTile { frame } => {
                quote! { IC::PlayFrameTile { frame: #frame } }
            }
            SetHorizontalPosition(x) => {
                quote! { IC::SetHorizontalPosition(#x) }
            }
            SetVerticalPosition(y) => {
                quote! { IC::SetVerticalPosition(#y) }
            }
            SetPosition { x, y } => {
                quote! { IC::SetPosition { x: #x, y: #y } }
            }
            Wait(frames) => {
                quote! { IC::Wait(#frames) }
            }
            WaitRandom { min, max } => {
                quote! { IC::WaitRandom { min: #min, max: #max } }
            }
            Goto(label) => {
                quote! { IC::Goto(#label) }
            }
            ImageOverlay { image, x, y } => {
                quote! { IC::ImageOverlay { image: #image, x: #x, y: #y } }
            }
            ImageUnderlay { image, x, y } => {
                quote! { IC::ImageUnderlay { image: #image, x: #x, y: #y } }
            }
            ImageOverlayOriginal(image) => {
                quote! { IC::ImageOverlayOriginal(#image) }
            }
            SwitchUnderlay(image) => {
                quote! { IC::SwitchUnderlay(#image) }
            }
            ImageOverlayUseLo { image, x, y } => {
                quote! { IC::ImageOverlayUseLo { image: #image, x: #x, y: #y } }
            }
            ImageUnderlayUseLo { image, x, y } => {
                quote! { IC::ImageUnderlayUseLo { image: #image, x: #x, y: #y } }
            }
            SpriteOverlay { sprite, x, y } => {
                quote! { IC::SpriteOverlay { sprite: #sprite, x: #x, y: #y } }
            }
            HighSpriteOverlay { sprite, x, y } => {
                quote! { IC::HighSpriteOverlay { sprite: #sprite, x: #x, y: #y } }
            }
            LowSpriteUnderlay { sprite, x, y } => {
                quote! { IC::LowSpriteUnderlay { sprite: #sprite, x: #x, y: #y } }
            }
            UnusedFlingyUnstable(flingy) => {
                quote! { IC::UnusedFlingyUnstable(#flingy) }
            }
            SpriteUnderlayUseLo { sprite, x, y } => {
                quote! { IC::SpriteUnderlayUseLo { sprite: #sprite, x: #x, y: #y } }
            }
            SpriteUnderlay { sprite, x, y } => {
                quote! { IC::SpriteUnderlay { sprite: #sprite, x: #x, y: #y } }
            }
            SpriteOverlayUseLo { sprite, overlay } => {
                quote! { IC::SpriteOverlayUseLo { sprite: #sprite, overlay: #overlay } }
            }
            End => quote! { IC::End },
            SetFlipState(state) => quote! { IC::SetFlipState(#state) },
            PlaySound(sound) => quote! { IC::PlaySound(#sound) },
            PlaySoundRandom { num_sounds, sounds } => {
                quote! {
                    IC::PlaySoundRandom { num_sounds: #num_sounds, sounds: &[#(#sounds),*] }
                }
            }
            PlaySoundBetween { min, max } => {
                quote! { IC::PlaySoundBetween { min: #min, max: #max } }
            }
            DoMissileDamage => quote! { IC::DoMissileDamage },
            AttackMelee { num_sounds, sounds } => {
                quote! {
                    IC::AttackMelee { num_sounds: #num_sounds, sounds: &[#(#sounds),*] }
                }
            }
            FollowMainGraphic => quote! { IC::FollowMainGraphic },
            RandomConditionalJump { chance, label } => {
                quote! { IC::RandomConditionalJump { chance: #chance, label: #label } }
            }
            TurnCounterClockwise(amount) => {
                quote! { IC::TurnCounterClockwise(#amount) }
            }
            TurnClockwise(amount) => {
                quote! { IC::TurnClockwise(#amount) }
            }
            TurnOnceClockwise => quote! { IC::TurnOnceClockwise },
            TurnRandom(amount) => {
                quote! { IC::TurnRandom(#amount) }
            }
            SetSpawnFrame(frame) => {
                quote! { IC::SetSpawnFrame(#frame) }
            }
            SignalOrder(signal) => {
                quote! { IC::SignalOrder(#signal) }
            }
            AttackWith(weapon) => {
                let weapon = match weapon {
                    1 => quote! { WeaponType::Ground },
                    2 => quote! { WeaponType::Air },
                    _ => panic!("Invalid weapon type: {weapon}"),
                };
                quote! { IC::AttackWith(#weapon) }
            }
            Attack => quote! { IC::Attack },
            CastSpell => quote! { IC::CastSpell },
            UseWeapon(weapon) => {
                quote! { IC::UseWeapon(#weapon) }
            }
            Move(amount) => {
                quote! { IC::Move(#amount) }
            }
            GotoRepeatAttack => quote! { IC::GotoRepeatAttack },
            EngineFrame(frame) => {
                quote! { IC::EngineFrame(#frame) }
            }
            EngineSet(set) => {
                quote! { IC::EngineSet(#set) }
            }
            Unknown45 => quote! { IC::Unknown45 },
            NoBreakCodeStart => quote! { IC::NoBreakCodeStart },
            NoBreakCodeEnd => quote! { IC::NoBreakCodeEnd },
            IgnoreRest => quote! { IC::IgnoreRest },
            AttackShiftProjectiles(amount) => {
                quote! { IC::AttackShiftProjectiles(#amount) }
            }
            TempRemoveGraphicStart => quote! { IC::TempRemoveGraphicStart },
            TempRemoveGraphicEnd => quote! { IC::TempRemoveGraphicEnd },
            SetFlingyDirection(direction) => {
                quote! { IC::SetFlingyDirection(#direction) }
            }
            Call(label) => {
                quote! { IC::Call(#label) }
            }
            Return => quote! { IC::Return },
            SetFlingySpeed(speed) => {
                quote! { IC::SetFlingySpeed(#speed) }
            }
            CreateGasOverlays(overlay) => {
                quote! { IC::CreateGasOverlays(#overlay) }
            }
            PowerupConditionalJump(label) => {
                quote! { IC::PowerupConditionalJump(#label) }
            }
            TriggerTargetRangeConditionalJump { distance, label } => {
                quote! {
                    IC::TriggerTargetRangeConditionalJump { distance: #distance, label: #label }
                }
            }
            TriggerTargetCConditionalJump {
                angle1,
                angle2,
                label,
            } => {
                quote! {
                    IC::TriggerTargetCConditionalJump {
                        angle1: #angle1,
                        angle2: #angle2,
                        label: #label,
                    }
                }
            }
            CurrentDirectionConditionalJump {
                angle1,
                angle2,
                label,
            } => {
                quote! {
                    IC::CurrentDirectionConditionalJump {
                        angle1: #angle1,
                        angle2: #angle2,
                        label: #label,
                    }
                }
            }
            ImageUnderlayNextId { x, y } => {
                quote! { IC::ImageUnderlayNextId { x: #x, y: #y } }
            }
            LiftOffConditionalJump(label) => {
                quote! { IC::LiftOffConditionalJump(#label) }
            }
            WarpOverlay(frame) => {
                quote! { IC::WarpOverlay(#frame) }
            }
            OrderDone(signal) => {
                quote! { IC::OrderDone(#signal) }
            }
            GroundSpriteOverlay { sprite, x, y } => {
                quote! { IC::GroundSpriteOverlay { sprite: #sprite, x: #x, y: #y } }
            }
            Unknown67 => quote! { IC::Unknown67 },
            DoGroundDamage => quote! { IC::DoGroundDamage },
        };

        code.to_tokens(tokens);
    }
}

fn read_command<R: Read>(r: &mut R, op: IscriptOp) -> anyhow::Result<IscriptCommand> {
    let result = match op {
        IscriptOp::PlayFrame => IscriptCommand::PlayFrame {
            frame: FrameId(r.read_u16::<LittleEndian>()?),
        },
        IscriptOp::PlayFrameTile => IscriptCommand::PlayFrameTile {
            frame: FrameId(r.read_u16::<LittleEndian>()?),
        },
        IscriptOp::SetHorizontalPosition => IscriptCommand::SetHorizontalPosition(r.read_i8()?),
        IscriptOp::SetVerticalPosition => IscriptCommand::SetVerticalPosition(r.read_i8()?),
        IscriptOp::SetPosition => IscriptCommand::SetPosition {
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::Wait => IscriptCommand::Wait(r.read_u8()?),
        IscriptOp::WaitRandom => IscriptCommand::WaitRandom {
            min: r.read_u8()?,
            max: r.read_u8()?,
        },
        IscriptOp::Goto => IscriptCommand::Goto(IscriptLabel(r.read_u16::<LittleEndian>()?)),
        IscriptOp::ImageOverlay => IscriptCommand::ImageOverlay {
            image: ImageId(r.read_u16::<LittleEndian>()?),
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::ImageUnderlay => IscriptCommand::ImageUnderlay {
            image: ImageId(r.read_u16::<LittleEndian>()?),
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::ImageOverlayOriginal => {
            IscriptCommand::ImageOverlayOriginal(ImageId(r.read_u16::<LittleEndian>()?))
        }
        IscriptOp::SwitchUnderlay => {
            IscriptCommand::SwitchUnderlay(ImageId(r.read_u16::<LittleEndian>()?))
        }
        IscriptOp::ImageOverlayUseLo => IscriptCommand::ImageOverlayUseLo {
            image: ImageId(r.read_u16::<LittleEndian>()?),
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::ImageUnderlayUseLo => IscriptCommand::ImageUnderlayUseLo {
            image: ImageId(r.read_u16::<LittleEndian>()?),
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::SpriteOverlay => IscriptCommand::SpriteOverlay {
            sprite: SpriteId(r.read_u16::<LittleEndian>()?),
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::HighSpriteOverlay => IscriptCommand::HighSpriteOverlay {
            sprite: SpriteId(r.read_u16::<LittleEndian>()?),
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::LowSpriteUnderlay => IscriptCommand::LowSpriteUnderlay {
            sprite: SpriteId(r.read_u16::<LittleEndian>()?),
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::UnusedFlingyUnstable => {
            IscriptCommand::UnusedFlingyUnstable(r.read_u16::<LittleEndian>()?)
        }
        IscriptOp::SpriteUnderlayUseLo => IscriptCommand::SpriteUnderlayUseLo {
            sprite: SpriteId(r.read_u16::<LittleEndian>()?),
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::SpriteUnderlay => IscriptCommand::SpriteUnderlay {
            sprite: SpriteId(r.read_u16::<LittleEndian>()?),
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::SpriteOverlayUseLo => IscriptCommand::SpriteOverlayUseLo {
            sprite: SpriteId(r.read_u16::<LittleEndian>()?),
            overlay: OverlayId(r.read_u8()?),
        },
        IscriptOp::End => IscriptCommand::End,
        IscriptOp::SetFlipState => IscriptCommand::SetFlipState(FlipState(r.read_u8()?)),
        IscriptOp::PlaySound => IscriptCommand::PlaySound(SoundId(r.read_u16::<LittleEndian>()?)),
        IscriptOp::PlaySoundRandom => {
            let num_sounds = r.read_u8()?;
            let sounds = (0..num_sounds)
                .map(|_| r.read_u16::<LittleEndian>())
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .map(|&x| SoundId(x))
                .collect();
            IscriptCommand::PlaySoundRandom { num_sounds, sounds }
        }
        IscriptOp::PlaySoundBetween => IscriptCommand::PlaySoundBetween {
            min: SoundId(r.read_u16::<LittleEndian>()?),
            max: SoundId(r.read_u16::<LittleEndian>()?),
        },
        IscriptOp::DoMissileDamage => IscriptCommand::DoMissileDamage,
        IscriptOp::AttackMelee => {
            let num_sounds = r.read_u8()?;
            let sounds = (0..num_sounds)
                .map(|_| r.read_u16::<LittleEndian>())
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .map(|&x| SoundId(x))
                .collect();
            IscriptCommand::AttackMelee { num_sounds, sounds }
        }
        IscriptOp::FollowMainGraphic => IscriptCommand::FollowMainGraphic,
        IscriptOp::RandomConditionalJump => IscriptCommand::RandomConditionalJump {
            chance: r.read_u8()?,
            label: IscriptLabel(r.read_u16::<LittleEndian>()?),
        },
        IscriptOp::TurnCounterClockwise => IscriptCommand::TurnCounterClockwise(r.read_u8()?),
        IscriptOp::TurnClockwise => IscriptCommand::TurnClockwise(r.read_u8()?),
        IscriptOp::TurnOnceClockwise => IscriptCommand::TurnOnceClockwise,
        IscriptOp::TurnRandom => IscriptCommand::TurnRandom(r.read_u8()?),
        IscriptOp::SetSpawnFrame => IscriptCommand::SetSpawnFrame(r.read_u8()?),
        IscriptOp::SignalOrder => IscriptCommand::SignalOrder(SignalId(r.read_u8()?)),
        IscriptOp::AttackWith => IscriptCommand::AttackWith(r.read_u8()?),
        IscriptOp::Attack => IscriptCommand::Attack,
        IscriptOp::CastSpell => IscriptCommand::CastSpell,
        IscriptOp::UseWeapon => IscriptCommand::UseWeapon(WeaponId(r.read_u8()?)),
        IscriptOp::Move => IscriptCommand::Move(r.read_u8()?),
        IscriptOp::GotoRepeatAttack => IscriptCommand::GotoRepeatAttack,
        IscriptOp::EngFrame => IscriptCommand::EngineFrame(FrameIdByte(r.read_u8()?)),
        IscriptOp::EngSet => IscriptCommand::EngineSet(FrameSet(r.read_u8()?)),
        IscriptOp::Unknown45 => IscriptCommand::Unknown45,
        IscriptOp::NoBreakCodeStart => IscriptCommand::NoBreakCodeStart,
        IscriptOp::NoBreakCodeEnd => IscriptCommand::NoBreakCodeEnd,
        IscriptOp::IgnoreRest => IscriptCommand::IgnoreRest,
        IscriptOp::AttackShiftProjectiles => IscriptCommand::AttackShiftProjectiles(r.read_u8()?),
        IscriptOp::TempRemoveGraphicStart => IscriptCommand::TempRemoveGraphicStart,
        IscriptOp::TempRemoveGraphicEnd => IscriptCommand::TempRemoveGraphicEnd,
        IscriptOp::SetFlingyDirection => IscriptCommand::SetFlingyDirection(r.read_u8()?),
        IscriptOp::Call => IscriptCommand::Call(IscriptLabel(r.read_u16::<LittleEndian>()?)),
        IscriptOp::Return => IscriptCommand::Return,
        IscriptOp::SetFlingySpeed => {
            IscriptCommand::SetFlingySpeed(Speed(r.read_u16::<LittleEndian>()?))
        }
        IscriptOp::CreateGasOverlays => IscriptCommand::CreateGasOverlays(GasOverlay(r.read_u8()?)),
        IscriptOp::PowerupConditionalJump => {
            IscriptCommand::PowerupConditionalJump(IscriptLabel(r.read_u16::<LittleEndian>()?))
        }
        IscriptOp::TriggerTargetRangeConditionalJump => {
            IscriptCommand::TriggerTargetRangeConditionalJump {
                distance: r.read_u16::<LittleEndian>()?,
                label: IscriptLabel(r.read_u16::<LittleEndian>()?),
            }
        }
        IscriptOp::TriggerTargetCConditionalJump => IscriptCommand::TriggerTargetCConditionalJump {
            angle1: r.read_u16::<LittleEndian>()?,
            angle2: r.read_u16::<LittleEndian>()?,
            label: IscriptLabel(r.read_u16::<LittleEndian>()?),
        },
        IscriptOp::CurrentDirectionConditionalJump => {
            IscriptCommand::CurrentDirectionConditionalJump {
                angle1: r.read_u16::<LittleEndian>()?,
                angle2: r.read_u16::<LittleEndian>()?,
                label: IscriptLabel(r.read_u16::<LittleEndian>()?),
            }
        }
        IscriptOp::ImageUnderlayNextId => IscriptCommand::ImageUnderlayNextId {
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::LiftOffConditionalJump => {
            IscriptCommand::LiftOffConditionalJump(IscriptLabel(r.read_u16::<LittleEndian>()?))
        }
        IscriptOp::WarpOverlay => {
            IscriptCommand::WarpOverlay(FrameId(r.read_u16::<LittleEndian>()?))
        }
        IscriptOp::OrderDone => IscriptCommand::OrderDone(SignalId(r.read_u8()?)),
        IscriptOp::GroundSpriteOverlay => IscriptCommand::GroundSpriteOverlay {
            sprite: SpriteId(r.read_u16::<LittleEndian>()?),
            x: r.read_i8()?,
            y: r.read_i8()?,
        },
        IscriptOp::Unknown67 => IscriptCommand::Unknown67,
        IscriptOp::DoGroundDamage => IscriptCommand::DoGroundDamage,
    };

    Ok(result)
}

fn type_to_animation_block_len(ty: u8) -> anyhow::Result<usize> {
    match ty {
        0 | 1 => Ok(2),
        2 => Ok(4),
        12 | 13 => Ok(14),
        14 | 15 => Ok(16),
        20 | 21 => Ok(22),
        23 => Ok(24),
        24 => Ok(26),
        26..=29 => Ok(28),
        _ => Err(anyhow!("Invalid iscript type: {ty}")),
    }
}
