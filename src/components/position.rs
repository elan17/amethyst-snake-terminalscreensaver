use std::{
    ops::{
        Add,
        Sub,
        Neg,
        Rem
    }
};
use amethyst::ecs::{Component, VecStorage};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Position (i32, i32);

impl Position{

    pub fn new(x: i32, y:i32) -> Self{
        Position(x, y)
    }

    pub fn set_position(&mut self, new_position: Self){
        *self = new_position;
    }

    pub fn get_coords(&self) -> (i32, i32){
        (self.0, self.1)
    }

}

impl Component for Position{
    type Storage = VecStorage<Self>;
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position (self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Neg for Position{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Position(-self.0, -self.1)
    }
}

impl Sub for Position{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Rem for Position{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Position(self.0.rem_euclid(rhs.0), self.1.rem_euclid(rhs.1))
    }
}
