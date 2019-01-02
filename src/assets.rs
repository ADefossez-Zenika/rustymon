use crate::animations::SpriteAnimation;
use amethyst::{
    animation::{
        Animation, InterpolationFunction, Sampler, SpriteRenderChannel, SpriteRenderPrimitive,
    },
    assets::{AssetStorage, Handle, Loader, Progress, RonFormat},
    ecs::prelude::World,
    renderer::{
        PngFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture,
        TextureHandle, TextureMetadata,
    },
};

/// Load a sprite sheet from the asset directory.
pub fn load_sprite_sheet<N: Into<String>>(
    path: N,
    texture_handle: TextureHandle,
    world: &World,
) -> SpriteSheetHandle {
    let loader = world.read_resource::<Loader>();
    loader.load(
        path,
        SpriteSheetFormat,
        texture_handle,
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}

/// Load a texture from the asset directory.
pub fn load_texture<N: Into<String>>(path: N, world: &World) -> TextureHandle {
    let loader = world.read_resource::<Loader>();
    loader.load(
        path,
        PngFormat,
        TextureMetadata::srgb(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}

/// Load a sprite animation from the asset directory.
pub fn load_sprite_animation<N, P>(path: N, progress: P, world: &World) -> Handle<SpriteAnimation>
where
    N: Into<String>,
    P: Progress,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        path,
        RonFormat,
        (),
        progress,
        &world.read_resource::<AssetStorage<SpriteAnimation>>(),
    )
}

pub fn load_sprite_render_animation(
    world: &World,
    sprite_animation_handle: Handle<SpriteAnimation>,
) -> Handle<Animation<SpriteRender>> {
    let loader = world.read_resource::<Loader>();
    let sprite_animation_storage = world.read_resource::<AssetStorage<SpriteAnimation>>();
    let sprite_animation = sprite_animation_storage
        .get(&sprite_animation_handle)
        .unwrap();

    let input = sprite_animation
        .key_frames
        .iter()
        .map(|k| k.time)
        .collect::<Vec<f32>>();

    let output = sprite_animation
        .key_frames
        .iter()
        .map(|k| SpriteRenderPrimitive::SpriteIndex(k.sprite_index))
        .collect::<Vec<SpriteRenderPrimitive>>();

    let sampler = Sampler {
        input,
        output,
        function: InterpolationFunction::Step,
    };

    let sampler_handle = loader.load_from_data(
        sampler,
        (),
        &world.read_resource::<AssetStorage<Sampler<SpriteRenderPrimitive>>>(),
    );

    let animation = Animation::<SpriteRender> {
        nodes: vec![(0, SpriteRenderChannel::SpriteIndex, sampler_handle)],
    };

    loader.load_from_data(animation, (), &world.read_resource())
}
