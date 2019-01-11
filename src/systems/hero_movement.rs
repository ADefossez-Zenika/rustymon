use crate::{
    animations::{create_singleton_looping_set, HeroAnimationId},
    components::Hero,
    resources::WorldBounds,
};
use amethyst::{
    animation::{AnimationControlSet, AnimationSet},
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    renderer::SpriteRender,
};
use nalgebra::base::Vector2;

/// Move the hero according to the input.
/// If the hero has animations, also animates him according to its direction.
pub struct HeroMovementSystem;

impl<'a> System<'a> for HeroMovementSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, InputHandler<String, String>>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Hero>,
        ReadStorage<'a, AnimationSet<HeroAnimationId, SpriteRender>>,
        WriteStorage<'a, AnimationControlSet<HeroAnimationId, SpriteRender>>,
        Option<Read<'a, WorldBounds>>,
    );

    fn run(
        &mut self,
        (entities, input, mut transforms, mut heros, animations, mut animation_controls, bounds): Self::SystemData,
    ) {
        for (entity, transform, hero) in (&entities, &mut transforms, &mut heros).join() {
            let left_right_amount = input.axis_value("right_left").unwrap() as f32;
            let up_down_amount = input.axis_value("up_down").unwrap() as f32;

            if left_right_amount != 0.0 || up_down_amount != 0.0 {
                let direction = Vector2::new(left_right_amount, up_down_amount).normalize();
                let translation = transform.translation();
                let mut x = translation.x + direction.x;
                let mut y = translation.y + direction.y;
                if let Some(bounds) = &bounds {
                    x = x.min(bounds.right).max(bounds.left);
                    y = y.min(bounds.top).max(bounds.bottom);
                }
                transform.set_xyz(x, y, 0.0);
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
