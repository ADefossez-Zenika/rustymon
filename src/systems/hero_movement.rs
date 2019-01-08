use crate::{
    animations::{create_singleton_looping_set, HeroAnimationId},
    components::HeroAnimation,
    resources::WorldBounds,
};
use amethyst::{
    animation::AnimationControlSet,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, System, WriteStorage},
    input::InputHandler,
    renderer::SpriteRender,
};
use nalgebra::base::Vector2;

pub struct HeroMovementSystem;

impl<'a> System<'a> for HeroMovementSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, InputHandler<String, String>>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, HeroAnimation>,
        WriteStorage<'a, AnimationControlSet<HeroAnimationId, SpriteRender>>,
        Option<Read<'a, WorldBounds>>,
    );

    fn run(
        &mut self,
        (entities, input, mut transforms, mut animations, mut animation_sets, bounds): Self::SystemData,
    ) {
        for (entity, transform, animations) in (&entities, &mut transforms, &mut animations).join()
        {
            let left_right_amount = input.axis_value("right_left").unwrap() as f32;
            let up_down_amount = input.axis_value("up_down").unwrap() as f32;

            if left_right_amount != 0.0 || up_down_amount != 0.0 {
                let direction = Vector2::new(left_right_amount, up_down_amount).normalize();
                let translation = transform.translation();
                let mut x = translation.x + direction.x;
                let mut y = translation.y + direction.y;
                if let Some(bounds) = &bounds {
                    x = x.min(bounds.width * 0.5).max(-bounds.width * 0.5);
                    y = y.min(bounds.height * 0.5).max(-bounds.height * 0.5);
                }
                transform.set_xyz(x, y, 0.0);
            }

            let (id, handle) = if left_right_amount > 0.0 {
                if up_down_amount > 0.0 {
                    &animations.go_right_forward
                } else if up_down_amount < 0.0 {
                    &animations.go_right_backward
                } else {
                    &animations.go_right
                }
            } else if left_right_amount < 0.0 {
                if up_down_amount > 0.0 {
                    &animations.go_left_forward
                } else if up_down_amount < 0.0 {
                    &animations.go_left_backward
                } else {
                    &animations.go_left
                }
            } else if up_down_amount > 0.0 {
                &animations.go_forward
            } else if up_down_amount < 0.0 {
                &animations.go_backward
            } else {
                &animations.idle
            };

            if animations.current_id.is_none() || animations.current_id.unwrap() != *id {
                let control_set = create_singleton_looping_set(*id, handle);
                animation_sets.insert(entity, control_set).unwrap();
                animations.current_id = Some(*id);
            }
        }
    }
}
