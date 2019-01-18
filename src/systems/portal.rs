use crate::{
    components::{Hero, Portal, Active},
    states::GameState,
};
use amethyst::{
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, Write},
    input::InputHandler,
};
use ncollide2d::{
    math::{Isometry, Point, Vector},
    query::PointQuery,
};

/// Check if a portal is being triggered.
/// If so then it will trigger a state change.
pub struct PortalTriggerSystem;

impl<'a> System<'a> for PortalTriggerSystem {
    type SystemData = (
        Write<'a, GameState>,
        Read<'a, InputHandler<String, String>>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Portal>,
        ReadStorage<'a, Hero>,
        ReadStorage<'a, Active>,
    );

    fn run(&mut self, (mut state, input, transforms, portals, hero, actives): Self::SystemData) {
        'outer: for (transform_p, portal, _) in (&transforms, &portals, &actives).join() {
            let trigger_position = {
                let t = transform_p.translation();
                Isometry::new(Vector::new(t.x, t.y), nalgebra::zero())
            };

            let zone = &portal.trigger_zone;
            for (transform_h, _, _) in (&transforms, &hero, &actives).join() {
                let hero_position = {
                    let t = transform_h.translation();
                    Point::new(t.x, t.y)
                };

                if zone.contains_point(&trigger_position, &hero_position) {
                    if input.action_is_down("use").unwrap() {
                        *state = GameState::Instance(portal.instance.clone());
                        break 'outer;
                    }
                }
            }
        }
    }
}
