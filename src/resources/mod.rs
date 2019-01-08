pub struct WorldBounds {
    pub width: f32,
    pub height: f32,
}

impl WorldBounds {
    pub fn new(width: f32, height: f32) -> Self {
        WorldBounds { width, height }
    }
}
