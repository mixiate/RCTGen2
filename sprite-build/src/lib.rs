fn load_image(path: &std::path::Path) -> anyhow::Result<renderer::image::IndexedImage> {
    let image = image::ImageReader::open(path)?.decode()?.to_rgba8();
    let width = usize::try_from(image.width())?;
    let height = usize::try_from(image.height())?;

    let pixels = {
        let mut pixels = vec![0; width * height];
        for (source, dest) in image.pixels().zip(pixels.iter_mut()) {
            let dest_index = if source.0[3] == 255 {
                renderer::palette::PALETTE.iter().position(|x| *x == source.0[0..3]).unwrap_or(0)
            } else {
                0
            };
            *dest = u8::try_from(dest_index)?;
        }
        pixels
    };

    Ok(renderer::image::IndexedImage::with_buffer(pixels, width, height))
}

fn load_image_keep_palette(path: &std::path::Path) -> anyhow::Result<renderer::image::IndexedImage> {
    use anyhow::Context as _;

    let file = std::fs::File::open(path)?;
    let decoder = png::Decoder::new(std::io::BufReader::new(file));
    let mut reader = decoder.read_info()?;

    let buffer_size = reader.output_buffer_size().context("Error reading buffer size")?;
    let mut buffer = vec![0; buffer_size];
    let info = reader.next_frame(&mut buffer)?;
    buffer.truncate(info.buffer_size());

    anyhow::ensure!(info.color_type == png::ColorType::Indexed, "Image is not indexed");

    // assume that these images are empty
    if info.bit_depth == png::BitDepth::One {
        return Ok(renderer::image::IndexedImage::new(1, 1));
    }

    anyhow::ensure!(info.bit_depth == png::BitDepth::Eight, "Image bit depth is not 1 or 8");

    let width = usize::try_from(info.width)?;
    let height = usize::try_from(info.height)?;

    Ok(renderer::image::IndexedImage::with_buffer(buffer, width, height))
}

struct Sprite {
    encoded_sprite: rct::csg::EncodedSprite,
    x: i32,
    y: i32,
}

fn encode_sprite(
    sprite: &openrct2::objects::image::ImageFile,
    base_directory: &std::path::Path,
) -> anyhow::Result<Sprite> {
    use anyhow::Context as _;

    let image_file_path = base_directory.join(&sprite.path);
    let image = if sprite.palette == Some(openrct2::objects::image::PaletteType::Keep) {
        load_image_keep_palette(&image_file_path)
    } else {
        load_image(&image_file_path)
    };
    let mut image = image.with_context(|| format!("Could not read image file {}", image_file_path.display()))?;

    image.offset.x = sprite.x.unwrap_or_default();
    image.offset.y = sprite.y.unwrap_or_default();
    image.crop();

    Ok(Sprite {
        encoded_sprite: rct::csg::EncodedSprite::new(image.as_raw(), image.width(), image.height()),
        x: image.offset.x,
        y: image.offset.y,
    })
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
struct Sprites {
    sprites: Vec<openrct2::objects::image::ImageFile>,
}

pub fn build(sprites_json_path: &std::path::Path, output_file_path: &std::path::Path) -> anyhow::Result<()> {
    use anyhow::Context as _;
    use rayon::prelude::*;

    let base_directory = sprites_json_path
        .parent()
        .with_context(|| format!("Could not get parent of path {}", sprites_json_path.display()))?;

    let sprites = std::fs::read_to_string(sprites_json_path)
        .with_context(|| format!("Could not read file {}", sprites_json_path.display()))?;
    let sprites = serde_json::from_str::<Sprites>(&sprites)
        .with_context(|| format!("Could not parse json in file {}", sprites_json_path.display()))?;

    let sprites = sprites
        .sprites
        .into_par_iter()
        .map(|sprite| encode_sprite(&sprite, base_directory))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut sprite_archive = rct::csg::Archive::with_capacity(sprites.len());
    for sprite in sprites {
        sprite_archive.add_encoded_sprite(&sprite.encoded_sprite, sprite.x, sprite.y);
    }

    sprite_archive
        .save(output_file_path)
        .with_context(|| format!("Could not save {}", output_file_path.display()))?;

    Ok(())
}
