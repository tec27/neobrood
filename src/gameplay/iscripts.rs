use crate::{
    gamedata::{
        IscriptCollection, IscriptCommand, IscriptType, LoadingAnimBundle, RenderStyle, ISCRIPTS,
        ISCRIPT_ANIMS,
    },
    random::LcgRand,
};
use bevy::{math::I16Vec2, prelude::*};
use std::ops::DerefMut;

use super::constructs::{ConstructImage, ConstructImageBundle, ConstructSprite, ImageOrder};

impl<'a> IscriptCollection<'a> {
    /// Returns the script of the given type within this collection, if it exists.
    pub fn get(&self, index: IscriptType) -> Option<&'a [IscriptCommand]> {
        let script = match self.scripts.get(usize::from(index)) {
            Some(script) => *script,
            None => None,
        };

        script.map(|s| ISCRIPT_ANIMS[s.0.get() as usize])
    }
}

/// Contains the state for a running iscript animation, and can be used to continue executing the
/// current animation or start a new one.
#[derive(Component, Debug, Clone)]
pub struct IscriptController {
    pub collection: &'static IscriptCollection<'static>,
    pub use_full_collection: bool,

    // state for the animation playback
    current_anim: Option<IscriptType>,
    /// The currently executing anim "program". Note this may not correspond to the program pointed
    /// to by `current_anim` within `collection` as `Goto` instructions can change it.
    program: Option<&'static [IscriptCommand]>,
    /// "Program counter" within the current animation. This is the offset of the next instruction
    /// to execute within [program].
    pc: usize,
    /// The target to return to when the current animation returns. This is a tuple of the target
    /// program and the `pc` value within it.
    return_target: Option<(&'static [IscriptCommand], usize)>,
    /// The number of turns to wait before executing the next instruction.
    wait_timer: u8,
}

const DEFAULT_ISCRIPT_COLLECTION: IscriptCollection = IscriptCollection {
    id: 0,
    scripts: &[],
};

impl Default for IscriptController {
    fn default() -> Self {
        Self {
            collection: &DEFAULT_ISCRIPT_COLLECTION,
            use_full_collection: false,
            current_anim: None,
            program: None,
            pc: 0,
            return_target: None,
            wait_timer: 0,
        }
    }
}

/// Context for executing an iscript animation. This is state that is necessary for the various
/// operations an iscript animation can perform.
pub struct IscriptExecContext<'a, ImageType, SpriteType>
where
    ImageType: DerefMut<Target = ConstructImage>,
    SpriteType: DerefMut<Target = ConstructSprite>,
{
    /// The [Entity] of the image the script is associated with.
    pub image_entity: Entity,
    /// The [ConstructImage] the script is associated with. This should generally be a [Mut]
    /// returned from a query.
    pub image: &'a mut ImageType,
    /// The [Entity] of the sprite that the image is associated with.
    pub parent_sprite_entity: Entity,
    /// The sprite that the image is associated with.
    pub parent_sprite: &'a mut SpriteType,
    /// The random number generator to use for any random operations.
    pub rand: &'a mut LcgRand,
}

impl IscriptController {
    pub fn for_image(image: &ConstructImage) -> Self {
        let collection = &ISCRIPTS[image.def().iscript as usize];
        Self {
            collection,
            use_full_collection: image.def().use_full_iscript,
            ..default()
        }
    }

    pub fn reset(&mut self) {
        self.current_anim = None;
        self.program = None;
        self.pc = 0;
        self.return_target = None;
        self.wait_timer = 0;
    }

    pub fn run_anim<I, S>(
        &mut self,
        mut anim: IscriptType,
        context: IscriptExecContext<I, S>,
        commands: &mut Commands,
    ) where
        I: DerefMut<Target = ConstructImage>,
        S: DerefMut<Target = ConstructSprite>,
    {
        if !self.use_full_collection && !matches!(anim, IscriptType::Init | IscriptType::Death) {
            return;
        }
        if matches!(
            anim,
            IscriptType::Death | IscriptType::Walking | IscriptType::Working
        ) && self.current_anim == Some(anim)
        {
            return;
        }

        if anim == IscriptType::GroundAttackRepeat
            && self.current_anim != Some(IscriptType::GroundAttackRepeat)
            && self.current_anim != Some(IscriptType::GroundAttackInit)
        {
            anim = IscriptType::GroundAttackInit;
        }
        if anim == IscriptType::AirAttackRepeat
            && self.current_anim != Some(IscriptType::AirAttackRepeat)
            && self.current_anim != Some(IscriptType::AirAttackInit)
        {
            anim = IscriptType::AirAttackInit;
        }

        self.reset();
        self.current_anim = Some(anim);
        self.program = self.collection.get(anim);

        if self.program.is_none() {
            error!("Asked to play iscript anim {anim:?} but it wasn't in the collection");
        } else {
            self.exec(context, commands);
        }
    }

