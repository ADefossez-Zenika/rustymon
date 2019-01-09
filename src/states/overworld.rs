use crate::{
    animations::*,
    assets,
    components::{Body, CameraTarget, Dynamic, HeroAnimation, Shape},
    resources::WorldBounds,
};

use super::{GameState, Instance, InstanceState};

use amethyst::{
    animation::{Animation, AnimationControlSet},
    assets::{Handle, ProgressCounter},
    core::transform::Transform,
    ecs::prelude::*,
    renderer::{Camera, DisplayConfig, Projection, SpriteRender},
    winit::VirtualKeyCode,
    GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans,
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
    hero: (Option<Entity>, Option<Transform>),
    camera: Option<Entity>,
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
            hero: (None, None),
            camera: None,
        }
    }
}

impl OverworldState {
    fn init_camera(&self, world: &mut World, target: Entity) -> Entity {
        let (half_width, half_height) = {
            let (width, height) = self.display_config.dimensions.unwrap();
            (width as f32 * 0.5, height as f32 * 0.5)
        };

        let mut transform = Transform::default();
        transform.set_z(super::CAM_Z_POS);
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
            .build()
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
        let world = data.world;
        world.add_resource(WorldBounds::new_around_origin(10000.0, 10000.0));

        let hero = self.build_hero(world);
        let camera = self.init_camera(world, hero);
        self.build_building(100.0, 100.0, world);

        self.hero.0 = Some(hero);
        self.camera = Some(camera);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&mut data.world);
        match *data.world.read_resource::<GameState>() {
            GameState::Instance(ref instance) => Trans::Push(Box::new(InstanceState::new(
                instance.clone(),
                self.hero.0.unwrap(),
                self.camera.unwrap(),
            ))),
            _ => Trans::None,
        }
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if amethyst::input::is_key_down(&event, VirtualKeyCode::Escape) {
                *data.world.write_resource::<GameState>() = GameState::Instance(Instance {
                    spawn: (1000.0, 1000.0),
                    bounds: WorldBounds::new(975.0, 1025.0, 975.0, 1025.0),
                });
            }
        }
        Trans::None
    }

    fn on_pause(&mut self, data: StateData<GameData>) {
        println!("Pausing OverworldState");

        let storage = data.world.read_storage::<Transform>();
        self.hero.1 = storage.get(self.hero.0.unwrap()).cloned();
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        println!("Resuming OverworldState");

        // Put hero back to the position it was before state switch.
        // Point camera on hero.
        {
            let hero_trans = self.hero.1.take().unwrap();
            let cam_trans = {
                let mut t = hero_trans.clone();
                t.set_z(super::CAM_Z_POS);
                t
            };

            let mut storage = data.world.write_storage::<Transform>();
            storage.insert(self.hero.0.unwrap(), hero_trans).unwrap();
            storage.insert(self.camera.unwrap(), cam_trans).unwrap();
        }
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
        data.data.update(&mut data.world);
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
