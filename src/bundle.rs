use crate::{
    animations::SpriteAnimation,
    systems::{CameraTargetingSystem, HeroMovementSystem, PhysicsSystem},
};
use amethyst::{
    assets::Processor,
    core::bundle::{Result, SystemBundle},
    ecs::prelude::DispatcherBuilder,
};

pub struct RustymonBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for RustymonBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(HeroMovementSystem, "hero_movement", &["input_system"]);
        builder.add(PhysicsSystem, "physics", &["hero_movement"]);
        builder.add(
            CameraTargetingSystem,
            "camera_targeting",
            &["hero_movement"],
        );
        builder.add(Processor::<SpriteAnimation>::new(), "", &[]);
        Ok(())
    }
}
