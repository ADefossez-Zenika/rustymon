use crate::{
    animations::SpriteAnimation,
    systems::{
        CameraTargetingSystem, HeroMovementSystem, MobMovementSystem, MobTargetSystem,
        PhysicsSystem, PortalTriggerSystem,
    },
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
        builder.add(PortalTriggerSystem, "portal", &["hero_movement"]);
        builder.add(MobTargetSystem, "mob_target", &[]);
        // Could have a dependency on 'mob_target' but it seems ok to have one frame latency before starting to follow the target.
        builder.add(MobMovementSystem, "mob_movement", &[]);
        builder.add(Processor::<SpriteAnimation>::new(), "", &[]);
        Ok(())
    }
}
