use bevy::{render::camera::Camera, transform::components::GlobalTransform};
use bevy_math::{primitives::InfinitePlane3d, Dir3, Vec2, Vec3};

/// Calculates in game world coordinates, for given mouse cursor position.
pub fn calc_mouse_world_coord(
    cursor_position: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec3> {
    let plane = InfinitePlane3d {
        normal: Dir3::new_unchecked(Vec3::new(0.0, 0.0, 1.0)),
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return None;
    };

    let Some(distance) = ray.intersect_plane(Vec3::ZERO, plane) else {
        return None;
    };

    let global_cursor = ray.get_point(distance);

    Some(global_cursor)
}

