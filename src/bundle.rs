use crate::{
    animations::SpriteAnimation,
    systems::{
        CameraTargetingSystem, HeroMovementSystem, MobMovementSystem, MobTargetSystem,
        MovementSystem, PhysicsSystem, PortalTriggerSystem,
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
        // Movement
        {
            builder.add(HeroMovementSystem, "hero_movement", &["input_system"]);
            // Could have a dependency on 'mob_target' but it seems ok to have one frame latency before starting to follow the target.
            builder.add(MobMovementSystem, "mob_movement", &[]);
            builder.add(
                MovementSystem,
                "movement",
                &["hero_movement", "mob_movement"],
            )
        }

        builder.add(PhysicsSystem, "physics", &["movement"]);
        builder.add(CameraTargetingSystem, "camera_targeting", &[]);
        builder.add(PortalTriggerSystem, "portal", &[]);
        builder.add(MobTargetSystem, "mob_target", &[]);

        // Processors
        builder.add(Processor::<SpriteAnimation>::new(), "", &[]);
        Ok(())
    }
}
