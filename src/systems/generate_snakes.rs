use rand::prelude::{
    SeedableRng,
    Rng,
    SmallRng
};

use amethyst::ecs::{
    System,
    WriteStorage,
    WriteExpect,
    Entities
};

use crate::entities::{
    head::*
};

use crate::resources::{
    grid::*
};

use crate::components::{
    ascii_representation::*,
    position::*
};

pub struct SnakeGenerator{
    rnd: SmallRng
}

impl SnakeGenerator{

    pub fn new() -> Self{
        SnakeGenerator{
            rnd: SmallRng::from_entropy()
        }
    }
}

impl<'s>System<'s> for SnakeGenerator{
    type SystemData = (
        Entities<'s>,
        WriteExpect<'s, Grid>,
        WriteStorage<'s, Head>,
        WriteStorage<'s, Position>,
        WriteStorage<'s, AsciiRepresentation>
    );

    fn run(&mut self, (entities, mut grid, mut heads, mut positions, mut representations): Self::SystemData) {
        let dim = grid.get_dimensions().get_coords();
        let x = self.rnd.gen_range(0..dim.0);
        let y = self.rnd.gen_range(0..dim.1);
        let color = self.rnd.gen_range(0..20);
        let pos: Position = Position::new(x, y);
        if grid.get_tile(pos) == None{
            let e = entities.build_entity()
                    .with(Head{tail: pos}, &mut heads)
                    .with(pos, &mut positions)
                    .with(AsciiRepresentation::new('0', color), &mut representations)
                    .build();
            grid.set_tile(pos, Some(e));
        }
    }
}
