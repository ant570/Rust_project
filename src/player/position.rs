#[derive(Debug)]
pub struct Position2 {
    pub x: f32,
    pub y: f32,
}

impl Position2 {
    pub fn new(x: f32, y: f32) -> Self {
        Position2 { x, y }
    }

    pub fn _to_str(&self) {
        println!("{{ x: {}, y: {} }}", self.x, self.y);
    }
}
