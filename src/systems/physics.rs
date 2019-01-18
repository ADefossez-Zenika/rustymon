use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
};

use ncollide2d::{
    math::{Isometry, Vector},
    query::{contact, Contact},
};

use crate::{
    components::{Body, CollisionMarker, Dynamic, Shape, Velocity},
    resources::WorldBounds,
};

/// Move entities according to their velocity.
/// Also make sure that entity is kept in world boundaries if any.
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Velocity>,
        Option<Read<'a, WorldBounds>>,
    );

    fn run(&mut self, (mut transforms, velocities, bounds): Self::SystemData) {
        for (transform, velocity) in (&mut transforms, &velocities).join() {
            let translation = transform.translation();
            let mut x = translation.x + velocity.direction.x * velocity.speed;
            let mut y = translation.y + velocity.direction.y * velocity.speed;

            if let Some(bounds) = &bounds {
                x = x.min(bounds.right).max(bounds.left);
                y = y.min(bounds.top).max(bounds.bottom);
            }

            transform.set_x(x);
            transform.set_y(y);
        }
    }
}

/// Simple physics computation system. Handle collision detection and resolution.
/// This is a first draft which might be removed in favor of ncollide simulation.
pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Body>,
        WriteStorage<'a, CollisionMarker>,
    );

    fn run(&mut self, (entities, mut transforms, bodies, mut collisions): Self::SystemData) {
        // Detect collisions and mark dynamics colliding entities
        for (entity_a, transform_a, body_a) in (&entities, &transforms, &bodies).join() {
            for (entity_b, transform_b, body_b) in (&entities, &transforms, &bodies).join() {
                if entity_a == entity_b {
                    continue;
                }

                let contact = compute_contact(transform_a, body_a, transform_b, body_b);

                if let Some(contact) = contact {
                    if Dynamic::Dynamic == body_a.dynamic {
                        collisions
                            .insert(
                                entity_a,
                                CollisionMarker::new(contact.normal.unwrap() * -contact.depth),
                            )
                            .unwrap();
                    }
                }
            }
        }

        // Resolve all collisions
        for (transform, collision) in (&mut transforms, &collisions).join() {
            transform.translate_x(collision.penetration.x);
            transform.translate_y(collision.penetration.y);
        }

        // Remove all collision markers
        collisions.clear();
    }
}

/// Compute the distance to separate two colliding entities.
/// Return None if not colliding.
fn compute_contact(
    transform_a: &Transform,
    body_a: &Body,
    transform_b: &Transform,
    body_b: &Body,
) -> Option<Contact<f32>> {
    let pos_a = Isometry::new(
        Vector::new(transform_a.translation().x, transform_a.translation().y),
        nalgebra::zero(),
    );
    let pos_b = Isometry::new(
        Vector::new(transform_b.translation().x, transform_b.translation().y),
        nalgebra::zero(),
    );

    match (&body_a.shape, &body_b.shape) {
        (Shape::Circle { shape: shape_a }, Shape::Circle { shape: shape_b }) => {
            contact(&pos_a, shape_a, &pos_b, shape_b, 0.0)
        }
        (Shape::Box { shape: shape_a }, Shape::Box { shape: shape_b }) => {
            contact(&pos_a, shape_a, &pos_b, shape_b, 0.0)
        }
        (Shape::Circle { shape: shape_a }, Shape::Box { shape: shape_b }) => {
            contact(&pos_a, shape_a, &pos_b, shape_b, 0.0)
        }
        (Shape::Box { shape: shape_a }, Shape::Circle { shape: shape_b }) => {
            contact(&pos_a, shape_a, &pos_b, shape_b, 0.0)
        }
    }
}
