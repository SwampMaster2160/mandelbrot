use fltk::prelude::{WidgetBase, GroupExt, WidgetExt, WindowExt};

fn render_mandelbrot(window_width: u32, window_height: u32, width: f64, aspect_ratio: f64, centre: num::complex::Complex<f64>, max_iter: u64) -> Vec<u8> {
	// Initial calculations
	let screen_aspect_ratio = (window_width as f64) / (window_height as f64);
	let applied_aspect_ratio = aspect_ratio / screen_aspect_ratio;
	
	// Render (not draw)
	let mut data: Vec<u8> = vec![0; window_width as usize * window_height as usize];
	for y in 0..window_height {
		for x in 0..window_width {
			let c = num::complex::Complex::new((x as f64) / (window_width as f64) - 0.5, ((y as f64) / (window_height as f64) - 0.5) * applied_aspect_ratio) * width + centre;
			let mut z = num::complex::Complex::new(0f64, 0f64);
			let mut iter_count: u64 = 0;
			while z.norm() < 2f64 && iter_count < max_iter {
				iter_count += 1;
				z = z * z + c;
			}
			data[(y as usize) * (window_width as usize) + (x as usize)] = ((iter_count as f64).log(max_iter as f64) * 256.) as u8;
		}
	}

	return data;
}

fn main() {
	// Vars
	let pixel_width: u32 = 640;
	let pixel_height: u32 = 480;
	let centre = num::complex::Complex::new(-0.5, 0.);
	let width = 4.;
	let aspect_ratio = 1.;
	let max_iter: u64 = 1000;

	// Calculation
	let program = fltk::app::App::default();
	let mut window = fltk::window::Window::new(0, 0, pixel_width as i32, pixel_height as i32, "Test").center_screen();
	window.make_resizable(true);
	let mut frame = fltk::frame::Frame::default().center_of(&window);
	let data = render_mandelbrot(pixel_width, pixel_height, width, aspect_ratio, centre, max_iter);
	let image = fltk::image::RgbImage::new(&data, pixel_width as i32, pixel_height as i32, fltk::enums::ColorDepth::L8).unwrap();
	frame.set_image(Some(image));

	// Display
	window.end();
	window.show();
	while program.wait() {
		frame.handle(
			move |f, event| match event {
				fltk::enums::Event::Resize => {
					let window = f.parent().unwrap();
					let window_width = window.width() as u32;
					let window_height = window.height() as u32;
					let data = render_mandelbrot(window_width, window_height, width, aspect_ratio, centre, max_iter);
					let image = fltk::image::RgbImage::new(&data, window_width as i32, window_height as i32, fltk::enums::ColorDepth::L8).unwrap();
					f.set_image(Some(image));
					return true;
				},
				_ => false
			}
		);
	}
}
