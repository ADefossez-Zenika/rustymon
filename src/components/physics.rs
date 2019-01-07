use amethyst::ecs::prelude::{Component, VecStorage};
use ncollide2d::{
    math::Vector,
    shape::{Ball, Cuboid},
};
use specs_derive::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Body {
    pub shape: Shape,
    pub dynamic: Dynamic,
}

pub enum Shape {
    Circle { shape: Ball<f32> },
    Box { shape: Cuboid<f32> },
}

#[derive(PartialEq)]
pub enum Dynamic {
    Static,
    Dynamic,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct CollisionMarker {
    pub penetration: Vector<f32>,
}

impl CollisionMarker {
    pub fn new(penetration: Vector<f32>) -> Self {
        CollisionMarker { penetration }
    }
}
