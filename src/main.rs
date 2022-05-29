use fltk::prelude::{WidgetBase, GroupExt, WidgetExt, WindowExt};

fn render_mandelbrot(frame: &mut fltk::frame::Frame/*, window_width: u32, window_height: u32*/, width: f64, aspect_ratio: f64, centre: num::complex::Complex<f64>, max_iter: u64) {
	// Initial calculations
	let window = frame.parent().unwrap();
	let window_width = window.width() as u32;
	let window_height = window.height() as u32;

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

	// Draw
	let image = fltk::image::RgbImage::new(&data, window_width as i32, window_height as i32, fltk::enums::ColorDepth::L8);
	frame.set_image(image.ok());
}

fn main() {
	// Vars
	let mut pixel_width: u32 = 640;
	let mut pixel_height: u32 = 480;
	let centre = num::complex::Complex::new(0., 0.);
	let width = 4.;
	let aspect_ratio = 1.;
	let max_iter: u64 = 256;

	// Calculation
	let app = fltk::app::App::default();
	let mut window = fltk::window::Window::new(0, 0, pixel_width as i32, pixel_height as i32, "Mandelbrot").center_screen();
	window.make_resizable(true);
	let mut frame = fltk::frame::Frame::default().center_of(&window);
	render_mandelbrot(&mut frame, width, aspect_ratio, centre, max_iter);

	// Display
	window.end();
	frame.handle(move |frame, event| match event {
		fltk::enums::Event::Resize => {
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
