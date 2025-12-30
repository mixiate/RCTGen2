mod curves;
mod track_curves;
mod track_desc;
mod track_sections;

const TILE_SIZE: f32 = 3.3;
const CLEARANCE_HEIGHT: f32 = 0.204_124_15; // 8 pixels tall

fn add_model_to_scene<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    model: &'a renderer::model::Model,
    is_ghost: Option<bool>,
    track_section: &track_sections::TrackSection,
    scale: f32,
    offset: f32,
    bank_angle: f32,
) -> anyhow::Result<()> {
    let transform = |(position, normal): (&glam::Vec3, &glam::Vec3)| {
        let distance = ((position.z / TILE_SIZE) * scale) + offset;
        let point = track_section.sample_curve(distance, bank_angle);

        let position = (point.position * TILE_SIZE) + (point.normal * position.y) + (point.binormal * position.x);
        let normal = (point.tangent * normal.z) + (point.normal * normal.y) + (point.binormal * normal.x);

        (position, normal)
    };
    scene.add_model_transform(model, transform, None, is_ghost)
}

fn render_rotation(
    scene: &renderer::Scene,
    camera: &glam::Mat4,
    lights: &[renderer::Light],
    rotation: usize,
    dither: bool,
) -> renderer::image::IndexedImage {
    let view_rotation = glam::Mat4::from_rotation_y(90.0_f32.to_radians() * rotation as f32);
    let camera = camera * view_rotation;

    let view_rotation_inverse = view_rotation.inverse();
    let lights = lights.iter().map(|x| x.transform(&view_rotation_inverse)).collect::<Vec<_>>();

    let framebuffer = renderer::render_scene(scene, &camera, &lights, 4, 4);
    framebuffer.into_cropped_indexed_image(dither)
}

#[expect(clippy::too_many_arguments)]
fn render_track_section(
    render_device: &renderer::Device,
    camera: &glam::Mat4,
    lights: &[renderer::Light],
    models: &track_desc::Models<renderer::model::Model>,
    dither: bool,
    track: &track_desc::Track,
    track_section: &track_sections::TrackSection,
    output_directory: &std::path::Path,
) -> anyhow::Result<()> {
    use rayon::prelude::*;

    let mesh_count = (0.5 + track_section.length / track.length).floor() as usize;
    let scale = track_section.length / (mesh_count as f32 * track.length);
    let length = scale * track.length;
    let bank_angle = track.bank_angle.to_radians();

    let mut scene = renderer::SceneBuilder::new(render_device)?;

    add_model_to_scene(
        &mut scene,
        &models.track,
        Some(true),
        track_section,
        scale,
        -length,
        bank_angle,
    )?;
    add_model_to_scene(
        &mut scene,
        &models.track,
        Some(true),
        track_section,
        scale,
        track_section.length,
        bank_angle,
    )?;

    for i in 0..mesh_count {
        add_model_to_scene(
            &mut scene,
            &models.track,
            None,
            track_section,
            scale,
            i as f32 * length,
            bank_angle,
        )?;
    }

    let scene = scene.build();

    let images = (0..4)
        .into_par_iter()
        .map(|rotation| render_rotation(&scene, camera, lights, rotation, dither))
        .collect::<Vec<_>>();

    for (i, image) in images.iter().enumerate() {
        let image_name = format!("{}_{i}", track_section.name);
        _ = image.save(&output_directory.join(&track.name).join(image_name).with_extension("png"));
    }

    Ok(())
}

