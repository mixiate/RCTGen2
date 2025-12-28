mod curves;
mod track_curves;
mod track_desc;
mod track_sections;

const TILE_SIZE: f32 = 3.3;
const CLEARANCE_HEIGHT: f32 = 0.204_124_15; // 8 pixels tall

fn get_track_point(track_section: &track_sections::TrackSection, distance: f32) -> track_sections::TrackPoint {
    if distance < 0.0 {
        let mut point = (track_section.curve)(0.0);
        point.position += point.tangent * distance;
        point
    } else if distance > track_section.length {
        let mut point = (track_section.curve)(track_section.length);
        point.position += point.tangent * (distance - track_section.length);
        point
    } else {
        (track_section.curve)(distance)
    }
}

fn add_model_to_scene<'a>(
    scene: &mut renderer::SceneBuilder<'a>,
    model: &'a renderer::model::Model,
    is_ghost: Option<bool>,
    track_section: &track_sections::TrackSection,
    scale: f32,
    offset: f32,
) -> anyhow::Result<()> {
    let transform = |(position, normal): (&glam::Vec3, &glam::Vec3)| {
        let distance = ((position.z / TILE_SIZE) * scale) + offset;
        let point = get_track_point(track_section, distance);

        let position = (point.position * TILE_SIZE) + (point.normal * position.y) + (point.binormal * position.x);
        let normal = (point.tangent * normal.z) + (point.normal * normal.y) + (point.binormal * normal.x);

        (position, normal)
    };
    scene.add_model_transform(model, transform, None, is_ghost)
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
    let mesh_count = (0.5 + track_section.length / track.length).floor() as usize;
    let scale = track_section.length / (mesh_count as f32 * track.length);
    let length = scale * track.length;

    let mut scene = renderer::SceneBuilder::new(render_device)?;

    add_model_to_scene(&mut scene, &models.track, Some(true), track_section, scale, -length)?;
    add_model_to_scene(
        &mut scene,
        &models.track,
        Some(true),
        track_section,
        scale,
        track_section.length,
    )?;

    for i in 0..mesh_count {
        add_model_to_scene(&mut scene, &models.track, None, track_section, scale, i as f32 * length)?;
    }

    let scene = scene.build();

    for i in 0..4 {
        let view_rotation = glam::Mat4::from_rotation_y(90.0_f32.to_radians() * i as f32);
        let camera = camera * view_rotation;

        let view_rotation_inverse = view_rotation.inverse();
        let lights = lights.iter().map(|x| x.transform(&view_rotation_inverse)).collect::<Vec<_>>();

        let framebuffer = renderer::render_scene(&scene, &camera, &lights, 4, 4);
        let image = framebuffer.into_cropped_indexed_image(dither);

        let image_name = format!("{}_{i}", track_section.name);
        image.save(&output_directory.join(&track.name).join(image_name).with_extension("png"))?;
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

    for track in &track_desc.tracks {
        let models = track.models.load(base_directory)?;

        render_track_section(
            &render_device,
            &camera,
            &lights,
            &models,
            track_desc.dither,
            track,
            &track_sections::FLAT,
            output_directory,
        )?;

        render_track_section(
            &render_device,
            &camera,
            &lights,
            &models,
            track_desc.dither,
            track,
            &track_sections::GENTLE,
            output_directory,
        )?;

        render_track_section(
            &render_device,
            &camera,
            &lights,
            &models,
            track_desc.dither,
            track,
            &track_sections::MEDIUM_TURN_LEFT,
            output_directory,
        )?;
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
