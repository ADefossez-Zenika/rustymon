use crate::{
    animations::{create_singleton_looping_set, HeroAnimationId},
    components::{Hero, Velocity},
};
use amethyst::{
    animation::{AnimationControlSet, AnimationSet},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::SpriteRender,
};
use ncollide2d::math::Vector;

/// Move the hero according to the input.
/// If the hero has animations, also animates him according to its direction.
pub struct HeroMovementSystem;

impl<'a> System<'a> for HeroMovementSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, InputHandler<String, String>>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Hero>,
        ReadStorage<'a, AnimationSet<HeroAnimationId, SpriteRender>>,
        WriteStorage<'a, AnimationControlSet<HeroAnimationId, SpriteRender>>,
    );

    fn run(
        &mut self,
        (entities, input, mut velocities, mut heros, animations, mut animation_controls): Self::SystemData,
    ) {
        for (entity, velocity, hero) in (&entities, &mut velocities, &mut heros).join() {
            velocity.reset();

            let left_right_amount = input.axis_value("right_left").unwrap() as f32;
            let up_down_amount = input.axis_value("up_down").unwrap() as f32;

            if left_right_amount != 0.0 || up_down_amount != 0.0 {
                velocity.direction = Vector::new(left_right_amount, up_down_amount).normalize();
                velocity.speed = 1.0;
            }

            if let Some(animations) = animations.get(entity) {
                let id = compute_animation_id(left_right_amount, up_down_amount);

                if hero.current_animation_id.is_none() || hero.current_animation_id.unwrap() != id {
                    let handle = animations.get(&id).unwrap();
                    let control_set = create_singleton_looping_set(id, handle);
                    animation_controls.insert(entity, control_set).unwrap();
                    hero.current_animation_id.replace(id);
                }
            }
        }
    }
}

/// Compute the current animation if from the direction of the hero.
fn compute_animation_id(left_right_amount: f32, up_down_amount: f32) -> HeroAnimationId {
    if left_right_amount > 0.0 {
        if up_down_amount > 0.0 {
            HeroAnimationId::GoRightForward
        } else if up_down_amount < 0.0 {
            HeroAnimationId::GoRightBackward
        } else {
            HeroAnimationId::GoRight
        }
    } else if left_right_amount < 0.0 {
        if up_down_amount > 0.0 {
            HeroAnimationId::GoLeftForward
        } else if up_down_amount < 0.0 {
            HeroAnimationId::GoLeftBackward
        } else {
            HeroAnimationId::GoLeft
        }
    } else if up_down_amount > 0.0 {
        HeroAnimationId::GoForward
    } else if up_down_amount < 0.0 {
        HeroAnimationId::GoBackward
    } else {
        HeroAnimationId::Idle
    }
}
