pub fn plane_curve_vertical(position: &glam::Vec3, tangent: &glam::Vec3) -> crate::track_sections::TrackPoint {
    crate::track_sections::TrackPoint {
        position: *position,
        tangent: *tangent,
        normal: glam::Vec3::new(0.0, tangent.z, -tangent.y),
        binormal: glam::Vec3::new(1.0, 0.0, 0.0),
    }
}
