use super::GameState;
use amethyst::winit::VirtualKeyCode;
use amethyst::{
    core::transform::Transform, ecs::Entity, input, GameData, SimpleState, SimpleTrans, StateData,
    StateEvent, Trans,
};

use crate::{entities, resources::WorldBounds};

/// Instance data.
#[derive(Clone, Copy, Debug)]
pub struct Instance {
    /// The position the hero will spawn when entering.
    pub spawn: (f32, f32),
    /// The boundaries of the instance.
    pub bounds: WorldBounds,
    /// The position in the overworld the hero will end up when exiting the instance.
    pub exit: (f32, f32),
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
                t.set_z(entities::CAM_Z_POS);
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
            GameState::Overworld(_) => Trans::Pop,
            _ => Trans::None,
        }
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if input::is_key_down(&event, VirtualKeyCode::Escape) {
                *data.world.write_resource::<GameState>() =
                    GameState::Overworld(self.instance.exit);
            }
        }
        Trans::None
    }
}
