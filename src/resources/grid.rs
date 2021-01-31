use amethyst::ecs::prelude::Entity;
use crate::components::position::Position;

pub type Dimensions = Position;

pub struct Grid{
    grid: Vec<Vec<Option<Entity>>>,
    dim: Dimensions
}

impl Grid{

    pub fn new(d: Dimensions) -> Self{
        let (x, y) = d.get_coords();
        Grid{
            grid: vec![vec![None; y as usize]; x as usize],
            dim: d
        }
    }

    pub fn get_tile(&self, position: Position) -> Option<Entity>{
        let p = (position % self.dim).get_coords();
        self.grid[p.0 as usize][p.1 as usize]
    }

    pub fn set_tile(&mut self, position: Position, e: Option<Entity>){
        let p = (position % self.dim).get_coords();
        self.grid[p.0 as usize][p.1 as usize] = e
    }

    pub fn get_dimensions(&self) -> Dimensions{
        self.dim
    }
}
