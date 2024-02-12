use bevy::prelude::*;

use super::asset::MapAsset;

// Useful links for .anim stuff:
// http://www.staredit.net/topic/17217/0/
// http://www.staredit.net/topic/17590/
// https://github.com/neivv/animosity/blob/master/src/anim.rs
// https://github.com/alexpineda/titan-reactor/blob/974f3b30a5a19c14dfcf03f85c82f869342ac76d/src/renderer/image/formats/parse-anim.ts

// Look up unit ID in units.dat, map flingy ID to flingy.dat, map sprite ID to sprites.dat, map
// image ID to images.dat (although I think this is actually unnecessary and you just load that ID
// as main_###.anim ?)

pub fn create_map_sprites(commands: &mut Commands, map: &MapAsset, map_entity: Entity) {
    info!(
        "Creating map sprites, map has {} sprites",
        map.sprites.len()
    );

    // Used for inverting y-coords
    let max_height = (map.height - 1) as f32;

    for (i, sprite) in map.sprites.iter().enumerate() {
        // TODO(tec27): Load real sprite images + use actual sizes
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.2, 0.2),
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    // TODO(tec27): Need to base this on the map's tile size, would probably be
                    // better to write something that keeps track of this sprite in map coords and
                    // manages this transform value
                    ((sprite.x as f32) / 32.0 - map.width as f32 / 2.0) * 64.0,
                    ((max_height - (sprite.y as f32) / 32.0) - map.height as f32 / 2.0) * 64.0,
                    1.0,
                )),
                ..default()
            })
            .insert(Name::new(format!("Sprite #{i}")))
            .set_parent(map_entity);
    }
}

pub fn create_placed_units(commands: &mut Commands, map: &MapAsset, map_entity: Entity) {
    info!(
        "Creating placed units, map has {} placed units",
        map.placed_units.len()
    );

    // Used for inverting y-coords
    let max_height = (map.height - 1) as f32;

    for unit in map.placed_units.iter() {
        // TODO(tec27): use real units with proper behavior and such
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.2, 0.2, 0.8),
                    custom_size: Some(Vec2::new(64.0, 64.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    // TODO(tec27): Need to base this on the map's tile size, would probably be
                    // better to write something that keeps track of this sprite in map coords and
                    // manages this transform value
                    ((unit.x as f32) / 32.0 - map.width as f32 / 2.0) * 64.0,
                    ((max_height - (unit.y as f32) / 32.0) - map.height as f32 / 2.0) * 64.0,
                    1.0,
                )),
                ..default()
            })
            .insert(Name::new(format!("Unit {:x}", unit.unit_id)))
            .set_parent(map_entity);
    }
}
