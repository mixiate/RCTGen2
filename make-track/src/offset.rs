#[derive(Debug)]
pub enum OffsetType {
    Flat,
    Gentle,
    Steep,
    FlatBanked,
    GentleBanked,
    Inverted,
    Diagonal,
    DiagonalGentle,
    DiagonalSteep,
    DiagonalBanked,
    Vertical,
}

#[derive(Debug)]
pub struct OffsetDesc {
    pub offset_type: OffsetType,
    pub banked_right: bool,
    pub rotation_offset: usize,
}

const VIEW_COUNT: usize = 4;
const ROTATION_MATRICES: [glam::Mat4; VIEW_COUNT] = [
    glam::Mat4::IDENTITY,
    glam::Mat4 {
        x_axis: glam::Vec4::new(0.0, 0.0, 1.0, 0.0),
        y_axis: glam::Vec4::new(0.0, 1.0, 0.0, 0.0),
        z_axis: glam::Vec4::new(-1.0, 0.0, 0.0, 0.0),
        w_axis: glam::Vec4::new(0.0, 0.0, 0.0, 1.0),
    },
    glam::Mat4 {
        x_axis: glam::Vec4::new(-1.0, 0.0, 0.0, 0.0),
        y_axis: glam::Vec4::new(0.0, 1.0, 0.0, 0.0),
        z_axis: glam::Vec4::new(0.0, 0.0, -1.0, 0.0),
        w_axis: glam::Vec4::new(0.0, 0.0, 0.0, 1.0),
    },
    glam::Mat4 {
        x_axis: glam::Vec4::new(0.0, 0.0, -1.0, 0.0),
        y_axis: glam::Vec4::new(0.0, 1.0, 0.0, 0.0),
        z_axis: glam::Vec4::new(1.0, 0.0, 0.0, 0.0),
        w_axis: glam::Vec4::new(0.0, 0.0, 0.0, 1.0),
    },
];

fn compare_vector(a: &glam::Vec3, b: &glam::Vec3, rotation: usize) -> bool {
    let x = a - ROTATION_MATRICES[rotation].transform_vector3(*b).normalize();
    x.dot(x).sqrt() < 0.19 // change from original of 0.15 to get zero_g_roll inverted. no side effects currently
}

fn get_offset_type(
    point: &crate::track_sections::TrackPoint,
    banked: bool,
    rotation: usize,
) -> Option<(OffsetType, bool)> {
    const FLAT_TANGENT: glam::Vec3 = glam::Vec3::new(0.0, 0.0, 1.0);
    const GENTLE_TANGENT: glam::Vec3 = glam::Vec3::new(0.0, 2.0 * crate::CLEARANCE_HEIGHT, 1.0);
    const STEEP_TANGENT: glam::Vec3 = glam::Vec3::new(0.0, 8.0 * crate::CLEARANCE_HEIGHT, 1.0);
    const DIAGONAL_TANGENT: glam::Vec3 = glam::Vec3::new(1.0, 0.0, 1.0);
    const DIAGONAL_GENTLE_TANGENT: glam::Vec3 = glam::Vec3::new(1.0, 2.0 * crate::CLEARANCE_HEIGHT, 1.0);
    const DIAGONAL_STEEP_TANGENT: glam::Vec3 = glam::Vec3::new(1.0, 8.0 * crate::CLEARANCE_HEIGHT, 1.0);
    const VERTICAL_NORMAL: glam::Vec3 = glam::Vec3::new(0.0, 0.0, -1.0);

    let banked_right = banked && point.binormal.y < 0.0;

    if compare_vector(&point.tangent, &FLAT_TANGENT, rotation) {
        if banked {
            Some((OffsetType::FlatBanked, banked_right))
        } else if point.normal.y < -0.9 {
            Some((OffsetType::Inverted, false))
        } else {
            Some((OffsetType::Flat, false))
        }
    } else if compare_vector(&point.tangent, &GENTLE_TANGENT, rotation) {
        if banked {
            Some((OffsetType::GentleBanked, banked_right))
        } else {
            Some((OffsetType::Gentle, false))
        }
    } else if compare_vector(&point.tangent, &STEEP_TANGENT, rotation) {
        Some((OffsetType::Steep, false))
    } else if compare_vector(&point.tangent, &DIAGONAL_TANGENT, rotation) {
        if banked {
            Some((OffsetType::DiagonalBanked, banked_right))
        } else {
            Some((OffsetType::Diagonal, false))
        }
    } else if compare_vector(&point.tangent, &DIAGONAL_GENTLE_TANGENT, rotation) {
        if banked {
            Some((OffsetType::DiagonalBanked, banked_right))
        } else {
            Some((OffsetType::DiagonalGentle, false))
        }
    } else if compare_vector(&point.tangent, &DIAGONAL_STEEP_TANGENT, rotation) {
        Some((OffsetType::DiagonalSteep, false))
    } else if compare_vector(&point.normal, &VERTICAL_NORMAL, rotation) {
        Some((OffsetType::Vertical, false))
    } else {
        None
    }
}

