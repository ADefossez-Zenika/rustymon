use crate::states::Instance;
use amethyst::ecs::prelude::{Component, VecStorage};
use ncollide2d::shape::Cuboid;
use specs_derive::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Portal {
    pub instance: Instance,
    pub trigger_zone: Cuboid<f32>,
}
