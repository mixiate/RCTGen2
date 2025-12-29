fn cubic(a: f32, b: f32, c: f32, d: f32, x: f32) -> f32 {
    x * (x * (x * a + b) + c) + d
}

fn cubic_derivative(a: f32, b: f32, c: f32, x: f32) -> f32 {
    x * (3.0 * x * a + 2.0 * b) + c
}

#[expect(clippy::too_many_arguments)]
fn reparameterize(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, x: f32) -> f32 {
    x * (x * (x * (x * (x * (x * (x * a + b) + c) + d) + e) + f) + g)
}

pub fn plane_curve_vertical(position: &glam::Vec3, tangent: &glam::Vec3) -> crate::track_sections::TrackPoint {
    crate::track_sections::TrackPoint {
        position: *position,
        tangent: *tangent,
        normal: glam::Vec3::new(0.0, tangent.z, -tangent.y),
        binormal: glam::Vec3::new(1.0, 0.0, 0.0),
    }
}

#[expect(clippy::too_many_arguments)]
pub fn cubic_curve_vertical(
    x_a: f32,
    x_b: f32,
    x_c: f32,
    x_d: f32,
    y_a: f32,
    y_b: f32,
    y_c: f32,
    y_d: f32,
    p_a: f32,
    p_b: f32,
    p_c: f32,
    p_d: f32,
    p_e: f32,
    p_f: f32,
    p_g: f32,
    distance: f32,
) -> crate::track_sections::TrackPoint {
    let u = reparameterize(p_a, p_b, p_c, p_d, p_e, p_f, p_g, distance);
    plane_curve_vertical(
        &glam::Vec3::new(0.0, cubic(y_a, y_b, y_c, y_d, u), cubic(x_a, x_b, x_c, x_d, u)),
        &glam::Vec3::new(
            0.0,
            cubic_derivative(y_a, y_b, y_c, u),
            cubic_derivative(x_a, x_b, x_c, u),
        )
        .normalize(),
    )
}

pub fn turn_left(distance: f32, radius: f32) -> crate::track_sections::TrackPoint {
    let angle = distance / radius;
    let angle_sin = angle.sin();
    let angle_cos = angle.cos();
    let tangent = glam::Vec3::new(-angle_sin, 0.0, angle_cos);
    let normal = glam::Vec3::new(0.0, 1.0, 0.0);
    crate::track_sections::TrackPoint {
        position: glam::Vec3::new(radius * (angle_cos - 1.0), 0.0, radius * angle_sin),
        tangent,
        normal,
        binormal: normal.cross(tangent),
    }
}

pub fn banked_curve(
    unbanked_point: &crate::track_sections::TrackPoint,
    angle: f32,
) -> crate::track_sections::TrackPoint {
    let (angle_sin, angle_cos) = angle.sin_cos();
    crate::track_sections::TrackPoint {
        position: unbanked_point.position,
        tangent: unbanked_point.tangent,
        normal: (unbanked_point.normal * angle_cos) + (unbanked_point.binormal * angle_sin),
        binormal: (unbanked_point.normal * -angle_sin) + (unbanked_point.binormal * angle_cos),
    }
}
