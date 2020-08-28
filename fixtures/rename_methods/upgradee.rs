pub struct Struct {}

impl Struct {
    pub fn print(&self) {}
}

pub enum Enum {
    None,
}

impl Enum {
    pub fn talk(&self) {}
}

pub union Union {
    pub y: bool,
}

impl Union {
    pub fn eat(&self) {}
}
