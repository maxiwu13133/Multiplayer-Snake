use crate::*;

#[derive(Debug, Clone, Serialize)]
pub struct SnakeSegment {
    pub x: f64,
    pub y: f64,
}

impl From<&game::Segment> for SnakeSegment {
    fn from(snake_seg: &game::Segment) -> Self {
        Self {
            x: snake_seg.x(),
            y: snake_seg.y(),
        }
    }
}
