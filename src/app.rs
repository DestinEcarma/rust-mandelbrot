use crate::defs::{self, Result};
use crate::error::Error;
use crate::shader::Shader;

use log::error;
use pixels::{wgpu, Pixels, SurfaceTexture};
use std::cell::{self, RefCell};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    pixels: Option<RefCell<Pixels>>,
    shader: Option<Shader>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.init(event_loop) {
            error!("Failed to initialize app: {e}");
            event_loop.exit();
            return;
        }

        if let Err(e) = self.render() {
            error!("Failed to draw: {e}");
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
                    error!("Failed to resize pixels: {e}");
                    event_loop.exit();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(e) = self.render() {
                    error!("Failed to draw: {e}");
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
    pub fn new(_iterations: u32) -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl App {
    /// Get a reference to the window.
    fn window(&self) -> Result<&Window> {
        self.window.as_ref().ok_or(Error::NoWindow)
    }

    /// Get a reference to the pixels buffer.
    fn pixels(&self) -> Result<cell::Ref<'_, Pixels>> {
        self.pixels
            .as_ref()
            .map_or(Err(Error::NoPixels), |value| Ok(value.borrow()))
    }

    /// Get a mutable reference to the pixels buffer.
    fn pixels_mut(&self) -> Result<cell::RefMut<'_, Pixels>> {
        self.pixels
            .as_ref()
            .map_or(Err(Error::NoPixels), |value| Ok(value.borrow_mut()))
    }

    /// Get a reference to the shader.
    fn shader(&self) -> Result<&Shader> {
        self.shader.as_ref().ok_or(Error::NoShader)
    }
}

impl App {
    /// Initialize the app.
    fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        let window = event_loop.create_window(defs::init_window())?;

        let size = window.inner_size();

        let pixels = {
            let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
            Pixels::new(size.width, size.height, surface_texture).expect("Failed to create pixels")
        };

        let shader = Shader::new(size, &pixels);

        self.window = Some(window);
        self.pixels = Some(RefCell::new(pixels));
        self.shader = Some(shader);

        Ok(())
    }

    /// Render the app, drawing the fractal to the pixels buffer.
    fn render(&mut self) -> Result<()> {
        let window = self.window()?;
        let pixels = self.pixels()?;

        let render_pipeline = &self.shader()?.render_pipeline;
        let bind_group = &self.shader()?.bind_group;

        pixels.render_with(move |encoder, view, _ctx| {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("begin_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(render_pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.draw(0..3, 0..1);

            Ok(())
        })?;

        if let Some(false) = window.is_visible() {
            window.set_visible(true);
        }

        Ok(())
    }

    /// Resize the pixels buffer and surface to the given size.
    fn resize(&mut self, size: PhysicalSize<u32>) -> Result<()> {
        let mut pixels = self.pixels_mut()?;

        let shader = self.shader()?;

        pixels.resize_buffer(size.width, size.height)?;
        pixels.resize_surface(size.width, size.height)?;

        pixels.queue().write_buffer(
            &shader.uniform_buffer,
            0,
            bytemuck::cast_slice(&[shader.params.with_size(size)]),
        );

        Ok(())
    }
}
