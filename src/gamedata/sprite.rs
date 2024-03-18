use bevy::reflect::Reflect;

#[allow(dead_code)]
#[derive(Clone, Debug, Reflect)]
pub struct BwSprite {
    pub image: u16,
    pub health_bar: Option<u8>,
    pub unknown_0: u8,
    pub visible: u8,
    pub selection_circle: Option<u8>,
    pub selection_circle_offset: Option<u8>,
}
