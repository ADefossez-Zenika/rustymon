use crate::{animations::SpriteAnimation, assets, entities, resources::WorldBounds};

use super::{GameState, Instance, InstanceState};

use amethyst::{
    animation::Animation,
    assets::{Handle, ProgressCounter},
    core::transform::Transform,
    ecs::prelude::*,
    renderer::{DisplayConfig, SpriteRender},
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};

use ncollide2d::{math::Vector, shape::Cuboid};

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
    hero: Option<Entity>,
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
            hero: None,
            camera: None,
        }
    }
}

impl OverworldState {
    fn build_overworld_bounds() -> WorldBounds {
        WorldBounds::new_around_origin(10000.0, 10000.0)
    }
}

impl SimpleState for OverworldState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        world.add_resource(Self::build_overworld_bounds());

        let building_texture = assets::load_texture("sprite_sheets/buildings.png", world);
        let building_sprite_sheet =
            assets::load_sprite_sheet("sprite_sheets/buildings.ron", building_texture, world);

        let hero = entities::build_hero(
            self.idle_animation_handle.clone(),
            self.go_right_animation_handle.clone(),
            self.go_left_animation_handle.clone(),
            self.go_forward_animation_handle.clone(),
            self.go_backward_animation_handle.clone(),
            self.go_right_forward_animation_handle.clone(),
            self.go_right_backward_animation_handle.clone(),
            self.go_left_backward_animation_handle.clone(),
            self.go_left_forward_animation_handle.clone(),
            world,
        );
        let camera = entities::build_camera(&self.display_config, world, hero);
        entities::build_building(100.0, 100.0, building_sprite_sheet.clone(), world);
        entities::build_portal(
            100.0,
            76.0,
            Instance {
                spawn: (1000.0, 1000.0),
                bounds: WorldBounds::new(975.0, 1025.0, 975.0, 1025.0),
                exit: (100.0, 76.0),
            },
            Cuboid::new(Vector::new(16.0, 16.0)),
            building_sprite_sheet,
            world,
        );

        self.hero = Some(hero);
        self.camera = Some(camera);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&mut data.world);
        match *data.world.read_resource::<GameState>() {
            GameState::Instance(ref instance) => Trans::Push(Box::new(InstanceState::new(
                instance.clone(),
                self.hero.unwrap(),
                self.camera.unwrap(),
            ))),
            _ => Trans::None,
        }
    }

    fn on_pause(&mut self, _data: StateData<GameData>) {
        println!("Pausing OverworldState");
    }

    fn on_resume(&mut self, data: StateData<GameData>) {
        println!("Resuming OverworldState");

        // Restore overworld's boundaries
        *data.world.write_resource::<WorldBounds>() = Self::build_overworld_bounds();

        // Put hero back to the position it was before state switch.
        // Point camera on hero.
        {
            match *data.world.read_resource::<GameState>() {
                GameState::Overworld((x, y)) => {
                    let hero_trans = {
                        let mut t = Transform::default();
                        t.set_xyz(x, y, 0.0);
                        t
                    };

                    let cam_trans = {
                        let mut t = Transform::default();
                        t.set_xyz(x, y, entities::CAM_Z_POS);
                        t
                    };

                    let mut storage = data.world.write_storage::<Transform>();
                    storage.insert(self.hero.unwrap(), hero_trans).unwrap();
                    storage.insert(self.camera.unwrap(), cam_trans).unwrap();
                }
                _ => panic!("Remusing Overworld state but the current state is not Overworld"),
            }
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
