use crate::{
    animations::HeroAnimationId,
    assets,
    components::{
        Active, Body, CameraTarget, Dynamic, Hero, InstanceCompat, Mob, OverworldCompat, Portal,
        Shape, Velocity,
    },
    states::Instance,
};

use amethyst::{
    animation::AnimationSet,
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::*,
    renderer::{Camera, DisplayConfig, Projection, SpriteRender, SpriteSheet},
};

use ncollide2d::{
    math::Vector,
    shape::{Ball, Cuboid},
};

pub const CAM_Z_POS: f32 = 1.0;

/// Build a camera following an entity.
pub fn build_camera(display_config: &DisplayConfig, world: &mut World, target: Entity) -> Entity {
    let (half_width, half_height) = {
        let (width, height) = display_config.dimensions.unwrap();
        (width as f32 * 0.5, height as f32 * 0.5)
    };

    let mut transform = Transform::default();
    transform.set_z(CAM_Z_POS);
    world
        .create_entity()
        .with(Active)
        .with(OverworldCompat)
        .with(InstanceCompat)
        .with(Camera::from(Projection::orthographic(
            -half_width,
            half_width,
            -half_height,
            half_height,
        )))
        .with(transform)
        .with(CameraTarget { entity: target })
        .build()
}

/// Build the hero.
pub fn build_hero(
    animations: AnimationSet<HeroAnimationId, SpriteRender>,
    world: &mut World,
) -> Entity {
    // let texture = assets::load_texture("sprite_sheets/hero.png", world);
    let texture = assets::load_texture("sprite_sheets/hero_debug.png", world);
    let sprite_sheet = assets::load_sprite_sheet("sprite_sheets/hero.ron", texture, world);

    world
        .create_entity()
        .with(Active)
        .with(OverworldCompat)
        .with(InstanceCompat)
        .with(Hero::new())
        .with(animations)
        .with(SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        })
        .with(Transform::default())
        .with(Velocity::new())
        .with(Body {
            shape: Shape::Circle {
                shape: Ball::new(16.0),
            },
            dynamic: Dynamic::Dynamic,
        })
        .build()
}

/// Build a ferris at a given position.
pub fn build_ferris(
    x: f32,
    y: f32,
    reset_threshold: f32,
    target_threshold: f32,
    sprite_sheet: Handle<SpriteSheet>,
    world: &mut World,
) {
    let mut transform = Transform::default();
    transform.set_xyz(x, y, 0.0);

    world
        .create_entity()
        .with(Active)
        .with(OverworldCompat)
        .with(Mob::new(x, y, reset_threshold, target_threshold))
        .with(SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        })
        .with(transform)
        .with(Velocity::new())
        .with(Body {
            shape: Shape::Circle {
                shape: Ball::new(16.0),
            },
            dynamic: Dynamic::Dynamic,
        })
        .build();
}

/// Build a building.
/// This is a static physical entity.
pub fn build_building(x: f32, y: f32, sprite_sheet: Handle<SpriteSheet>, world: &mut World) {
    let mut transform = Transform::default();
    transform.set_xyz(x, y, 0.0);

    world
        .create_entity()
        .with(Active)
        .with(OverworldCompat)
        .with(SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        })
        .with(transform)
        .with(Body {
            shape: Shape::Box {
                shape: Cuboid::new(Vector::new(32.0, 16.0)),
            },
            dynamic: Dynamic::Static,
        })
        .build();
}

/// Build a portal.
/// When triggered a portal will bring the player into the pointed instance.
pub fn build_portal(
    x: f32,
    y: f32,
    instance: Instance,
    trigger_zone: Cuboid<f32>,
    sprite_sheet: Handle<SpriteSheet>,
    world: &mut World,
) {
    let mut transform = Transform::default();
    transform.set_xyz(x, y, 0.0);

    world
        .create_entity()
        .with(Active)
        .with(OverworldCompat)
        .with(SpriteRender {
            sprite_sheet,
            sprite_number: 1,
        })
        .with(transform)
        .with(Portal {
            instance,
            trigger_zone,
        })
        .build();
}
