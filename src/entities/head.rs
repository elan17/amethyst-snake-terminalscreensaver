use amethyst::prelude::{
    Builder,
    World,
    WorldExt
};
use amethyst::ecs::{
    Component,
    HashMapStorage
};
use crate::components::{
    position::Position,
    ascii_representation::{
        AsciiRepresentation,
        Color
    }
};

use crate::resources::grid::*;

#[derive(Clone, Copy)]
pub struct Head{
    pub tail: Position
}

impl Component for Head{
    type Storage = HashMapStorage<Self>;
}

pub fn initialise_snake(world: &mut World, p: Position, c: Color){
    let comp = Head{tail: p};
    let e = world
                .create_entity()
                .with(comp)
                .with(p)
                .with(AsciiRepresentation::new('0', c))
                .build();
    let mut grid = world.fetch_mut::<Grid>();
    grid.set_tile(p, Some(e))
}
