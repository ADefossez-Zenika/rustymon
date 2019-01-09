#[derive(Copy, Clone)]
pub struct WorldBounds {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl WorldBounds {
    /// Create new bounds around origin.
    pub fn new_around_origin(width: f32, height: f32) -> Self {
        WorldBounds {
            left: -width * 0.5,
            right: width * 0.5,
            bottom: -height * 0.5,
            top: height * 0.5,
        }
    }

    pub fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        WorldBounds {
            left,
            right,
            bottom,
            top,
        }
    }
}