    fn exec<I, S>(&mut self, mut context: IscriptExecContext<I, S>, commands: &mut Commands)
    where
        I: DerefMut<Target = ConstructImage>,
        S: DerefMut<Target = ConstructSprite>,
    {
        if self.wait_timer > 0 {
            self.wait_timer -= 1;
            return;
        }

        let Some(mut program) = self.program else {
            return;
        };

        loop {
            let command = &program[self.pc];
            // warn!("ISCRIPT: {:?}", command);
            self.pc += 1;
            match command {
                IscriptCommand::End => {
                    // FIXME: this should destroy the image (despawn?)
                    self.reset();
                    break;
                }
                IscriptCommand::Return => {
                    // FIXME: implement properly
                    break;
                }
                IscriptCommand::Goto(anim) => {
                    // NOTE(tec27): As part of the code gen process these are guaranteed to always
                    // exist in the slice
                    program = &ISCRIPT_ANIMS[anim.0 as usize];
                    self.program = Some(program);
                    self.pc = 0;
                }
                IscriptCommand::PlayFrame { frame } => {
                    context.image.frame_base = frame.0;
                }
                IscriptCommand::Wait(frames) => {
                    self.wait_timer = *frames - 1;
                    break;
                }
                IscriptCommand::WaitRandom { min, max } => {
                    self.wait_timer = context.rand.in_range_u8(*min, *max) - 1;
                    break;
                }
                IscriptCommand::ImageOverlay {
                    image: image_id,
                    x,
                    y,
                } => {
                    let offset = I16Vec2::new(*x as i16, *y as i16) + context.image.offset;
                    self.add_image(
                        image_id.0,
                        offset,
                        ImageOrder::Above(Some(context.image_entity)),
                        &mut context,
                        commands,
                    );
                }
                IscriptCommand::ImageUnderlay {
                    image: image_id,
                    x,
                    y,
                } => {
                    let offset = I16Vec2::new(*x as i16, *y as i16) + context.image.offset;
                    self.add_image(
                        image_id.0,
                        offset,
                        ImageOrder::Below(Some(context.image_entity)),
                        &mut context,
                        commands,
                    )
                }
                _c => {
                    // warn!("Unimplemented: {c:?}");
                }
            }
        }

        // warn!("==================");
    }

    /// Creates and initializes an image for an iscript instruction. This will run scripts for the
    /// image immediately but defer the actual entity spawning via [Commands].
    fn add_image<'a, 'b, I, S>(
        &mut self,
        image_id: u16,
        offset: I16Vec2,
        order: ImageOrder,
        creating_context: &'a mut IscriptExecContext<'b, I, S>,
        commands: &mut Commands,
    ) where
        'b: 'a,
        I: DerefMut<Target = ConstructImage>,
        S: DerefMut<Target = ConstructSprite>,
    {
        let mut image = ConstructImage {
            id: image_id,
            offset,
            ..default()
        };
        image.render_style = image.def().render_style;
        let mut iscript = IscriptController::for_image(&image);
        let image_entity = commands.spawn_empty().id();
        creating_context
            .parent_sprite
            .add_image(image_entity, order);

        let context = IscriptExecContext {
            image_entity,
            image: &mut &mut image,
            parent_sprite_entity: creating_context.parent_sprite_entity,
            parent_sprite: creating_context.parent_sprite,
            rand: creating_context.rand,
        };
        iscript.run_anim(IscriptType::Init, context, commands);

        if image.render_style.is_none()
            && matches!(
                creating_context.image.render_style,
                Some(
                    RenderStyle::EnemyUnitCloak
                        | RenderStyle::OwnUnitCloak
                        | RenderStyle::AllyUnitCloak
                        | RenderStyle::OwnUnitCloak2
                        | RenderStyle::OwnUnitCloakDrawOnly
                        | RenderStyle::Crash
                )
            )
        {
            image.render_style = creating_context.image.render_style;
            // TODO(tec27): Seems like there is sometimes data associated with the render styles?
            // Not sure what this does yet, but it seems to be copied from the creator in this case
        }
        // TODO(tec27): Handle hiding the image in some cases?

        commands.entity(image_entity).insert(ConstructImageBundle {
            image,
            iscript,
            loading_anim: LoadingAnimBundle::new(image_id),
            ..default()
        });
        commands
            .entity(creating_context.parent_sprite_entity)
            .add_child(image_entity);
    }
}

pub fn exec_iscripts(
    mut q_images: Query<(Entity, &mut IscriptController, &mut ConstructImage, &Parent)>,
    mut q_sprites: Query<(Entity, &mut ConstructSprite)>,
    mut commands: Commands,
    mut rand: ResMut<LcgRand>,
) {
    for (image_entity, mut controller, mut image, parent) in q_images.iter_mut() {
        if let Ok((sprite_entity, mut sprite)) = q_sprites.get_mut(parent.get()) {
            let context = IscriptExecContext {
                image_entity,
                image: &mut image,
                parent_sprite_entity: sprite_entity,
                parent_sprite: &mut sprite,
                rand: &mut rand,
            };
            controller.exec(context, &mut commands);
        } else {
            warn!("Found image entity {image_entity:?} without a parent sprite");
        }
    }
}
