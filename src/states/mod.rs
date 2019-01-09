mod instance;
mod overworld;

pub use self::{
    instance::{Instance, InstanceState},
    overworld::LoadingState as OverworldState,
};

const CAM_Z_POS: f32 = 1.0;

pub enum GameState {
    Overworld,
    Instance(Instance),
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Overworld
    }
}
