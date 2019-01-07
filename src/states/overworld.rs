use crate::{
    animations::*,
    assets,
    components::{Body, CameraTarget, Dynamic, HeroAnimation, Shape},
};

use amethyst::{
    animation::{Animation, AnimationControlSet},
    assets::{Handle, ProgressCounter},
    core::transform::Transform,
    ecs::prelude::*,
    renderer::{Camera, DisplayConfig, Projection, SpriteRender},
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};

use ncollide2d::{
    math::Vector,
    shape::{Ball, Cuboid},
};

pub struct OverworldState {
    display_config: DisplayConfig,
    idle_animation_handle: Handle<Animation<SpriteRender>>,
    go_right_animation_handle: Handle<Animation<SpriteRender>>,
    go_left_animation_handle: Handle<Animation<SpriteRender>>,
    go_forward_animation_handle: Handle<Animation<SpriteRender>>,
    go_backward_animation_handle: Handle<Animation<SpriteRender>>,
    go_right_forward_animation_handle: Handle<Animation<SpriteRender>>,
    go_right_backward_animation_handle: Handle<Animation<SpriteRender>>,
    go_left_backward_animation_handle: Handle<Animation<SpriteRender>>,
    go_left_forward_animation_handle: Handle<Animation<SpriteRender>>,
}

impl OverworldState {
    pub fn new(
        display_config: DisplayConfig,
        idle_animation_handle: Handle<Animation<SpriteRender>>,
        go_right_animation_handle: Handle<Animation<SpriteRender>>,
        go_left_animation_handle: Handle<Animation<SpriteRender>>,
        go_forward_animation_handle: Handle<Animation<SpriteRender>>,
        go_backward_animation_handle: Handle<Animation<SpriteRender>>,
        go_right_forward_animation_handle: Handle<Animation<SpriteRender>>,
        go_right_backward_animation_handle: Handle<Animation<SpriteRender>>,
        go_left_backward_animation_handle: Handle<Animation<SpriteRender>>,
        go_left_forward_animation_handle: Handle<Animation<SpriteRender>>,
    ) -> Self {
        OverworldState {
            display_config,
            idle_animation_handle,
            go_right_animation_handle,
            go_left_animation_handle,
            go_forward_animation_handle,
            go_backward_animation_handle,
            go_right_forward_animation_handle,
            go_right_backward_animation_handle,
            go_left_backward_animation_handle,
            go_left_forward_animation_handle,
        }
    }
}

impl OverworldState {
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

        world
            .create_entity()
            .with(AnimationControlSet::<HeroAnimationId, SpriteRender>::default())
            .with(HeroAnimation {
                idle: (HeroAnimationId::Idle, self.idle_animation_handle.clone()),
                go_right: (
                    HeroAnimationId::GoRight,
                    self.go_right_animation_handle.clone(),
                ),
                go_left: (
                    HeroAnimationId::GoLeft,
                    self.go_left_animation_handle.clone(),
                ),
                go_forward: (
                    HeroAnimationId::GoForward,
                    self.go_forward_animation_handle.clone(),
                ),
                go_backward: (
                    HeroAnimationId::GoBackward,
                    self.go_backward_animation_handle.clone(),
                ),
                go_right_forward: (
                    HeroAnimationId::GoRightForward,
                    self.go_right_forward_animation_handle.clone(),
                ),
                go_right_backward: (
                    HeroAnimationId::GoRightBackward,
                    self.go_right_backward_animation_handle.clone(),
                ),
                go_left_backward: (
                    HeroAnimationId::GoLeftBackward,
                    self.go_left_backward_animation_handle.clone(),
                ),
                go_left_forward: (
                    HeroAnimationId::GoLeftForward,
                    self.go_left_forward_animation_handle.clone(),
                ),
                current_id: None,
            })
            .with(SpriteRender {
                sprite_sheet,
                sprite_number: 0,
            })
            .with(Transform::default())
            .with(Body {
                shape: Shape::Circle {
                    shape: Ball::new(16.0),
                },
                dynamic: Dynamic::Dynamic,
            })
            .build()
    }

    fn build_building(&mut self, x: f32, y: f32, world: &mut World) {
        let texture = assets::load_texture("sprite_sheets/buildings.png", world);
        let sprite_sheet = assets::load_sprite_sheet("sprite_sheets/buildings.ron", texture, world);

        let mut transform = Transform::default();
        transform.set_xyz(x, y, 0.0);

        world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet,
                sprite_number: 0,
            })
            .with(transform)
            .with(Body {
                shape: Shape::Box {
                    shape: Cuboid::new(Vector::new(32.0, 16.0)),
                },
                dynamic: Dynamic::Static,
            })
            .build();
    }
}

