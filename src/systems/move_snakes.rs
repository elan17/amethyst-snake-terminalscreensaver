use crate::{
    entities::{
        head::*,
        body::*,
        dead_snake_tracker::*
    },
    resources::{
        grid::*
    },
    components::{
        position::*,
        ascii_representation::*
    }
};

use rand::{
    SeedableRng,
    Rng,
    rngs::SmallRng
};

use amethyst::ecs::{
    System,
    WriteStorage,
    WriteExpect,
    Join,
    Entities,
    Entity
};

use std::{
    collections::{
        VecDeque
    }
};


pub struct MoveSnakes{
    rnd: SmallRng,
    allowed_movements: Vec<Position>
}

impl <'s> System<'s> for MoveSnakes{
    type SystemData = (
        Entities<'s>,
        WriteExpect<'s, Grid>,
        WriteStorage<'s, Position>,
        WriteStorage<'s, AsciiRepresentation>,
        WriteStorage<'s, Body>,
        WriteStorage<'s, Head>,
        WriteStorage<'s, DeadSnake>
    );

    fn run(&mut self, (entities, mut grid, mut positions, mut representations, mut bodies, mut heads, mut dead_snakes): Self::SystemData) {
        let iterator: VecDeque<_> = {
            (&entities, &positions, &heads, &representations)
                .join()
                .map(|(e, p, h, r)| (e, *p, *h, *r))
                .collect()
        };
        for (e, pos, head, repre) in iterator {
            let mut valid_positions = Vec::with_capacity(self.allowed_movements.len());
            for x in self.allowed_movements.iter(){
                let p: Position =  pos + *x;
                let t = (&grid).get_tile(p);
                if t == None{
                    valid_positions.push(p)
                }
            }
            if valid_positions.len() != 0 {
                let index: usize = self.rnd.gen_range(0..valid_positions.len());
                let new_pos: Position = valid_positions[index];
                Self::move_snake( &entities, &mut representations, &mut positions
                                , &mut bodies, repre, pos, new_pos, &mut grid, e);
            }
            else{
                Self::kill_snake( &mut bodies, &mut heads, &mut representations
                                , &mut dead_snakes, e, pos, repre.get_color(), head);
            }
        }
    }
}

impl MoveSnakes{
    pub fn new() -> Self{
        MoveSnakes{
            rnd: SmallRng::from_entropy(),
            allowed_movements: vec![ Position::new( 1,  0)
                                   , Position::new(-1,  0)
                                   , Position::new( 0, -1)
                                   , Position::new( 0,  1)]
        }
    }

    pub fn move_snake( entities: &Entities
                     , representations: &mut WriteStorage<AsciiRepresentation>
                     , positions: &mut WriteStorage<Position>
                     , bodies: &mut WriteStorage<Body>
                     , repre: AsciiRepresentation
                     , pos: Position
                     , new_pos: Position
                     , grid: &mut Grid
                     , e: Entity){
        let be = entities.build_entity()
                    .with(Body{next_position: new_pos}, bodies)
                    .with(pos, positions)
                    .with(AsciiRepresentation::new('#', repre.get_color()), representations)
                    .build();
        let jorl = positions.get_mut(e).unwrap();
        *jorl = new_pos;
        grid.set_tile(pos, Some(be));
        grid.set_tile(new_pos, Some(e));
    }

    pub fn kill_snake( bodies: &mut WriteStorage<Body>
                     , heads: &mut WriteStorage<Head>
                     , representations: &mut WriteStorage<AsciiRepresentation>
                     , dead_snakes: &mut WriteStorage<DeadSnake>
                     , e: Entity
                     , pos: Position
                     , color: Color
                     , head: Head){
        bodies.insert(e, Body{next_position: pos});
        heads.remove(e);
        representations.insert(e, AsciiRepresentation::new('#', color));
        dead_snakes.insert(e, DeadSnake::new(&head));
    }
}
