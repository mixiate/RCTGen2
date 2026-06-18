use renderer::image::IndexedImage;

pub struct Atlas {
    pub image: IndexedImage,
    pub coords: Vec<glam::IVec2>,
}

pub fn create_atlas(images: &[IndexedImage], columns: i32) -> Atlas {
    let (x_min, y_min, x_max, y_max) = {
        let mut x_min = 0;
        let mut y_min = 0;
        let mut x_max = 0;
        let mut y_max = 0;

        for image in images {
            let x_neg = image.offset.x;
            let x_pos = i32::from(image.width()) + image.offset.x;
            if x_neg < x_min {
                x_min = x_neg;
            }
            if x_pos > x_max {
                x_max = x_pos;
            }
            let y_neg = image.offset.y;
            let y_pos = i32::from(image.height()) + image.offset.y;
            if y_neg < y_min {
                y_min = y_neg;
            }
            if y_pos > y_max {
                y_max = y_pos;
            }
        }
        (x_min, y_min, x_max, y_max)
    };

    let column_width = x_max - x_min;
    let row_height = y_max - y_min;

    let image_count = i32::try_from(images.len()).unwrap();
    let rows = image_count / columns;
    let rows = if image_count % columns != 0 { rows + 1 } else { rows };

    let width = u16::try_from(column_width * columns).unwrap();
    let height = u16::try_from(row_height * rows).unwrap();

    let mut grid_image = IndexedImage::new(width, height);
    let mut coords = vec![glam::IVec2::new(0, 0); images.len()];
    for ((i, image), coord) in images.iter().enumerate().zip(coords.iter_mut()) {
        let i = i32::try_from(i).unwrap();
        let row = i / columns;
        let column = i - (row * columns);
        coord.x = (column_width * column) - x_min + image.offset.x;
        coord.y = (row_height * row) - y_min + image.offset.y;

        grid_image.blit(image, coord.x.try_into().unwrap(), coord.y.try_into().unwrap());
    }

    Atlas {
        image: grid_image,
        coords,
    }
}
