mod bundle;
mod states;
mod systems;
mod assets;

use crate::bundle::RustymonBundle;
use crate::states::GameplayState;
use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let display_config = DisplayConfig::load("configs/display.ron");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RustymonBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config.clone())).with_sprite_sheet_processor(),
        )?;
    let mut game =
        Application::build("assets/", GameplayState::new(display_config))?.build(game_data)?;
    game.run();
    Ok(())
}
