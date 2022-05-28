use fltk::prelude::{WidgetBase, GroupExt, WidgetExt};

fn main() {
    // Setup
    let program = fltk::app::App::default();
    let width: u64 = 600;
    let height: u64 = 600;
    let mut window = fltk::window::Window::new(100, 100, width as i32, height as i32, "Test");
    let mut frame = fltk::frame::Frame::default().center_of(&window);
    let mut data: Vec<u8> = vec![0; width as usize * height as usize];

    // Draw
    for y in 0..height {
        for x in 0..width {
            let c = num::complex::Complex::new((x as f64) / (width as f64) * 4f64 - 2f64, (y as f64) / (height as f64) * 4f64 - 2f64);
            let mut z = num::complex::Complex::new(0f64, 0f64);
            let mut iter_count: u64 = 0;
            while z.norm() < 2f64 && iter_count < 256 {
                iter_count += 1;
                z = z * z + c;
            }
            data[(y as usize) * (width as usize) + (x as usize)] = iter_count as u8;
        }
    }
    data[0] = 255;
    let image = fltk::image::RgbImage::new(&data, width as i32, height as i32, fltk::enums::ColorDepth::L8).unwrap();
    frame.set_image(Some(image));

    // End
    window.end();
    window.show();
    program.run().unwrap();
}
