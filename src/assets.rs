use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::prelude::World,
    renderer::{
        PngFormat, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureHandle,
        TextureMetadata,
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
