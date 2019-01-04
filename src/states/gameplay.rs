use crate::{
    animations::*,
    assets,
    components::{CameraTarget, HeroAnimation},
};
use amethyst::{
    animation::AnimationControlSet,
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::*,
    renderer::{Camera, DisplayConfig, Projection, SpriteRender},
    GameData, SimpleState, StateData,
};

pub struct GameplayState {
    display_config: DisplayConfig,
    idle_animation_handle: Handle<SpriteAnimation>,
    go_right_animation_handle: Handle<SpriteAnimation>,
    go_left_animation_handle: Handle<SpriteAnimation>,
    go_forward_animation_handle: Handle<SpriteAnimation>,
    go_backward_animation_handle: Handle<SpriteAnimation>,
}

impl GameplayState {
    pub fn new(
        display_config: DisplayConfig,
        idle_animation_handle: Handle<SpriteAnimation>,
        go_right_animation_handle: Handle<SpriteAnimation>,
        go_left_animation_handle: Handle<SpriteAnimation>,
        go_forward_animation_handle: Handle<SpriteAnimation>,
        go_backward_animation_handle: Handle<SpriteAnimation>,
    ) -> Self {
        GameplayState {
            display_config,
            idle_animation_handle,
            go_right_animation_handle,
            go_left_animation_handle,
            go_forward_animation_handle,
            go_backward_animation_handle,
        }
    }
}

impl GameplayState {
    fn init_camera(&self, world: &mut World, target: Entity) {
        let (half_width, half_height) = {
            let (width, height) = self.display_config.dimensions.unwrap();
            (width as f32 * 0.5, height as f32 * 0.5)
        };

        let mut transform = Transform::default();
        transform.set_z(1.0);
        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                -half_width,
                half_width,
                -half_height,
                half_height,
            )))
            .with(transform)
            .with(CameraTarget { entity: target })
            .build();
    }

    fn build_hero(&mut self, world: &mut World) -> Entity {
        let texture = assets::load_texture("sprite_sheets/hero.png", world);
        let sprite_sheet = assets::load_sprite_sheet("sprite_sheets/hero.ron", texture, world);

        let idle = assets::load_sprite_render_animation(world, self.idle_animation_handle.clone());
        let go_right =
            assets::load_sprite_render_animation(world, self.go_right_animation_handle.clone());
        let go_left =
            assets::load_sprite_render_animation(world, self.go_left_animation_handle.clone());
        let go_forward =
            assets::load_sprite_render_animation(world, self.go_forward_animation_handle.clone());
        let go_backward =
            assets::load_sprite_render_animation(world, self.go_backward_animation_handle.clone());

        world
            .create_entity()
            .with(AnimationControlSet::<HeroAnimationId, SpriteRender>::default())
            .with(HeroAnimation {
                idle: (HeroAnimationId::Idle, idle),
                go_right: (HeroAnimationId::GoRight, go_right),
                go_left: (HeroAnimationId::GoLeft, go_left),
                go_forward: (HeroAnimationId::GoForward, go_forward),
                go_backward: (HeroAnimationId::GoBackward, go_backward),
                current_id: None,
            })
            .with(SpriteRender {
                sprite_sheet,
                sprite_number: 0,
            })
            .with(Transform::default())
            .build()
    }
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let hero = self.build_hero(data.world);
        self.init_camera(data.world, hero);
    }
}
