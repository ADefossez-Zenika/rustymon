use crate::{
    animations::{create_singleton_looping_set, HeroAnimationId},
    components::HeroAnimation,
};
use amethyst::{
    animation::AnimationControlSet,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, System, WriteStorage},
    input::InputHandler,
    renderer::SpriteRender,
};

#[derive(Default)]
pub struct HeroMovementSystem;

impl<'a> System<'a> for HeroMovementSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, InputHandler<String, String>>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, HeroAnimation>,
        WriteStorage<'a, AnimationControlSet<HeroAnimationId, SpriteRender>>,
    );

    fn run(
        &mut self,
        (entities, input, mut transforms, mut animations, mut animation_sets): Self::SystemData,
    ) {
        for (entity, transform, animations) in (&entities, &mut transforms, &mut animations).join()
        {
            let left_right_amount = input.axis_value("right_left").unwrap() as f32;
            let up_down_amount = input.axis_value("up_down").unwrap() as f32;

            if left_right_amount != 0.0 {
                transform.translate_x(left_right_amount);
            } else {
                transform.translate_y(up_down_amount);
            }

            let (id, handle) = if left_right_amount > 0.0 {
                &animations.go_right
            } else if left_right_amount < 0.0 {
                &animations.go_left
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