fn render(
    track_desc: &track_desc::Desc,
    base_directory: &std::path::Path,
    output_directory: &std::path::Path,
) -> anyhow::Result<()> {
    use anyhow::Context as _;

    let render_device = renderer::Device::try_new().context("Could not create render device")?;

    let camera = glam::Mat4::from_mat3(
        glam::Mat3::from_cols(
            glam::Vec3::new(32.0, 0.0, 32.0),
            glam::Vec3::new(16.0, -16.0 * 6.0_f32.sqrt(), -16.0),
            glam::Vec3::new(-16.0 * 3.0_f32.sqrt(), -16.0 * 2.0_f32.sqrt(), 16.0 * 3.0_f32.sqrt()),
        )
        .transpose(),
    ) / TILE_SIZE;

    let lights = track_desc.get_lights();

    let track_sections = [
        &track_sections::FLAT,
        &track_sections::FLAT_TO_GENTLE,
        &track_sections::GENTLE,
        &track_sections::GENTLE_TO_FLAT,
        &track_sections::GENTLE_TO_STEEP,
        &track_sections::STEEP_TO_GENTLE,
        &track_sections::STEEP,
        &track_sections::STEEP_TO_VERTICAL,
        &track_sections::VERTICAL_TO_STEEP,
        &track_sections::VERTICAL,
        &track_sections::SMALL_FLAT_TO_STEEP,
        &track_sections::SMALL_STEEP_TO_FLAT,
        &track_sections::FLAT_TO_STEEP,
        &track_sections::STEEP_TO_FLAT,
        &track_sections::SMALL_TURN_LEFT,
        &track_sections::MEDIUM_TURN_LEFT,
        &track_sections::LARGE_TURN_LEFT_TO_DIAG,
        &track_sections::LARGE_TURN_RIGHT_TO_DIAG,
        &track_sections::FLAT_DIAG,
        &track_sections::FLAT_TO_GENTLE_DIAG,
        &track_sections::GENTLE_TO_FLAT_DIAG,
        &track_sections::GENTLE_DIAG,
        &track_sections::GENTLE_TO_STEEP_DIAG,
        &track_sections::STEEP_TO_GENTLE_DIAG,
        &track_sections::STEEP_DIAG,
        &track_sections::SMALL_FLAT_TO_STEEP_DIAG,
        &track_sections::SMALL_STEEP_TO_FLAT_DIAG,
        &track_sections::FLAT_TO_STEEP_DIAG,
        &track_sections::STEEP_TO_FLAT_DIAG,
        &track_sections::FLAT_TO_LEFT_BANK,
        &track_sections::FLAT_TO_RIGHT_BANK,
        &track_sections::LEFT_BANK_TO_GENTLE,
        &track_sections::RIGHT_BANK_TO_GENTLE,
        &track_sections::GENTLE_TO_LEFT_BANK,
        &track_sections::GENTLE_TO_RIGHT_BANK,
        &track_sections::LEFT_BANK,
        &track_sections::SMALL_TURN_LEFT_BANK,
        &track_sections::MEDIUM_TURN_LEFT_BANK,
        &track_sections::LARGE_TURN_LEFT_TO_DIAG_BANK,
        &track_sections::LARGE_TURN_RIGHT_TO_DIAG_BANK,
        &track_sections::FLAT_TO_LEFT_BANK_DIAG,
        &track_sections::FLAT_TO_RIGHT_BANK_DIAG,
        &track_sections::LEFT_BANK_TO_GENTLE_DIAG,
        &track_sections::RIGHT_BANK_TO_GENTLE_DIAG,
        &track_sections::GENTLE_TO_LEFT_BANK_DIAG,
        &track_sections::GENTLE_TO_RIGHT_BANK_DIAG,
        &track_sections::LEFT_BANK_DIAG,
        &track_sections::SMALL_TURN_LEFT_GENTLE,
        &track_sections::SMALL_TURN_RIGHT_GENTLE,
        &track_sections::MEDIUM_TURN_LEFT_GENTLE,
        &track_sections::MEDIUM_TURN_RIGHT_GENTLE,
        &track_sections::LARGE_TURN_LEFT_TO_DIAG_GENTLE,
        &track_sections::LARGE_TURN_RIGHT_TO_DIAG_GENTLE,
        &track_sections::LARGE_TURN_LEFT_TO_ORTHOGONAL_GENTLE,
        &track_sections::LARGE_TURN_RIGHT_TO_ORTHOGONAL_GENTLE,
        &track_sections::VERY_SMALL_TURN_LEFT_STEEP,
        &track_sections::VERY_SMALL_TURN_RIGHT_STEEP,
        &track_sections::VERTICAL_TWIST_LEFT,
        &track_sections::VERTICAL_TWIST_RIGHT,
    ];

    for track in &track_desc.tracks {
        let models = track.models.load(base_directory)?;

        for track_section in &track_sections {
            render_track_section(
                &render_device,
                &camera,
                &lights,
                &models,
                track_desc.dither,
                track,
                track_section,
                output_directory,
            )?;
        }
    }

    Ok(())
}

pub fn make_track(
    track_description_file_path: &std::path::Path,
    output_directory: &std::path::Path,
) -> anyhow::Result<()> {
    use anyhow::Context as _;

    let desc = track_desc::Desc::load(track_description_file_path)?;

    let base_directory = track_description_file_path.parent().with_context(|| {
        format!(
            "Could not get parent directory of {}",
            track_description_file_path.display()
        )
    })?;

    render(&desc, base_directory, output_directory)?;

    Ok(())
}
