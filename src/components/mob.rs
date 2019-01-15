use amethyst::ecs::{Component, Entity, VecStorage};
use specs_derive::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Mob {
    /// The original spawn position of the mob.
    pub spawn: (f32, f32),

    /// The squared distance from its spawn at which the mobs resets its position.
    pub squared_reset_threshold: f32,

    /// Is the mob resetting.
    pub resetting: bool,

    /// The currently targeted entity.
    pub target: Option<Entity>,

    /// The squared distance from which it starts targetting an entity.
    pub squared_target_threshold: f32,
}

impl Mob {
    pub fn new(x: f32, y: f32, reset_threshold: f32, target_threshold: f32) -> Self {
        Mob {
            spawn: (x, y),
            squared_reset_threshold: reset_threshold * reset_threshold,
            resetting: false,
            target: None,
            squared_target_threshold: target_threshold * target_threshold,
        }
    }
}
