fn cubic(coeffs: &[f32; 4], x: f32) -> f32 {
    x * (x * (x * coeffs[0] + coeffs[1]) + coeffs[2]) + coeffs[3]
}

fn cubic_derivative(coeffs: &[f32; 4], x: f32) -> f32 {
    x * (3.0 * x * coeffs[0] + 2.0 * coeffs[1]) + coeffs[2]
}

fn cubic_second_derivative(a: f32, b: f32, x: f32) -> f32 {
    6.0 * x * a + 2.0 * b
}

pub fn reparameterize(c: &[f32; 7], x: f32) -> f32 {
    x * (x * (x * (x * (x * (x * (x * c[0] + c[1]) + c[2]) + c[3]) + c[4]) + c[5]) + c[6])
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

pub fn cubic_curve_vertical(
    x: &[f32; 4],
    y: &[f32; 4],
    reparam_coeffs: &[f32; 7],
    distance: f32,
) -> crate::track_sections::TrackPoint {
    let u = reparameterize(reparam_coeffs, distance);
    plane_curve_vertical(
        &glam::Vec3::new(0.0, cubic(y, u), cubic(x, u)),
        &glam::Vec3::new(0.0, cubic_derivative(y, u), cubic_derivative(x, u)).normalize(),
    )
}

pub fn cubic_curve_horizontal(
    x: &[f32; 4],
    y: &[f32; 4],
    reparam_coeffs: &[f32; 7],
    distance: f32,
) -> crate::track_sections::TrackPoint {
    let u = reparameterize(reparam_coeffs, distance);
    plane_curve_horizontal(
        &glam::Vec3::new(cubic(y, u), 0.0, cubic(x, u)),
        &glam::Vec3::new(cubic_derivative(y, u), 0.0, cubic_derivative(x, u)).normalize(),
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

pub fn cubic_curve_vertical_diagonal(
    x: &[f32; 4],
    y: &[f32; 4],
    reparam_coeffs: &[f32; 7],
    distance: f32,
) -> crate::track_sections::TrackPoint {
    use std::f32::consts::SQRT_2;
    let u = reparameterize(reparam_coeffs, distance);
    let c_x = cubic(x, u);
    let c_y = cubic(y, u);
    let d_x = cubic_derivative(x, u);
    let d_y = cubic_derivative(y, u);
    plane_curve_vertical_diagonal(
        &glam::Vec3::new(c_x / SQRT_2, c_y, c_x / SQRT_2),
        &glam::Vec3::new(d_x / SQRT_2, d_y, d_x / SQRT_2).normalize(),
    )
}

pub fn bezier3d(
    x: &[f32; 4],
    y: &[f32; 4],
    z: &[f32; 4],
    roll: &[f32; 4],
    reparam_coeffs: &[f32; 7],
    distance: f32,
) -> crate::track_sections::TrackPoint {
    let u = reparameterize(reparam_coeffs, distance);

    let position = glam::Vec3::new(cubic(x, u), cubic(y, u), cubic(z, u));
    let tangent = glam::Vec3::new(cubic_derivative(x, u), cubic_derivative(y, u), cubic_derivative(z, u)).normalize();
    let second_derivative = glam::Vec3::new(
        cubic_second_derivative(x[0], x[1], u),
        cubic_second_derivative(y[0], y[1], u),
        cubic_second_derivative(z[0], z[1], u),
    );
    let normal = (second_derivative - (tangent * tangent.dot(second_derivative))).normalize();
    let binormal = normal.cross(tangent);

    let angle = cubic(roll, u);
    let (angle_sin, angle_cos) = angle.sin_cos();

    crate::track_sections::TrackPoint {
        position,
        tangent,
        normal: (normal * angle_cos) + (binormal * angle_sin),
        binormal: (normal * -angle_sin) + (binormal * angle_cos),
    }
}

pub fn zero_g_roll(
    radius: f32,
    x: &[f32; 4],
    y: &[f32; 4],
    roll: &[f32; 4],
    reparam_coeffs: &[f32; 7],
    distance: f32,
) -> crate::track_sections::TrackPoint {
    let u = reparameterize(reparam_coeffs, distance);

    let unbanked_point = plane_curve_vertical(
        &glam::Vec3::new(0.0, cubic(y, u), cubic(x, u)),
        &glam::Vec3::new(0.0, cubic_derivative(y, u), cubic_derivative(x, u)).normalize(),
    );

    let roll_rate = cubic_derivative(roll, distance);
    let roll = cubic(roll, distance);

    let (roll_sin, roll_cos) = roll.sin_cos();

    let position = unbanked_point.normal * (radius * (1.0 - roll_cos));
    let position = position + (unbanked_point.binormal * (radius * roll_sin));
    let position = unbanked_point.position + position;

    let tangent = unbanked_point.normal * radius * roll_rate * roll_sin;
    let tangent = tangent + (unbanked_point.binormal * radius * roll_rate * roll_cos);
    let tangent = (unbanked_point.tangent + tangent).normalize();

    let normal = (unbanked_point.normal * roll_cos) + (unbanked_point.binormal * -roll_sin);

    crate::track_sections::TrackPoint {
        position,
        tangent,
        normal,
        binormal: normal.cross(tangent),
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

pub fn flip_x_axis_diagonal(mut point: crate::track_sections::TrackPoint) -> crate::track_sections::TrackPoint {
    use glam::Vec3Swizzles as _;
    point.position = point.position.with_xz(glam::Vec2::new(point.position.z, point.position.x));
    point.normal = point.normal.with_xz(glam::Vec2::new(point.normal.z, point.normal.x));
    point.tangent = point.tangent.with_xz(glam::Vec2::new(point.tangent.z, point.tangent.x));
    point.binormal = point.normal.cross(point.tangent);
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

pub fn large_turn_to_diag_gentle(x: &[f32; 4], y: &[f32; 4], distance: f32) -> crate::track_sections::TrackPoint {
    let u = reparameterize(
        &[
            1.751793e-5,
            -1.0761573e-4,
            3.104037e-4,
            6.3291336e-5,
            1.0465963e-3,
            -3.471377e-4,
            3.0918294e-1,
        ],
        distance,
    );
    let mut point = cubic_curve_horizontal(x, y, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0], u);

    point.position.y += 6.0 * crate::CLEARANCE_HEIGHT * u - 1.5 * crate::CLEARANCE_HEIGHT * u * u * (u - 1.0);
    point.tangent.y +=
        crate::CLEARANCE_HEIGHT * (6.0 - 1.5 * u * (3.0 * u - 2.0)) / crate::track_curves::LARGE_TURN_LENGTH;
    point.tangent = point.tangent.normalize();
    point.normal = point.tangent.cross(point.binormal);

    point
}

pub fn large_turn_to_orthogonal_gentle(x: &[f32; 4], y: &[f32; 4], distance: f32) -> crate::track_sections::TrackPoint {
    let u = reparameterize(
        &[
            1.751793e-5,
            -1.0761573e-4,
            3.104037e-4,
            6.3291336e-5,
            1.0465963e-3,
            -3.471377e-4,
            3.0918294e-1,
        ],
        distance,
    );
    let mut point = cubic_curve_horizontal(x, y, &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0], u);

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

pub fn flatten_ends(point: crate::track_sections::TrackPoint, percentage: f32) -> crate::track_sections::TrackPoint {
    const FLAT_PERCENTAGE: f32 = 0.05;
    let percentage = if percentage <= FLAT_PERCENTAGE {
        1.0 - (percentage / FLAT_PERCENTAGE)
    } else if percentage >= (1.0 - FLAT_PERCENTAGE) {
        (percentage - (1.0 - FLAT_PERCENTAGE)) / FLAT_PERCENTAGE
    } else {
        return point;
    };

    let percentage = percentage * percentage;

    let normal = point.normal.lerp(glam::Vec3::Y, percentage);
    let binormal = normal.cross(point.tangent).normalize();
    crate::track_sections::TrackPoint {
        position: point.position,
        normal,
        tangent: binormal.cross(normal).normalize(),
        binormal,
    }
}
