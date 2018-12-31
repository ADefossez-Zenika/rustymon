mod animations;
mod assets;
mod bundle;
mod states;
mod systems;

use crate::{bundle::RustymonBundle, states::GameplayState};

use amethyst::{
    animation::AnimationBundle,
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, SpriteRender, Stage, ALPHA,
    },
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let display_config = DisplayConfig::load("configs/display.ron");

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RustymonBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(AnimationBundle::<u32, SpriteRender>::new(
            "control", "sampler",
        ))?
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config.clone())).with_sprite_sheet_processor(),
        )?;
    let mut game =
        Application::build("assets/", GameplayState::new(display_config))?.build(game_data)?;
    game.run();
    Ok(())
}
