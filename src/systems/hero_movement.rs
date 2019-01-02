use crate::animations::HeroAnimationId;
use amethyst::{
    animation::AnimationControlSet, ecs::prelude::*, input::InputHandler, renderer::SpriteRender,
};

pub struct HeroMovementSystem {
    current_animation_id: HeroAnimationId,
}

impl Default for HeroMovementSystem {
    fn default() -> Self {
        HeroMovementSystem {
            current_animation_id: HeroAnimationId::Idle,
        }
    }
}

impl HeroMovementSystem {
    fn toggle_animation(
        &mut self,
        id: HeroAnimationId,
        control_set: &mut AnimationControlSet<HeroAnimationId, SpriteRender>,
    ) {
        control_set.pause(self.current_animation_id);
        control_set.start(id);
        self.current_animation_id = id;
    }
}

impl<'a> System<'a> for HeroMovementSystem {
    type SystemData = (
        Read<'a, InputHandler<String, String>>,
        WriteStorage<'a, AnimationControlSet<HeroAnimationId, SpriteRender>>,
    );

    fn run(&mut self, (input, mut animation_sets): Self::SystemData) {
        for animation_set in (&mut animation_sets).join() {
            let next_animation_id = if let Some(right_left_amount) = input.axis_value("right_left")
            {
                if right_left_amount > 0.0 {
                    HeroAnimationId::GoRight
                } else if right_left_amount < 0.0 {
                    HeroAnimationId::GoLeft
                } else {
                    HeroAnimationId::Idle
                }
            } else {
                HeroAnimationId::Idle
            };
            self.toggle_animation(next_animation_id, animation_set);
        }
    }
}
