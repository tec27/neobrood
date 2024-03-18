use super::BwSprite;

// TODO(tec27): The types here could probably be made more useful (e.g. some of these are probably
// float or fixed point, some of these could point to newtypes, etc.)
#[derive(Debug, Clone)]
pub struct Flingy {
    pub id: u8,
    pub sprite_id: u16,
    pub speed: u32,
    pub acceleration: u16,
    pub halt_distance: u32,
    pub turn_radius: u8,
    // NOTE(tec27): unused field here is skipped for now (unless we find a use for it)
    pub movement_control: u8,
}

impl Flingy {
    #[inline]
    pub fn sprite(&self) -> &'static BwSprite {
        &super::SPRITES[self.sprite_id as usize]
    }
}
