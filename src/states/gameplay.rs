use crate::{animations::*, assets};
use amethyst::{
    assets::*,
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
    fn init_camera(&self, world: &mut World) {
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
            .build();
    }

    fn init_hero(&mut self, world: &mut World) {
        let texture = assets::load_texture("sprite_sheets/hero.png", world);
        let sprite_sheet = assets::load_sprite_sheet("sprite_sheets/hero.ron", texture, world);
        let control_set = build_animation_control_set(
            world,
            self.idle_animation_handle.clone(),
            self.go_right_animation_handle.clone(),
            self.go_left_animation_handle.clone(),
            self.go_forward_animation_handle.clone(),
            self.go_backward_animation_handle.clone(),
        );

        world
            .create_entity()
            .with(control_set)
            .with(SpriteRender {
                sprite_sheet,
                sprite_number: 0,
            })
            .with(Transform::default())
            .build();
    }
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        self.init_camera(data.world);
        self.init_hero(data.world);
    }
}
