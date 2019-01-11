use crate::animations::HeroAnimationId;
use amethyst::ecs::prelude::{Component, VecStorage};
use specs_derive::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Hero {
    pub current_animation_id: Option<HeroAnimationId>,
}

impl Hero {
    pub fn new() -> Self {
        Hero {
            current_animation_id: None,
        }
    }
}
