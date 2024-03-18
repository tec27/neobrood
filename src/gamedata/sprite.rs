use super::BwImage;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BwSprite {
    pub id: u16,
    pub image: &'static BwImage,
    pub health_bar: Option<u8>,
    pub unknown_0: u8,
    pub visible: u8,
    pub selection_circle: Option<u8>,
    pub selection_circle_offset: Option<u8>,
}
