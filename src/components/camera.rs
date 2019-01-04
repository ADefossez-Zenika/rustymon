use amethyst::ecs::prelude::{Component, Entity, VecStorage};
use specs_derive::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct CameraTarget {
    pub entity: Entity,
}
