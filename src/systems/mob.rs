use crate::components::{Active, Hero, Mob, Velocity};
use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Join, ReadStorage, System, WriteStorage},
};
use ncollide2d::math::Vector;

const MOB_SPEED: f32 = 0.8;

/// Acquire close target.
pub struct MobTargetSystem;

impl<'a> System<'a> for MobTargetSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Mob>,
        ReadStorage<'a, Hero>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Active>,
    );

    fn run(&mut self, (entities, mut mobs, heros, transforms, actives): Self::SystemData) {
        for (mob, m_transform, _) in (&mut mobs, &transforms, &actives).join() {
            if mob.resetting {
                continue;
            }

            let m_position = get_position(m_transform);
            for (entity, _, h_transform, _) in (&entities, &heros, &transforms, &actives).join() {
                // check target theshold
                let h_position = get_position(h_transform);
                let squared_distance = (m_position - h_position).norm_squared();
                if squared_distance < mob.squared_target_threshold {
                    mob.target.replace(entity);
                }
            }
        }
    }
}

/// Handle mob movement.
/// Follow currently targetted entity.
/// Get back to spawn when too far.
pub struct MobMovementSystem;

impl<'a> System<'a> for MobMovementSystem {
    type SystemData = (
        WriteStorage<'a, Mob>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Active>,
    );

    fn run(&mut self, (mut mobs, transforms, mut velocities, actives): Self::SystemData) {
        for (mob, transform, velocity, _) in
            (&mut mobs, &transforms, &mut velocities, &actives).join()
        {
            //Reset velocity
            velocity.reset();

            // Go toward target
            if let Some(target) = mob.target {
                if actives.get(target).is_some() {
                    let target_position = get_position(transforms.get(target).unwrap());
                    let mob_position = get_position(transform);

                    velocity.direction = (target_position - mob_position).normalize();
                    velocity.speed = MOB_SPEED;
                }
            }

            // Check if should reset or stop resetting
            let mob_position = get_position(transform);
            let spawn = Vector::new(mob.spawn.0, mob.spawn.1);
            let direction = spawn - mob_position;
            let squared_distance = direction.norm_squared();
            if squared_distance > mob.squared_reset_threshold {
                mob.target = None;
                mob.resetting = true;
            }

            // If resetting, go toward spawn and stop when close
            if mob.resetting && squared_distance > 1.0 {
                velocity.direction = direction.normalize();
                velocity.speed = MOB_SPEED;
            } else if mob.resetting {
                mob.resetting = false;
            }
        }
    }
}

/// Get the position as a Vector from a `Tranform`.
fn get_position(t: &Transform) -> Vector<f32> {
    let translation = t.translation();
    Vector::new(translation.x, translation.y)
}
