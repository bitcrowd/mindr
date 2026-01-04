#[derive(Copy, PartialEq, Clone, Debug)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RelativeLocation {
    Top,
    Bottom,
    Left,
    Right,
    Center,
}
