use crate::defs::Result;

use pixels::{Pixels, SurfaceTexture};
use std::mem::{self, MaybeUninit};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub struct App {
    window: MaybeUninit<Window>,
    pixels: MaybeUninit<Pixels>,
    initialized: bool,
    iterations: u32,
}

impl Drop for App {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                mem::drop(self.window.assume_init_read());
                mem::drop(self.pixels.assume_init_read());
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.init(event_loop) {
            // TODO: Use `log` crate.
            eprintln!("Failed to initialize app: {e}");
            event_loop.exit();
        }

        if let Err(e) = self.draw() {
            // TODO: Use `log` crate.
            eprintln!("Failed to draw: {e}");
            event_loop.exit();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Err(e) = self.resize(size) {
                    // TODO: Use `log` crate.
                    eprintln!("Failed to resize pixels: {e}");
                    event_loop.exit();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(e) = self.draw() {
                    // TODO: Use `log` crate.
                    eprintln!("Failed to draw: {e}");
                    event_loop.exit();
                }
            }
            // TODO: Implement zoom and pan features.
            // TODO: Implement changing the number of iterations.
            // TODO: Implement changing the color scheme.
            // TODO: Implement saving the fractal to an image file.
            _ => (),
        }
    }
}

impl App {
    /// Create a new app with the given number of iterations.
    pub fn new(iterations: u32) -> Self {
        Self {
            window: MaybeUninit::uninit(),
            pixels: MaybeUninit::uninit(),
            initialized: false,
            iterations,
        }
    }
}

impl App {
    /// Get a reference to the window.
    fn window(&self) -> &Window {
        unsafe { self.window.assume_init_ref() }
    }

    /// Get a mutable reference to the pixels.
    fn pixels_mut(&mut self) -> &mut Pixels {
        unsafe { self.pixels.assume_init_mut() }
    }
}

impl App {
    /// Initialize the app.
    fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        let window = event_loop.create_window(crate::defs::init_window())?;

        let pixels = {
            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(size.width, size.height, surface_texture).expect("Failed to create pixels")
        };

        self.window.write(window);
        self.pixels.write(pixels);
        self.initialized = true;

        Ok(())
    }

    /// Draw the fractal to the pixels buffer.
    fn draw(&mut self) -> Result<()> {
        let max_iter = self.iterations;
        println!("iterations: {}", max_iter);

        let size = self.window().inner_size();

        let pixels = self.pixels_mut();

        for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            let x = i % size.width as usize;
            let y = i / size.width as usize;

            let (c_re, c_im) = App::pixel_to_complex(x as f64, y as f64, size);
            let iterations = App::iterations(c_re, c_im, max_iter);

            let color = App::iterations_to_color(iterations, max_iter);

            pixel.copy_from_slice(&color);
        }

        pixels.render()?;

        let window = self.window();

        if let Some(false) = window.is_visible() {
            window.set_visible(true);
        }

        Ok(())
    }

    /// Resize the pixels buffer and surface to the given size.
    fn resize(&mut self, size: PhysicalSize<u32>) -> Result<()> {
        let pixels = self.pixels_mut();

        pixels.resize_buffer(size.width, size.height)?;
        pixels.resize_surface(size.width, size.height)?;

        Ok(())
    }
}

impl App {
    /// Convert a pixel coordinate to a complex number.
    fn pixel_to_complex(x: f64, y: f64, size: PhysicalSize<u32>) -> (f64, f64) {
        let (real_min, real_max, imag_min, imag_max) = crate::defs::BOUNDS;

        (
            real_min + (x / size.width as f64) * (real_max - real_min),
            imag_min + (y / size.height as f64) * (imag_max - imag_min),
        )
    }

    /// Calculate the number of iterations for the given complex number.
    fn iterations(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
        let mut z_re = c_re;
        let mut z_im = c_im;

        for i in 0..max_iter {
            let z_re2 = z_re * z_re;
            let z_im2 = z_im * z_im;

            if z_re2 + z_im2 > 4.0 {
                return i;
            }

            z_im = 2.0 * z_re * z_im + c_im;
            z_re = z_re2 - z_im2 + c_re;
        }

        max_iter
    }

    /// Convert the number of iterations to a color.
    fn iterations_to_color(iterations: u32, max_iter: u32) -> [u8; 4] {
        if iterations == max_iter {
            return [0x00, 0x00, 0x00, 0xff];
        }

        let t = iterations as f64 / max_iter as f64;
        let r = (9.0 * (1.0 - t) * t * t * t * 255.0) as u8;
        let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0) as u8;
        let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0) as u8;

        [r, g, b, 0xff]
    }
}
