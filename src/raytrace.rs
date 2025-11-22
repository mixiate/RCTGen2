pub fn add_model(
    scene: &embree4_rs::Scene,
    model: &crate::model::Model,
    translation: &glam::Vec3,
    rotation: &glam::Quat,
) -> anyhow::Result<()> {
    let transform = glam::Mat4::from_translation(*translation) * glam::Mat4::from_quat(*rotation);

    let positions: Vec<(f32, f32, f32)> =
        model.positions.iter().map(|x| transform.transform_point3(*x).into()).collect();

    let tri_mesh = embree4_rs::geometry::TriangleMeshGeometry::try_new(&scene.device, &positions, &model.indices)?;
    scene.attach_geometry(&tri_mesh)?;

    Ok(())
}

pub fn trace_ray(scene: &embree4_rs::CommittedScene, origin: &glam::Vec3, direction: &glam::Vec3) -> bool {
    let ray = embree4_sys::RTCRay {
        org_x: origin.x,
        org_y: origin.y,
        org_z: origin.z,
        dir_x: direction.x,
        dir_y: direction.y,
        dir_z: direction.z,
        tnear: 0.0,
        tfar: f32::INFINITY,
        ..Default::default()
    };
    scene.intersect_1(ray, None).unwrap_or_default().is_some()
}
