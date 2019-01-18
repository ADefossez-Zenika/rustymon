use amethyst::ecs::{Component, NullStorage};
use specs_derive::*;

/// Mark an entity as being active.
/// Inactive entities must not be processed by 'game logic' systems.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Active;

/// Mark an entity as being compatible with the `Overworld` state.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct OverworldCompat;

/// Mark an entity as being compatible with the `Instance` state.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct InstanceCompat;
