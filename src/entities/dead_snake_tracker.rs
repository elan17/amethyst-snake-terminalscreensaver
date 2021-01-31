use amethyst::ecs::{
    Component,
    HashMapStorage
};

use crate::components::{
    position::*
};

use crate::entities::{
    head::*
};

pub struct DeadSnake{
    pub tail: Position
}

impl DeadSnake{

    pub fn new(head: &Head) -> Self{
        DeadSnake{
            tail: head.tail
        }
    }
}

impl Component for DeadSnake{
    type Storage = HashMapStorage<Self>;
}
