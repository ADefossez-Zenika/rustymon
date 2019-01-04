use serde_derive::Deserialize;

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Deserialize)]
pub enum AnimationId {
    Idle,
    GoRight,
    GoLeft,
    GoForward,
    GoBackward,
    GoRightForward,
    GoRightBackward,
    GoLeftBackward,
    GoLeftForward,
}
