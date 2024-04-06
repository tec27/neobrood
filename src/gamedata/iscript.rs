use std::num::NonZeroU16;

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone)]
pub struct IscriptCollection<'a> {
    pub id: u16,
    pub scripts: &'a [Option<IscriptId>],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IscriptId(pub NonZeroU16);

#[derive(Copy, Clone, Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(usize)]
pub enum IscriptType {
    Init = 0,
    Death = 1,
    GroundAttackInit = 2,
    AirAttackInit = 3,
    Unused0 = 4,
    GroundAttackRepeat = 5,
    AirAttackRepeat = 6,
    CastSpell = 7,
    GroundAttackToIdle = 8,
    AirAttackToIdle = 9,
    Unused2 = 10,
    Walking = 11,
    WalkingToIdle = 12,
    SpecialState1 = 13,
    SpecialState2 = 14,
    AlmostBuilt = 15,
    Built = 16,
    Landing = 17,
    LiftOff = 18,
    Working = 19,
    WorkingToIdle = 20,
    WarpIn = 21,
    Unused3 = 22,
    StarEditInit = 23,
    Disable = 24,
    Burrow = 25,
    Unburrow = 26,
    Enable = 27,
}

#[derive(Debug, Clone, Copy)]
pub struct FrameId(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct FrameIdByte(pub u8);

#[derive(Debug, Clone, Copy)]
pub struct FrameSet(pub u8);

#[derive(Debug, Clone, Copy)]
pub struct IscriptLabel(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct ImageId(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct SpriteId(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct OverlayId(pub u8);

#[derive(Debug, Clone, Copy)]
pub struct FlipState(pub u8);

#[derive(Debug, Clone, Copy)]
pub struct SoundId(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct SignalId(pub u8);

#[derive(Debug, Clone, Copy)]
pub struct WeaponId(pub u8);

#[derive(Debug, Clone, Copy)]
pub struct Speed(pub u16);

#[derive(Debug, Clone, Copy)]
pub struct GasOverlay(pub u8);

#[derive(Debug, Clone, Copy)]
pub enum WeaponType {
    Ground,
    Air,
}

#[allow(dead_code)]
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
        sounds: &'static [SoundId],
    },
    PlaySoundBetween {
        min: SoundId,
        max: SoundId,
    },
    DoMissileDamage,
    AttackMelee {
        num_sounds: u8,
        sounds: &'static [SoundId],
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
    AttackWith(WeaponType),
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
