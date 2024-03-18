// NOTE(tec27): This lint is way too sensitive for typical bevy queries, and I think is easy enough
// to catch in reviews anyway
#![allow(clippy::type_complexity)]

pub mod asset_packs;
pub mod camera;
pub mod constructs;
pub mod ecs;
pub mod gamedata;
pub mod gameplay;
pub mod main_menu;
pub mod maps;
pub mod players;
pub mod races;
pub mod random;
pub mod render;
pub mod selection;
pub mod states;
