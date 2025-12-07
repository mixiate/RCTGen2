pub(crate) struct Texture {
    pixels: Vec<glam::Vec3>,
    width: usize,
    height: usize,
}

fn read_png(file: &std::fs::File, greyscale: bool) -> anyhow::Result<Texture> {
    use anyhow::Context as _;

    let decoder = png::Decoder::new(std::io::BufReader::new(file));
    let mut reader = decoder.read_info()?;

    let output_buffer_size = reader.output_buffer_size().context("Could not get output buffer size")?;
    let mut png_buffer = vec![0; output_buffer_size];
    let info = reader.next_frame(&mut png_buffer)?;

    if info.bit_depth != png::BitDepth::Eight {
        anyhow::bail!("Bit depth is not 8");
    }

    let width = usize::try_from(info.width)?;
    let height = usize::try_from(info.height)?;

    let pixels_buffer = &png_buffer[..info.buffer_size()];

    let mut pixels: Vec<glam::Vec3> = match info.color_type {
        png::ColorType::Rgb => {
            pixels_buffer.as_chunks::<3>().0.iter().map(crate::palette::srgb_to_linear_rgb).collect()
        }
        png::ColorType::Rgba => pixels_buffer
            .as_chunks::<4>()
            .0
            .iter()
            .map(|x| crate::palette::srgb_to_linear_rgb(&[x[0], x[1], x[2]]))
            .collect(),
        _ => {
            anyhow::bail!("Format is not RGB or RGBA");
        }
    };

    if greyscale {
        pixels.iter_mut().for_each(|x| *x = crate::palette::linear_rgb_to_luminence_rgb(x));
    }

    Ok(Texture { pixels, width, height })
}

fn wrap_coord(x: f32) -> f32 {
    (x - x.floor()).clamp(0.0, 1.0)
}

impl Texture {
    pub(crate) fn load(path: &std::path::Path, greyscale: bool) -> anyhow::Result<Self> {
        use anyhow::Context as _;

        let file = std::fs::File::open(path).with_context(|| format!("Could not open {}", path.display()))?;
        read_png(&file, greyscale).with_context(|| format!("Could not decode {}", path.display()))
    }

    pub(crate) fn sample_wrapped(&self, uv: glam::Vec2) -> glam::Vec3 {
        let x = (wrap_coord(uv.x) * self.width as f32) as usize;
        let y = (wrap_coord(uv.y) * self.height as f32) as usize;
        self.pixels[x + self.width * y]
    }
}
