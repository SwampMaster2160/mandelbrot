use num::complex;
use fltk::{frame, image, enums, app, window};
use fltk::prelude::{WidgetBase, GroupExt, WidgetExt, WindowExt};

fn render_mandelbrot(frame: &mut frame::Frame, width: f64, aspect_ratio: f64, centre: complex::Complex<f64>, max_iter: u64) {
	// Initial calculations
	let window = frame.parent().unwrap();
	let window_width = window.width() as u32;
	let window_height = window.height() as u32;

	let screen_aspect_ratio = (window_width as f64) / (window_height as f64);
	let applied_aspect_ratio = aspect_ratio / screen_aspect_ratio;
	
	// Render
	let mut data: Vec<u8> = vec![0; window_width as usize * window_height as usize];
	let mut c = complex::Complex::new(0., 0.);
	for y in 0..window_height {
		c.im = ((y as f64) / (window_height as f64) - 0.5) * applied_aspect_ratio * width + centre.im;
		for x in 0..window_width {
			c.re = ((x as f64) / (window_width as f64) - 0.5) * width + centre.re;
			let mut z = complex::Complex::new(0f64, 0f64);
			let mut iter_count: u64 = 0;
			while z.re * z.re + z.im * z.im < 4. && iter_count < max_iter {
				let re_temp = z.re * z.re - z.im * z.im + c.re;
				z.im = 2. * z.re * z.im + c.im;
				z.re = re_temp;
				iter_count += 1;
			}
			data[(y as usize) * (window_width as usize) + (x as usize)] = ((iter_count as f64).log(max_iter as f64) * 256.) as u8;
		}
	}

	// Draw
	let image = image::RgbImage::new(&data, window_width as i32, window_height as i32, enums::ColorDepth::L8);
	frame.set_image(image.ok());
}

fn main() {
	// Vars
	let mut pixel_width: u32 = 640;
	let mut pixel_height: u32 = 480;
	let centre = complex::Complex::new(0., 0.);
	let width = 4.;
	let aspect_ratio = 1.;
	let max_iter: u64 = 256;

	// Calculation
	let app = app::App::default();
	let mut window = window::Window::new(0, 0, pixel_width as i32, pixel_height as i32, "Mandelbrot").center_screen();
	window.make_resizable(true);
	let mut frame = frame::Frame::default().center_of(&window);
	render_mandelbrot(&mut frame, width, aspect_ratio, centre, max_iter);

	// Display
	window.end();
	frame.handle(move |frame, event| match event {
		enums::Event::Resize => {
			let window = frame.parent().unwrap();
			let new_width = window.width() as u32;
			let new_height = window.height() as u32;
			if new_width == pixel_width && new_height == pixel_height
			{
				return true;
			}
			pixel_width = new_width;
			pixel_height = new_height;
			render_mandelbrot(frame, width, aspect_ratio, centre, max_iter);
			return true;
		},
		_ => false
	});
	window.show();
	app.run().unwrap();
}
