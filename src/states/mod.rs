mod instance;
mod overworld;

pub use self::{
    instance::{Instance, InstanceState},
    overworld::LoadingState as OverworldState,
};

/// Represent an order to transition into another states.
/// Each state will test the current value of this ressource.
/// If it doesn't match the one they are attached to then it has to
/// transition to the requested one.
#[derive(Debug)]
pub enum GameState {
    /// Should transition to the overworld state and set the players at the given position.
    Overworld((f32, f32)),
    /// Should transition to the instance state. Inside the provided instance.
    Instance(Instance),
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Overworld((0.0, 0.0))
    }
}
