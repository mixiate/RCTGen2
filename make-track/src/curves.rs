fn cubic(a: f32, b: f32, c: f32, d: f32, x: f32) -> f32 {
    x * (x * (x * a + b) + c) + d
}

fn cubic_derivative(a: f32, b: f32, c: f32, x: f32) -> f32 {
    x * (3.0 * x * a + 2.0 * b) + c
}

fn cubic_second_derivative(a: f32, b: f32, x: f32) -> f32 {
    6.0 * x * a + 2.0 * b
}

#[expect(clippy::too_many_arguments)]
pub fn reparameterize(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, x: f32) -> f32 {
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

pub fn plane_curve_horizontal(position: &glam::Vec3, tangent: &glam::Vec3) -> crate::track_sections::TrackPoint {
    crate::track_sections::TrackPoint {
        position: *position,
        tangent: *tangent,
        normal: glam::Vec3::new(0.0, 1.0, 0.0),
        binormal: glam::Vec3::new(tangent.z, 0.0, -tangent.x),
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

#[expect(clippy::too_many_arguments)]
pub fn cubic_curve_horizontal(
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
    plane_curve_horizontal(
        &glam::Vec3::new(cubic(y_a, y_b, y_c, y_d, u), 0.0, cubic(x_a, x_b, x_c, x_d, u)),
        &glam::Vec3::new(
            cubic_derivative(y_a, y_b, y_c, u),
            0.0,
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

pub fn plane_curve_vertical_diagonal(position: &glam::Vec3, tangent: &glam::Vec3) -> crate::track_sections::TrackPoint {
    use std::f32::consts::SQRT_2;
    let binormal = 0.5_f32.sqrt();
    crate::track_sections::TrackPoint {
        position: *position,
        tangent: *tangent,
        normal: glam::Vec3::new(-tangent.y / SQRT_2, tangent.z * SQRT_2, -tangent.y / SQRT_2),
        binormal: glam::Vec3::new(binormal, 0.0, -binormal),
    }
}

#[expect(clippy::too_many_arguments)]
pub fn cubic_curve_vertical_diagonal(
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
    use std::f32::consts::SQRT_2;
    let u = reparameterize(p_a, p_b, p_c, p_d, p_e, p_f, p_g, distance);
    let x = cubic(x_a, x_b, x_c, x_d, u);
    let y = cubic(y_a, y_b, y_c, y_d, u);
    let d_x = cubic_derivative(x_a, x_b, x_c, u);
    let d_y = cubic_derivative(y_a, y_b, y_c, u);
    plane_curve_vertical_diagonal(
        &glam::Vec3::new(x / SQRT_2, y, x / SQRT_2),
        &glam::Vec3::new(d_x / SQRT_2, d_y, d_x / SQRT_2).normalize(),
    )
}

#[expect(clippy::too_many_arguments)]
pub fn bezier3d(
    xa: f32,
    xb: f32,
    xc: f32,
    xd: f32,
    ya: f32,
    yb: f32,
    yc: f32,
    yd: f32,
    za: f32,
    zb: f32,
    zc: f32,
    zd: f32,
    ra: f32,
    rb: f32,
    rc: f32,
    rd: f32,
    pa: f32,
    pb: f32,
    pc: f32,
    pd: f32,
    pe: f32,
    pf: f32,
    pg: f32,
    distance: f32,
) -> crate::track_sections::TrackPoint {
    let u = reparameterize(pa, pb, pc, pd, pe, pf, pg, distance);

    let position = glam::Vec3::new(
        cubic(xa, xb, xc, xd, u),
        cubic(ya, yb, yc, yd, u),
        cubic(za, zb, zc, zd, u),
    );
    let tangent = glam::Vec3::new(
        cubic_derivative(xa, xb, xc, u),
        cubic_derivative(ya, yb, yc, u),
        cubic_derivative(za, zb, zc, u),
    )
    .normalize();
    let second_derivative = glam::Vec3::new(
        cubic_second_derivative(xa, xb, u),
        cubic_second_derivative(ya, yb, u),
        cubic_second_derivative(za, zb, u),
    );
    let normal = (second_derivative - (tangent * tangent.dot(second_derivative))).normalize();
    let binormal = normal.cross(tangent);

    let angle = cubic(ra, rb, rc, rd, u);
    let (angle_sin, angle_cos) = angle.sin_cos();

    crate::track_sections::TrackPoint {
        position,
        tangent,
        normal: (normal * angle_cos) + (binormal * angle_sin),
        binormal: (normal * -angle_sin) + (binormal * angle_cos),
    }
}

pub fn flip_x_axis(mut point: crate::track_sections::TrackPoint) -> crate::track_sections::TrackPoint {
    point.position.x *= -1.0;
    point.normal.x *= -1.0;
    point.tangent.x *= -1.0;
    point.binormal.y *= -1.0;
    point.binormal.z *= -1.0;
    point
}

pub fn sloped_turn_left(radius: f32, gradient: f32, distance: f32) -> crate::track_sections::TrackPoint {
    let arc_length = radius * (1.0 + gradient * gradient).sqrt();
    let angle = distance / arc_length;

    let (angle_sin, angle_cos) = angle.sin_cos();

    let tangent_z = 1.0 / (1.0 + gradient * gradient).sqrt();
    let tangent_y = gradient / (1.0 + gradient * gradient).sqrt();

    let tangent = glam::Vec3::new(-tangent_z * angle_sin, tangent_y, tangent_z * angle_cos).normalize();
    let normal = glam::Vec3::new(tangent_y * angle_sin, tangent_z, -tangent_y * angle_cos).normalize();

    crate::track_sections::TrackPoint {
        position: glam::Vec3::new(
            radius * (angle_cos - 1.0),
            angle * radius * gradient,
            radius * angle_sin,
        ),
        tangent,
        normal,
        binormal: normal.cross(tangent),
    }
}

pub fn sloped_turn_right(radius: f32, gradient: f32, distance: f32) -> crate::track_sections::TrackPoint {
    let arc_length = radius * (1.0 + gradient * gradient).sqrt();
    let angle = distance / arc_length;

    let (angle_sin, angle_cos) = angle.sin_cos();

    let tangent_z = 1.0 / (1.0 + gradient * gradient).sqrt();
    let tangent_y = gradient / (1.0 + gradient * gradient).sqrt();

    let tangent = glam::Vec3::new(tangent_z * angle_sin, tangent_y, tangent_z * angle_cos).normalize();
    let normal = glam::Vec3::new(-tangent_y * angle_sin, tangent_z, -tangent_y * angle_cos).normalize();

    crate::track_sections::TrackPoint {
        position: glam::Vec3::new(
            radius * (1.0 - angle_cos),
            angle * radius * gradient,
            radius * angle_sin,
        ),
        tangent,
        normal,
        binormal: normal.cross(tangent),
    }
}

#[expect(clippy::too_many_arguments)]
pub fn large_turn_to_diag_gentle(
    x_a: f32,
    x_b: f32,
    x_c: f32,
    x_d: f32,
    y_a: f32,
    y_b: f32,
    y_c: f32,
    y_d: f32,
    distance: f32,
) -> crate::track_sections::TrackPoint {
    let u = reparameterize(
        1.751_793e-5,
        -1.076_157_3e-4,
        3.104_037e-4,
        6.329_133_6e-5,
        1.046_596_3e-3,
        -3.471_377e-4,
        3.091_829_4e-1,
        distance,
    );
    let mut point = cubic_curve_horizontal(
        x_a, x_b, x_c, x_d, y_a, y_b, y_c, y_d, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, u,
    );

    point.position.y += 6.0 * crate::CLEARANCE_HEIGHT * u - 1.5 * crate::CLEARANCE_HEIGHT * u * u * (u - 1.0);
    point.tangent.y +=
        crate::CLEARANCE_HEIGHT * (6.0 - 1.5 * u * (3.0 * u - 2.0)) / crate::track_curves::LARGE_TURN_LENGTH;
    point.tangent = point.tangent.normalize();
    point.normal = point.tangent.cross(point.binormal);

    point
}

#[expect(clippy::too_many_arguments)]
pub fn large_turn_to_orthogonal_gentle(
    x_a: f32,
    x_b: f32,
    x_c: f32,
    x_d: f32,
    y_a: f32,
    y_b: f32,
    y_c: f32,
    y_d: f32,
    distance: f32,
) -> crate::track_sections::TrackPoint {
    let u = reparameterize(
        1.751_793e-5,
        -1.076_157_3e-4,
        3.104_037e-4,
        6.329_133_6e-5,
        1.046_596_3e-3,
        -3.471_377e-4,
        3.091_829_4e-1,
        distance,
    );
    let mut point = cubic_curve_horizontal(
        x_a, x_b, x_c, x_d, y_a, y_b, y_c, y_d, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, u,
    );

    point.position.y += 6.0 * crate::CLEARANCE_HEIGHT * u - 1.5 * crate::CLEARANCE_HEIGHT * u * (u * u - 2.0 * u + 1.0);
    point.tangent.y +=
        crate::CLEARANCE_HEIGHT * (6.0 - 1.5 * (3.0 * u * u - 4.0 * u + 1.0)) / crate::track_curves::LARGE_TURN_LENGTH;
    point.tangent = point.tangent.normalize();
    point.normal = point.tangent.cross(point.binormal);

    point
}

pub fn roll_left(length: f32, radius: f32, distance: f32) -> crate::track_sections::TrackPoint {
    use std::f32::consts::PI;

    let u = distance / length;

    let (sin, cos) = (PI * u).sin_cos();

    let position = glam::Vec3::new(radius * sin, radius * (1.0 - cos), 3.0 * u);
    let tangent = if (1.0e-4..=length - 1.0e-4).contains(&distance) {
        glam::Vec3::new(radius * PI * cos / length, radius * PI * sin / length, 1.0).normalize()
    } else {
        glam::Vec3::new(0.0, 0.0, 1.0)
    };
    let normal = glam::Vec3::new(-sin, cos, 0.0);
    let binormal = normal.cross(tangent);

    crate::track_sections::TrackPoint {
        position,
        tangent,
        normal,
        binormal,
    }
}
