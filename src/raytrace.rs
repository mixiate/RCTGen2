pub fn add_model(scene: &embree4_rs::Scene, model: &crate::model::Model) -> anyhow::Result<()> {
    let tri_mesh =
        embree4_rs::geometry::TriangleMeshGeometry::try_new(&scene.device, &model.positions, &model.indices)?;
    scene.attach_geometry(&tri_mesh)?;

    Ok(())
}

pub fn trace_ray(scene: &embree4_rs::CommittedScene, origin: &[f32; 3], direction: &[f32; 3]) -> bool {
    let ray = embree4_sys::RTCRay {
        org_x: origin[0],
        org_y: origin[1],
        org_z: origin[2],
        dir_x: direction[0],
        dir_y: direction[1],
        dir_z: direction[2],
        tnear: 0.0,
        tfar: f32::INFINITY,
        ..Default::default()
    };
    scene.intersect_1(ray, None).unwrap_or_default().is_some()
}
