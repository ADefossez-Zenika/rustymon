use crate::{animations::SpriteAnimation, systems::HeroMovementSystem};
use amethyst::{
    assets::Processor,
    core::bundle::{Result, SystemBundle},
    ecs::prelude::DispatcherBuilder,
};

pub struct RustymonBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for RustymonBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            HeroMovementSystem::default(),
            "hero_movement",
            &["input_system"],
        );
        builder.add(Processor::<SpriteAnimation>::new(), "", &[]);
        Ok(())
    }
}
