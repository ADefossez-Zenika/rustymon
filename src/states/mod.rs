mod instance;
mod overworld;

pub use self::{
    instance::{Instance, InstanceState},
    overworld::LoadingState as OverworldState,
};

pub enum GameState {
    Overworld,
    Instance(Instance),
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Overworld
    }
}