impl SimpleState for OverworldState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let hero = self.build_hero(data.world);
        self.init_camera(data.world, hero);
        self.build_building(100.0, 100.0, data.world);
    }
}

pub struct LoadingState {
    display_config: DisplayConfig,
    progress: ProgressCounter,
    idle_animation_handle: Option<Handle<SpriteAnimation>>,
    go_right_animation_handle: Option<Handle<SpriteAnimation>>,
    go_left_animation_handle: Option<Handle<SpriteAnimation>>,
    go_forward_animation_handle: Option<Handle<SpriteAnimation>>,
    go_backward_animation_handle: Option<Handle<SpriteAnimation>>,
    go_right_forward_animation_handle: Option<Handle<SpriteAnimation>>,
    go_right_backward_animation_handle: Option<Handle<SpriteAnimation>>,
    go_left_backward_animation_handle: Option<Handle<SpriteAnimation>>,
    go_left_forward_animation_handle: Option<Handle<SpriteAnimation>>,
}

impl LoadingState {
    pub fn new(display_config: DisplayConfig) -> Self {
        LoadingState {
            display_config,
            progress: ProgressCounter::new(),
            idle_animation_handle: None,
            go_right_animation_handle: None,
            go_left_animation_handle: None,
            go_forward_animation_handle: None,
            go_backward_animation_handle: None,
            go_right_forward_animation_handle: None,
            go_right_backward_animation_handle: None,
            go_left_backward_animation_handle: None,
            go_left_forward_animation_handle: None,
        }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        self.idle_animation_handle = Some(assets::load_sprite_animation(
            "animations/hero/idle.ron",
            &mut self.progress,
            data.world,
        ));
        self.go_right_animation_handle = Some(assets::load_sprite_animation(
            "animations/hero/go_right.ron",
            &mut self.progress,
            data.world,
        ));
        self.go_left_animation_handle = Some(assets::load_sprite_animation(
            "animations/hero/go_left.ron",
            &mut self.progress,
            data.world,
        ));
        self.go_forward_animation_handle = Some(assets::load_sprite_animation(
            "animations/hero/go_forward.ron",
            &mut self.progress,
            data.world,
        ));
        self.go_backward_animation_handle = Some(assets::load_sprite_animation(
            "animations/hero/go_backward.ron",
            &mut self.progress,
            data.world,
        ));
        self.go_right_forward_animation_handle = Some(assets::load_sprite_animation(
            "animations/hero/go_right_forward.ron",
            &mut self.progress,
            data.world,
        ));
        self.go_right_backward_animation_handle = Some(assets::load_sprite_animation(
            "animations/hero/go_right_backward.ron",
            &mut self.progress,
            data.world,
        ));
        self.go_left_backward_animation_handle = Some(assets::load_sprite_animation(
            "animations/hero/go_left_backward.ron",
            &mut self.progress,
            data.world,
        ));
        self.go_left_forward_animation_handle = Some(assets::load_sprite_animation(
            "animations/hero/go_left_forward.ron",
            &mut self.progress,
            data.world,
        ));
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        let world = &data.world;
        if self.progress.is_complete() {
            return Trans::Switch(Box::new(OverworldState::new(
                self.display_config.clone(),
                assets::load_sprite_render_animation(
                    world,
                    self.idle_animation_handle.take().unwrap(),
                ),
                assets::load_sprite_render_animation(
                    world,
                    self.go_right_animation_handle.take().unwrap(),
                ),
                assets::load_sprite_render_animation(
                    world,
                    self.go_left_animation_handle.take().unwrap(),
                ),
                assets::load_sprite_render_animation(
                    world,
                    self.go_forward_animation_handle.take().unwrap(),
                ),
                assets::load_sprite_render_animation(
                    world,
                    self.go_backward_animation_handle.take().unwrap(),
                ),
                assets::load_sprite_render_animation(
                    world,
                    self.go_right_forward_animation_handle.take().unwrap(),
                ),
                assets::load_sprite_render_animation(
                    world,
                    self.go_right_backward_animation_handle.take().unwrap(),
                ),
                assets::load_sprite_render_animation(
                    world,
                    self.go_left_backward_animation_handle.take().unwrap(),
                ),
                assets::load_sprite_render_animation(
                    world,
                    self.go_left_forward_animation_handle.take().unwrap(),
                ),
            )));
        }
        Trans::None
    }
}