fn get_offset_desc(point: &crate::track_sections::TrackPoint, banked: bool) -> Option<OffsetDesc> {
    for rotation in 0..VIEW_COUNT {
        if let Some((offset_type, banked_right)) = get_offset_type(point, banked, rotation) {
            return Some(OffsetDesc {
                offset_type,
                banked_right,
                rotation_offset: rotation,
            });
        }
    }
    None
}

fn get_track_point(
    track_section: &crate::track_sections::TrackSection,
    bank_angle: f32,
    distance: f32,
) -> (crate::track_sections::TrackPoint, bool) {
    let point = track_section.sample_curve(distance, bank_angle);
    let point_unbanked = track_section.sample_curve(distance, 0.0);

    let angle = point.normal.angle_between(point_unbanked.normal);
    let banked = angle > bank_angle * 0.9 && angle < bank_angle * 1.1;

    (point, banked)
}

fn get_offset(offsets: &crate::track_desc::Offsets, offset_desc: &OffsetDesc, rotation: usize) -> glam::Vec3 {
    let right_rotation_offset = if offset_desc.banked_right { 2 } else { 0 };
    let offset_index = (rotation + (VIEW_COUNT - offset_desc.rotation_offset) + right_rotation_offset) % VIEW_COUNT;

    let offset = match offset_desc.offset_type {
        OffsetType::Flat => offsets.flat[offset_index],
        OffsetType::Gentle => offsets.gentle[offset_index],
        OffsetType::Steep => offsets.steep[offset_index],
        OffsetType::FlatBanked => offsets.flat_banked[offset_index],
        OffsetType::GentleBanked => offsets.gentle_banked[offset_index],
        OffsetType::Inverted => offsets.inverted[offset_index],
        OffsetType::Diagonal => offsets.diagonal[offset_index],
        OffsetType::DiagonalGentle => offsets.diagonal_gentle[offset_index],
        OffsetType::DiagonalSteep => offsets.diagonal_steep[offset_index],
        OffsetType::DiagonalBanked => offsets.diagonal_banked[offset_index],
        OffsetType::Vertical => offsets.vertical[offset_index],
    };

    let mut offset = glam::Vec3::new(0.0, offset[1] * crate::CLEARANCE_HEIGHT / 8.0, offset[0] / 32.0);

    if offset_desc.banked_right {
        offset.z *= -1.0;
    }

    if std::matches!(
        offset_desc.offset_type,
        OffsetType::Diagonal | OffsetType::DiagonalGentle | OffsetType::DiagonalSteep | OffsetType::DiagonalBanked
    ) {
        offset.z *= std::f32::consts::FRAC_1_SQRT_2;
        offset.x = -offset.z; // negative to match the originals flipped x axis, but not sure if ultimately correct?
    }

    if offset_desc.rotation_offset > 0 {
        offset = ROTATION_MATRICES[offset_desc.rotation_offset].transform_point3(offset);
    }

    offset
}

pub fn calculate(
    offsets: &crate::track_desc::Offsets,
    track_section: &crate::track_sections::TrackSection,
    bank_angle: f32,
    distance: f32,
    rotation: usize,
) -> glam::Vec3 {
    let (point, banked) = get_track_point(track_section, bank_angle, distance);
    let desc = get_offset_desc(&point, banked);

    if let Some(desc) = desc {
        get_offset(offsets, &desc, rotation)
    } else {
        glam::Vec3::splat(0.0)
    }
}
