use std::collections::VecDeque;

use amethyst::ecs::{
    System,
    WriteStorage,
    WriteExpect,
    Join,
    Entities,
    ReadStorage
};

use crate::entities::{
    dead_snake_tracker::*,
    body::*
};

use crate::resources::{
    grid::*
};

pub struct ClearDeadSnakesSystem;

impl<'s> System<'s> for ClearDeadSnakesSystem{
    type SystemData = (
        Entities<'s>,
        WriteExpect<'s, Grid>,
        WriteStorage<'s, DeadSnake>,
        ReadStorage<'s, Body>
    );

    fn run(&mut self, (entities, mut grid, mut dead_snakes, bodies): Self::SystemData) {
        let iterator: VecDeque<_> = (&entities, &mut dead_snakes).join().collect();
        for (e, dead_snake) in iterator{
            let p = dead_snake.tail;
            let b = grid.get_tile(p);
            match b {
                None => {
                    entities.delete(e);
                }
                Some(b) => {
                    let next_position = bodies.get(b).unwrap().next_position;
                    dead_snake.tail = next_position;
                    grid.set_tile(p, None);
                    entities.delete(b);
                    if p == next_position{
                        entities.delete(e);
                    }
                }
            }
        }
    }
}
