use crate::components::{Hero, Mob};
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
    );

    fn run(&mut self, (entities, mut mobs, heros, transforms): Self::SystemData) {
        for (mob, m_transform) in (&mut mobs, &transforms).join() {
            if mob.resetting {
                continue;
            }

            let m_position = get_position(m_transform);
            for (entity, _, h_transform) in (&entities, &heros, &transforms).join() {
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

fn get_position(t: &Transform) -> Vector<f32> {
    let translation = t.translation();
    Vector::new(translation.x, translation.y)
}

/// Handle mob movement.
/// Follow currently targetted entity.
/// Get back to spawn when too far.
pub struct MobMovementSystem;

impl<'a> System<'a> for MobMovementSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Mob>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (entities, mut mobs, mut transforms): Self::SystemData) {
        for (entity, mob) in (&entities, &mut mobs).join() {
            // Go toward target
            if let Some(target) = mob.target {
                let target_position = get_position(transforms.get(target).unwrap());

                let mob_transform = transforms.get_mut(entity).unwrap();
                let mob_position = get_position(mob_transform);

                let direction = (target_position - mob_position).normalize();
                mob_transform.translate_x(direction.x * MOB_SPEED);
                mob_transform.translate_y(direction.y * MOB_SPEED);
            }

            // Check if should reset or stop resetting
            let mob_transform = transforms.get_mut(entity).unwrap();
            let mob_position = get_position(mob_transform);
            let spawn = Vector::new(mob.spawn.0, mob.spawn.1);
            let direction = spawn - mob_position;
            let squared_distance = direction.norm_squared();
            if squared_distance > mob.squared_reset_threshold {
                mob.target = None;
                mob.resetting = true;
            }

            // If resetting, go toward spawn and stop when close
            if mob.resetting && squared_distance > 1.0 {
                let direction = direction.normalize();
                mob_transform.translate_x(direction.x * MOB_SPEED);
                mob_transform.translate_y(direction.y * MOB_SPEED);
            } else if mob.resetting {
                mob.resetting = false;
            }
        }
    }
}
