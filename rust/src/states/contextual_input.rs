#[derive(Debug, Clone, Copy)]
pub enum Direction1D {
    Right,
    Left,
}

impl Into<Direction2D> for Direction1D {
    fn into(self) -> Direction2D {
        match self {
            Direction1D::Left => Direction2D::Left,
            Direction1D::Right => Direction2D::Right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction2D {
    Right,
    Up,
    Left,
    Down,
}

impl Direction2D {
    pub fn to_1d(self) -> Direction1D {
        match self {
            Direction2D::Right => Direction1D::Right,
            Direction2D::Up => Direction1D::Left,
            Direction2D::Left => Direction1D::Left,
            Direction2D::Down => Direction1D::Right,
        }
    }
}

pub enum ContextualInput {
    StateAdded,
    StateRemoved,

    MenuTab(Direction1D),
    MenuMove(Direction2D),
    MenuSelect,
    MenuCancel,
}
