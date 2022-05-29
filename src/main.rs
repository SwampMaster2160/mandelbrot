use fltk::prelude::{WidgetBase, GroupExt, WidgetExt, WindowExt};

fn main() {
    // Setup
    let pixel_width: u64 = 640;
    let pixel_height: u64 = 480;
    let centre = num::complex::Complex::new(-0.5, 0.);
    let width = 4.;
    let aspect_ratio = 1.;
    let max_iter: u64 = 1000;

    let screen_aspect_ratio = (pixel_width as f64) / (pixel_height as f64);
    let applied_aspect_ratio = aspect_ratio / screen_aspect_ratio;

    let program = fltk::app::App::default();
    let mut window = fltk::window::Window::new(0, 0, pixel_width as i32, pixel_height as i32, "Test").center_screen();
    let mut frame = fltk::frame::Frame::default().center_of(&window);
    let mut data: Vec<u8> = vec![0; pixel_width as usize * pixel_height as usize];

    // Draw
    for y in 0..pixel_height {
        for x in 0..pixel_width {
            let c = num::complex::Complex::new((x as f64) / (pixel_width as f64) - 0.5, ((y as f64) / (pixel_height as f64) - 0.5) * applied_aspect_ratio) * width + centre;
            let mut z = num::complex::Complex::new(0f64, 0f64);
            let mut iter_count: u64 = 0;
            while z.norm() < 2f64 && iter_count < max_iter {
                iter_count += 1;
                z = z * z + c;
            }
            data[(y as usize) * (pixel_width as usize) + (x as usize)] = ((iter_count as f64).log(max_iter as f64) * 256.) as u8;
        }
    }
    let image = fltk::image::RgbImage::new(&data, pixel_width as i32, pixel_height as i32, fltk::enums::ColorDepth::L8).unwrap();
    frame.set_image(Some(image));

    // End
    window.end();
    window.show();
    program.run().unwrap();
}
