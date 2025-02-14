use godot::classes::Viewport;

use godot::prelude::*;

/// What wall grid coordinate is the cursor hovering over, if any.
pub fn hovered_wall_grid_coord(viewport: Gd<Viewport>) -> Option<Vector2i> {
    let Some(camera) = viewport.get_camera_3d() else {
        return None;
    };

    let mouse_pos = viewport.get_mouse_position();
    let mouse_origin = camera.project_ray_origin(mouse_pos);
    let mouse_normal = camera.project_ray_normal(mouse_pos);

    let floor_height = 0.0;
    let floor = Plane::new(Vector3::UP, floor_height);

    let Some(hover_pos) = floor.intersect_ray(mouse_origin, mouse_origin + mouse_normal * 1024.0)
    else {
        return None;
    };

    let coord = Vector2i::new(hover_pos.x.round() as i32, hover_pos.z.round() as i32);
    if coord.x < 0 || coord.y < 0 || coord.x > 32 || coord.y > 32 {
        return None;
    }
    Some(coord)
}
