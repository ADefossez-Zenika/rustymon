mod hero;

use amethyst::{
    assets::{Asset, Handle, ProcessingState, Result},
    ecs::prelude::VecStorage,
};
use serde_derive::*;

pub use self::hero::build_animation_control_set;
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
