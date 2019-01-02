use crate::{animations::SpriteAnimation, assets, states::GameplayState};
use amethyst::{
    assets::{Handle, ProgressCounter},
    renderer::DisplayConfig,
    GameData, SimpleState, SimpleTrans, StateData, Trans,
};

pub struct LoadingState {
    display_config: DisplayConfig,
    progress: ProgressCounter,
    idle_animation_handle: Option<Handle<SpriteAnimation>>,
    go_right_animation_handle: Option<Handle<SpriteAnimation>>,
    go_left_animation_handle: Option<Handle<SpriteAnimation>>,
    go_forward_animation_handle: Option<Handle<SpriteAnimation>>,
    go_backward_animation_handle: Option<Handle<SpriteAnimation>>,
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
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        if self.progress.is_complete() {
            return Trans::Switch(Box::new(GameplayState::new(
                self.display_config.clone(),
                self.idle_animation_handle.take().unwrap(),
                self.go_right_animation_handle.take().unwrap(),
                self.go_left_animation_handle.take().unwrap(),
                self.go_forward_animation_handle.take().unwrap(),
                self.go_backward_animation_handle.take().unwrap(),
            )));
        }
        Trans::None
    }
}
