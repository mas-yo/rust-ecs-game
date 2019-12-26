use quicksilver::geom::Vector;

#[derive(Default)]
pub(crate) struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

pub(crate) type Velocity = Vector;

pub(crate) type Position = Vector;

#[derive(Default)]
pub(crate) struct CharacterView {
    pub position: Vector,
    pub direction: f32,
    pub radius: f32,
}
