pub fn plane_curve_vertical(position: &glam::Vec3, tangent: &glam::Vec3) -> crate::track_sections::TrackPoint {
    crate::track_sections::TrackPoint {
        position: *position,
        tangent: *tangent,
        normal: glam::Vec3::new(0.0, tangent.z, -tangent.y),
        binormal: glam::Vec3::new(1.0, 0.0, 0.0),
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
