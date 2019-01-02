use crate::{animations::SpriteAnimation, assets::load_sprite_render_animation};
use amethyst::{animation::*, assets::Handle, ecs::prelude::World, renderer::SpriteRender};
use serde_derive::Deserialize;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Deserialize)]
pub enum AnimationId {
    Idle,
    GoRight,
    GoLeft,
    GoForward,
    GoBackward,
}

pub fn build_animation_control_set(
    world: &mut World,
    idle_animation_handle: Handle<SpriteAnimation>,
    go_right_animation_handle: Handle<SpriteAnimation>,
    go_left_animation_handle: Handle<SpriteAnimation>,
    go_forward_animation_handle: Handle<SpriteAnimation>,
    go_backward_animation_handle: Handle<SpriteAnimation>,
) -> AnimationControlSet<AnimationId, SpriteRender> {
    let mut animation_control_set = AnimationControlSet::<AnimationId, SpriteRender>::default();
    animation_control_set.add_animation(
        AnimationId::Idle,
        &load_sprite_render_animation(world, idle_animation_handle),
        EndControl::Loop(None),
        1.0,
        AnimationCommand::Start,
    );

    animation_control_set.add_animation(
        AnimationId::GoRight,
        &load_sprite_render_animation(world, go_right_animation_handle),
        EndControl::Loop(None),
        1.0,
        AnimationCommand::Init,
    );

    animation_control_set.add_animation(
        AnimationId::GoLeft,
        &load_sprite_render_animation(world, go_left_animation_handle),
        EndControl::Loop(None),
        1.0,
        AnimationCommand::Init,
    );

    animation_control_set.add_animation(
        AnimationId::GoForward,
        &load_sprite_render_animation(world, go_forward_animation_handle),
        EndControl::Loop(None),
        1.0,
        AnimationCommand::Init,
    );

    animation_control_set.add_animation(
        AnimationId::GoBackward,
        &load_sprite_render_animation(world, go_backward_animation_handle),
        EndControl::Loop(None),
        1.0,
        AnimationCommand::Init,
    );
    animation_control_set
}
