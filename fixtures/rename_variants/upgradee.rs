#[derive(Clone)]
pub enum Enum {
    Apple,
    Banana,
    Orange,
    Grape(u16),
    Melon { size: u8 },
}
