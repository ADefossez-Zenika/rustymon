use super::GameState;
use amethyst::winit::VirtualKeyCode;
use amethyst::{
    core::transform::Transform, ecs::Entity, input, GameData, SimpleState, SimpleTrans, StateData,
    StateEvent, Trans,
};

use crate::resources::WorldBounds;

/// Instance data.
#[derive(Clone)]
pub struct Instance {
    pub spawn: (f32, f32),
    pub bounds: WorldBounds,
}

/// State active when inside an instance (building/dungeon).
pub struct InstanceState {
    instance: Instance,
    hero: Entity,
    camera: Entity,
}

impl InstanceState {
    /// Build a new instance state from an `Instance`.
    pub fn new(instance: Instance, hero: Entity, camera: Entity) -> Self {
        InstanceState {
            instance,
            hero,
            camera,
        }
    }

    fn build_transform_from_spawn(&self) -> Transform {
        let mut t = Transform::default();
        t.set_xyz(self.instance.spawn.0, self.instance.spawn.1, 0.0);
        t
    }
}

impl SimpleState for InstanceState {
    /// Create and add entities to the world
    fn on_start(&mut self, data: StateData<GameData>) {
        println!("Starting InstanceState");

        // set up hero and camera position
        {
            let mut storage = data.world.write_storage::<Transform>();
            let hero_transform = self.build_transform_from_spawn();
            let cam_trans = {
                let mut t = hero_transform.clone();
                t.set_z(super::CAM_Z_POS);
                t
            };

            storage.insert(self.hero, hero_transform.clone()).unwrap();
            storage.insert(self.camera, cam_trans).unwrap();
        }

        // set up world boundaries
        *data.world.write_resource::<WorldBounds>() = self.instance.bounds;
    }

    /// Remove entities specific to the instance.
    fn on_stop(&mut self, _data: StateData<GameData>) {
        println!("Stoping InstanceState");
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&mut data.world);

        match *data.world.read_resource::<GameState>() {
            GameState::Overworld => Trans::Pop,
            _ => Trans::None,
        }
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if input::is_key_down(&event, VirtualKeyCode::Escape) {
                *data.world.write_resource::<GameState>() = GameState::Overworld;
            }
        }
        Trans::None
    }
}
