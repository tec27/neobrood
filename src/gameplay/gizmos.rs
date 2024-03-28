use bevy::prelude::*;

use crate::{
    gamedata::ConstructTypeId,
    maps::{
        game_map::{GameMap, GameMapSize, LOGIC_TILE_SIZE},
        position::Position,
    },
    settings::GameSettings,
};

#[derive(Reflect, GizmoConfigGroup)]
pub struct ConstructGizmos {
    pub bounds: Option<Color>,
}

impl Default for ConstructGizmos {
    fn default() -> Self {
        Self {
            bounds: Some(Color::RED),
        }
    }
}

pub fn show_construct_gizmos(
    mut gizmos: Gizmos<ConstructGizmos>,
    store: Res<GizmoConfigStore>,
    constructs: Query<(&ConstructTypeId, &Position)>,
    map: Query<&GameMapSize, With<GameMap>>,
    settings: Res<GameSettings>,
) {
    let config = store.config::<ConstructGizmos>().1;
    if let Some(color) = config.bounds {
        let Ok(map_size) = map.get_single() else {
            // Can't convert the positions without a map size
            return;
        };
        let half_map_size = Vec2::from(map_size) / 2.0;
        let tile_size = settings.asset_quality.tile_size();
        let half_tile_adjustment = Vec2::new(tile_size.x / 2.0, tile_size.y / 2.0);

        for (c, pos) in constructs.iter() {
            let rect = c.def().bounds.at_pos(pos.into());

            let convert_point = |x: i32, y: i32| {
                let mut point = Vec2::new(x as f32, y as f32) / (LOGIC_TILE_SIZE as f32);
                point.y = map_size.height as f32 - point.y;
                point = (point - half_map_size) * tile_size - half_tile_adjustment;
                point
            };
            // NOTE(tec27): We swap the y's used here because the smaller one will become larger
            let min = convert_point(rect.min.x, rect.max.y);
            let max = convert_point(rect.max.x, rect.min.y);

            let points = [
                min,
                Vec2::new(max.x, min.y),
                max,
                Vec2::new(min.x, max.y),
                min,
            ];
            gizmos.linestrip_2d(points, color);
        }
    }
}
