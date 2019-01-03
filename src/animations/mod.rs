mod hero;

use amethyst::{
    animation::{Animation, AnimationCommand, AnimationControlSet, AnimationSampling, EndControl},
    assets::{Asset, Handle, ProcessingState, Result},
    ecs::prelude::VecStorage,
};
use serde_derive::*;

pub use self::hero::AnimationId as HeroAnimationId;

#[derive(Serialize, Deserialize)]
pub struct SpriteAnimation {
    pub key_frames: Vec<SpriteAnimationKeyFrame>,
}

#[derive(Serialize, Deserialize)]
pub struct SpriteAnimationKeyFrame {
    pub time: f32,
    pub sprite_index: usize,
}

impl Asset for SpriteAnimation {
    const NAME: &'static str = "rustymon::SpriteAnimation";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<SpriteAnimation> for Result<ProcessingState<SpriteAnimation>> {
    fn from(sprite_animation: SpriteAnimation) -> Self {
        Ok(ProcessingState::Loaded(sprite_animation))
    }
}

pub fn create_singleton_looping_set<I, T>(
    id: I,
    animation: &Handle<Animation<T>>,
) -> AnimationControlSet<I, T>
where
    I: PartialEq,
    T: AnimationSampling,
{
    let mut control_set = AnimationControlSet::<I, T>::default();
    control_set.add_animation(
        id,
        animation,
        EndControl::Loop(None),
        1.0,
        AnimationCommand::Start,
    );
    control_set
}
