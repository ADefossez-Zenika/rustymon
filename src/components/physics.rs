use amethyst::ecs::prelude::{Component, VecStorage};
use ncollide2d::{
    math::Vector,
    shape::{Ball, Cuboid},
};
use specs_derive::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Velocity {
    pub direction: Vector<f32>,
    pub speed: f32,
}

impl Velocity {
    /// Create a new velocity with default values.
    pub fn new() -> Self {
        Velocity {
            direction: Vector::new(0.0, 0.0),
            speed: 0.0,
        }
    }

    /// Reset the velocity to its default values.
    pub fn reset(&mut self) {
        self.direction = Vector::new(0.0, 0.0);
        self.speed = 0.0;
    }
}

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
