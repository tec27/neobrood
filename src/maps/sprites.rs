use bevy::prelude::*;

use crate::{
    gamedata::{BwGameData, LoadingAnim},
    render::ysort::YSort,
};

use super::asset::MapAsset;

pub fn create_map_sprites(
    commands: &mut Commands,
    map: &MapAsset,
    map_entity: Entity,
    game_data: &BwGameData,
    asset_server: &Res<AssetServer>,
) {
    info!(
        "Creating map sprites, map has {} sprites",
        map.sprites.len()
    );

    // Used for inverting y-coords
    let max_height = (map.height - 1) as f32;

    for (i, sprite) in map.sprites.iter().enumerate() {
        let image_id = game_data
            .sprites
            .image
            .get(sprite.id as usize)
            .copied()
            .unwrap_or_else(|| {
                warn!(
                    "Encountered Sprite {} which isn't a valid ID, using placeholder sprite",
                    sprite.id
                );
                0
            });

        commands
            .spawn(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    // TODO(tec27): Need to base this on the map's tile size, would probably be
                    // better to write something that keeps track of this sprite in map coords and
                    // manages this transform value
                    ((sprite.x as f32) / 32.0 - map.width as f32 / 2.0) * 64.0 - 32.0,
                    ((max_height - (sprite.y as f32) / 32.0) - map.height as f32 / 2.0) * 64.0
                        + 32.0,
                    1.0,
                )),
                ..default()
            })
            .insert(YSort(2.0))
            .insert(Name::new(format!("Sprite #{i}")))
            .with_children(|builder| {
                builder.spawn(LoadingAnim {
                    // FIXME: generate a path based on current settings
                    handle: asset_server
                        .load(format!("casc-extracted/HD2/anim/main_{image_id:03}.anim")),
                });
            })
            .set_parent(map_entity);
    }
}

pub fn create_placed_units(
    commands: &mut Commands,
    map: &MapAsset,
    map_entity: Entity,
    game_data: &BwGameData,
    asset_server: &Res<AssetServer>,
) {
    info!(
        "Creating placed units, map has {} placed units",
        map.placed_units.len()
    );

    // Used for inverting y-coords
    let max_height = (map.height - 1) as f32;

    for unit in map.placed_units.iter() {
        let flingy_id = game_data
            .units
            .flingy
            .get(unit.unit_id as usize)
            .copied()
            .unwrap_or_else(|| {
                warn!(
                    "Encountered Unit {} which isn't a valid ID, using placeholder flingy",
                    unit.unit_id
                );
                0
            });
        let sprite_id = game_data
            .flingy
            .sprite
            .get(flingy_id as usize)
            .copied()
            .unwrap_or_else(|| {
                warn!(
                    "Encountered Flingy {} which isn't a valid ID, using placeholder sprite",
                    flingy_id
                );
                0
            });
        let image_id = game_data
            .sprites
            .image
            .get(sprite_id as usize)
            .copied()
            .unwrap_or_else(|| {
                warn!(
                    "Encountered Sprite {} which isn't a valid ID, using placeholder sprite",
                    sprite_id
                );
                0
            });

        commands
            .spawn(SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    // TODO(tec27): Need to base this on the map's tile size, would probably be
                    // better to write something that keeps track of this sprite in map coords and
                    // manages this transform value
                    ((unit.x as f32) / 32.0 - map.width as f32 / 2.0) * 64.0 - 32.0,
                    ((max_height - (unit.y as f32) / 32.0) - map.height as f32 / 2.0) * 64.0 + 32.0,
                    1.0,
                )),
                ..default()
            })
            .insert(YSort(2.0))
            .insert(Name::new(format!("Unit {:x}", unit.unit_id)))
            .with_children(|builder| {
                builder.spawn(LoadingAnim {
                    // FIXME: generate a path based on current settings
                    handle: asset_server
                        .load(format!("casc-extracted/HD2/anim/main_{image_id:03}.anim")),
                });
            })
            .set_parent(map_entity);
    }
}
