pub fn add_model(scene: &embree4_rs::Scene, model: &crate::model::Model) -> anyhow::Result<()> {
    let tri_mesh =
        embree4_rs::geometry::TriangleMeshGeometry::try_new(&scene.device, &model.positions, &model.indices)?;
    scene.attach_geometry(&tri_mesh)?;

    Ok(())
}
