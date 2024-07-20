use std::cmp::{max, min};
use std::path::PathBuf;
use imageproc::drawing;
use imageproc::rect::Rect;
use crate::stats::{get_dir_files};
use image_match::image::{grayscale_image};
use image_match::{crop_boundaries, grid_points, Bounds};
use image::io::Reader as ImageReader;

pub fn draw_debug(dir: &PathBuf, out_dir: &PathBuf) {
    for subdir in ["original", "cropped", "grown", "shrunk"] {
        let source = dir.join(subdir);
        let dest = out_dir.join(subdir);
        for name in get_dir_files(&source) {
            dbg!(&name);
            let mut image = ImageReader::open(&source.join(&name))
                .expect("Couldn't open image")
                .decode()
                .expect("Couldn't decode image");
            let gray = grayscale_image(image.clone());
            let bounds = crop_boundaries(&gray, 0.05);
            let points = grid_points(&bounds, 10);
            let mut out_gray = imageproc::image::GrayImage::from_raw(image.width(), image.height(), gray.into_iter().flatten().collect()).expect("Ahh");

            let width = bounds.upper_x - bounds.lower_x;
            let height = bounds.upper_y - bounds.lower_y;

            drawing::draw_hollow_rect_mut(&mut out_gray,
                Rect::at(bounds.lower_x as i32, bounds.lower_y as i32)
                    .of_size(width as u32, height as u32),
                imageproc::image::Luma([255])
            );
            drawing::draw_hollow_rect_mut(&mut out_gray,
                Rect::at((bounds.lower_x+1) as i32, (bounds.lower_y-1) as i32)
                    .of_size((width-2) as u32, (height-2) as u32),
                imageproc::image::Luma([0])
            );

            let square_edge = 
        max(
            2_usize,
            ((0.5 + min(width, height) as f32 / 20.0) as f32).floor() as usize,
        ) / 2;


            for (_, point) in points {
                drawing::draw_hollow_circle_mut(&mut out_gray,
                    (point.0 as i32, point.1 as i32), 
                    square_edge.try_into().unwrap(),
                    imageproc::image::Luma([255])
                );
                drawing::draw_hollow_circle_mut(&mut out_gray,
                    (point.0 as i32, point.1 as i32), 
                    (square_edge-1).try_into().unwrap(),
                    imageproc::image::Luma([0])
                );
            }
            out_gray.save(&dest.join(name)).expect("Coudln't save iamge");
        }
    }
}
