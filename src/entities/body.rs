use amethyst::prelude::{
    Builder,
    World,
    WorldExt
};
use amethyst::ecs::{
    Component,
    DenseVecStorage
};
use crate::components::{
    position::Position,
    ascii_representation::{
        AsciiRepresentation,
        Color
    }
};

use crate::resources::grid::*;

pub struct Body{
    pub next_position: Position
}

impl Component for Body{
    type Storage = DenseVecStorage<Self>;
}
