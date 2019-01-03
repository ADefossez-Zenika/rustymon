use crate::animations::HeroAnimationId;
use amethyst::{
    animation::Animation,
    assets::Handle,
    ecs::prelude::{Component, VecStorage},
    renderer::SpriteRender,
};

pub struct HeroAnimation {
    pub idle: (HeroAnimationId, Handle<Animation<SpriteRender>>),
    pub go_right: (HeroAnimationId, Handle<Animation<SpriteRender>>),
    pub go_left: (HeroAnimationId, Handle<Animation<SpriteRender>>),
    pub go_forward: (HeroAnimationId, Handle<Animation<SpriteRender>>),
    pub go_backward: (HeroAnimationId, Handle<Animation<SpriteRender>>),
    pub current_id: Option<HeroAnimationId>,
}

impl Component for HeroAnimation {
    type Storage = VecStorage<Self>;
}
