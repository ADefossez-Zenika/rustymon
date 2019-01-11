use crate::{
    animations::{HeroAnimationId, SpriteAnimation},
    assets, entities,
    resources::WorldBounds,
};

use super::{GameState, Instance, InstanceState};

use amethyst::{
    animation::AnimationSet,
    assets::{Handle, ProgressCounter},
    core::transform::Transform,
    ecs::prelude::*,
    renderer::{DisplayConfig, SpriteRender},
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};

use ncollide2d::{math::Vector, shape::Cuboid};

pub struct OverworldState {
    display_config: DisplayConfig,
    /// Option so I can transfer ownership of the animation set ... Is there a better option ?
    hero_animations: Option<AnimationSet<HeroAnimationId, SpriteRender>>,
    hero: Option<Entity>,
    camera: Option<Entity>,
}

impl OverworldState {
    pub fn new(
        display_config: DisplayConfig,
        hero_animations: AnimationSet<HeroAnimationId, SpriteRender>,
    ) -> Self {
        OverworldState {
            display_config,
            hero_animations: Some(hero_animations),
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

        let hero = entities::build_hero(self.hero_animations.take().unwrap(), world);
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

    fn build_hero_animations(
        &mut self,
        world: &World,
    ) -> AnimationSet<HeroAnimationId, SpriteRender> {
        let load = |handle: &mut Option<Handle<SpriteAnimation>>| {
            assets::load_sprite_render_animation(world, handle.take().unwrap())
        };

        let idle = load(&mut self.idle_animation_handle);
        let go_right = load(&mut self.go_right_animation_handle);
        let go_left = load(&mut self.go_left_animation_handle);
        let go_forward = load(&mut self.go_forward_animation_handle);
        let go_backward = load(&mut self.go_backward_animation_handle);
        let go_right_forward = load(&mut self.go_right_forward_animation_handle);
        let go_right_backward = load(&mut self.go_right_backward_animation_handle);
        let go_left_backward = load(&mut self.go_left_backward_animation_handle);
        let go_left_forward = load(&mut self.go_left_forward_animation_handle);

        let mut animations = AnimationSet::new();
        animations.insert(HeroAnimationId::Idle, idle);
        animations.insert(HeroAnimationId::GoRight, go_right);
        animations.insert(HeroAnimationId::GoLeft, go_left);
        animations.insert(HeroAnimationId::GoForward, go_forward);
        animations.insert(HeroAnimationId::GoBackward, go_backward);
        animations.insert(HeroAnimationId::GoRightForward, go_right_forward);
        animations.insert(HeroAnimationId::GoRightBackward, go_right_backward);
        animations.insert(HeroAnimationId::GoLeftBackward, go_left_backward);
        animations.insert(HeroAnimationId::GoLeftForward, go_left_forward);

        animations
    }

    fn load_sprite_animation<N: Into<String>>(
        &mut self,
        path: N,
        world: &World,
    ) -> Option<Handle<SpriteAnimation>> {
        Some(assets::load_sprite_animation(
            path,
            &mut self.progress,
            world,
        ))
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        self.idle_animation_handle =
            self.load_sprite_animation("animations/hero/idle.ron", data.world);
        self.go_right_animation_handle =
            self.load_sprite_animation("animations/hero/go_right.ron", data.world);
        self.go_left_animation_handle =
            self.load_sprite_animation("animations/hero/go_left.ron", data.world);
        self.go_forward_animation_handle =
            self.load_sprite_animation("animations/hero/go_forward.ron", data.world);
        self.go_backward_animation_handle =
            self.load_sprite_animation("animations/hero/go_backward.ron", data.world);
        self.go_right_forward_animation_handle =
            self.load_sprite_animation("animations/hero/go_right_forward.ron", data.world);
        self.go_right_backward_animation_handle =
            self.load_sprite_animation("animations/hero/go_right_backward.ron", data.world);
        self.go_left_backward_animation_handle =
            self.load_sprite_animation("animations/hero/go_left_backward.ron", data.world);
        self.go_left_forward_animation_handle =
            self.load_sprite_animation("animations/hero/go_left_forward.ron", data.world);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&mut data.world);
        let world = &data.world;
        if self.progress.is_complete() {
            return Trans::Switch(Box::new(OverworldState::new(
                self.display_config.clone(),
                self.build_hero_animations(world),
            )));
        }
        Trans::None
    }
}
