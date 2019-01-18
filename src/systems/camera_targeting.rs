use crate::components::{Active, CameraTarget};
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Join, ReadStorage, System, WriteStorage},
    renderer::Camera,
};
use nalgebra::base::Vector3;

const MAX_TARGET_DISTANCE: f32 = 2500.0;
const MAX_CAMERA_LERP_FACTOR: f32 = 0.01;

/// Make cameras follow their target.
pub struct CameraTargetingSystem;

impl<'a> System<'a> for CameraTargetingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, CameraTarget>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Active>,
    );

    fn run(&mut self, (entities, cameras, targets, mut transforms, actives): Self::SystemData) {
        for (entity, _, target, _) in (&entities, &cameras, &targets, &actives).join() {
            if actives.get(target.entity).is_none() {
                continue;
            }
            
            let new_cam_position =
                compute_new_camera_position(transforms.get(entity), transforms.get(target.entity));
            if let Some(new_cam_position) = new_cam_position {
                if let Some(cam_trans) = transforms.get_mut(entity) {
                    cam_trans.set_x(new_cam_position.x);
                    cam_trans.set_y(new_cam_position.y);
                }
            }
        }
    }
}

fn compute_new_camera_position(
    cam_trans: Option<&Transform>,
    target_trans: Option<&Transform>,
) -> Option<Vector3<f32>> {
    if let (Some(cam_trans), Some(target_trans)) = (cam_trans, target_trans) {
        let direction = target_trans.translation() - cam_trans.translation();
        let distance = direction.norm_squared();
        if distance > MAX_TARGET_DISTANCE {
            Some(
                cam_trans
                    .translation()
                    .lerp(target_trans.translation(), MAX_CAMERA_LERP_FACTOR),
            )
        } else {
            None
        }
    } else {
        None
    }
}
