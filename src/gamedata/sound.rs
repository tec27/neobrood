#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BwSound {
    pub id: u16,
    pub file: &'static str,
    priority: u8,
    flags: u8,
    race: u16,
    min_volume: u8,
}
