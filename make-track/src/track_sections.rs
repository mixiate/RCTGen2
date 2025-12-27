pub struct TrackPoint {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub tangent: glam::Vec3,
    pub binormal: glam::Vec3,
}

pub struct TrackSection {
    pub name: &'static str,
    pub curve: fn(f32) -> TrackPoint,
    pub length: f32,
}

const CLEARANCE_HEIGHT: f32 = 0.204_124_15; // 8 pixels tall

fn plane_curve_vertical(position: &glam::Vec3, tangent: &glam::Vec3) -> TrackPoint {
    TrackPoint {
        position: *position,
        tangent: *tangent,
        normal: glam::Vec3::new(0.0, tangent.z, -tangent.y),
        binormal: glam::Vec3::new(1.0, 0.0, 0.0),
    }
}

fn curve_flat(distance: f32) -> TrackPoint {
    plane_curve_vertical(&glam::Vec3::new(0.0, 0.0, distance), &glam::Vec3::new(0.0, 0.0, 1.0))
}

pub const TRACK_FLAT: TrackSection = TrackSection {
    name: "flat",
    curve: curve_flat,
    length: 1.0,
};

fn curve_gentle(distance: f32) -> TrackPoint {
    let u = distance / TRACK_GENTLE_LENGTH;
    plane_curve_vertical(
        &glam::Vec3::new(0.0, 2.0 * CLEARANCE_HEIGHT * u, u),
        &glam::Vec3::new(0.0, 2.0 * CLEARANCE_HEIGHT, 1.0).normalize(),
    )
}

const TRACK_GENTLE_LENGTH: f32 = 1.080123;
pub const TRACK_GENTLE: TrackSection = TrackSection {
    name: "gentle",
    curve: curve_gentle,
    length: TRACK_GENTLE_LENGTH,
};

fn curve_medium_turn_left(distance: f32) -> TrackPoint {
    const RADIUS: f32 = -2.5;
    let angle = distance / RADIUS;
    let angle_sin = angle.sin();
    let angle_cos = angle.cos();
    let tangent = glam::Vec3::new(angle_sin, 0.0, angle_cos);
    let normal = glam::Vec3::new(0.0, 1.0, 0.0);
    TrackPoint {
        position: glam::Vec3::new(RADIUS * (1.0 - angle_cos), 0.0, RADIUS * angle_sin),
        tangent,
        normal,
        binormal: normal.cross(tangent),
    }
}

pub const TRACK_MEDIUM_TURN_LEFT: TrackSection = TrackSection {
    name: "medium_turn_left",
    curve: curve_medium_turn_left,
    length: 1.25 * std::f32::consts::PI,
};
