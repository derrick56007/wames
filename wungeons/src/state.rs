use crate::components::Rect;

pub struct State {
    pub grid_size: Rect,
}

impl State {
    pub fn new(grid_size: Rect) -> Self {
        Self {
            grid_size: grid_size,
        }
    }
}
