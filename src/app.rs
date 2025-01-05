use crate::camera::Camera;
use crate::defs::{self, Result};
use crate::error::Error;
use crate::shader::Shader;

use log::error;
use pixels::{wgpu, Pixels, SurfaceTexture};
use std::cell::{Ref, RefCell, RefMut};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    shader: Option<RefCell<Shader>>,
    camera: Option<RefCell<Camera>>,
    pixels: Option<RefCell<Pixels>>,
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
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                if let Err(e) = self.update_mouse_position((position.x as f32, position.y as f32)) {
                    error!("Failed to update mouse position: {e}");
                    event_loop.exit();
                }
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(_x, y),
                device_id: _,
                phase: _,
            } => {
                if let Err(e) = self.zoom(y) {
                    error!("Failed to zoom: {e}");
                    event_loop.exit();
                }
            }
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
    fn pixels(&self) -> Result<Ref<'_, Pixels>> {
        self.pixels
            .as_ref()
            .map_or(Err(Error::NoPixels), |value| Ok(value.borrow()))
    }

    /// Get a mutable reference to the pixels buffer.
    fn pixels_mut(&self) -> Result<RefMut<'_, Pixels>> {
        self.pixels
            .as_ref()
            .map_or(Err(Error::NoPixels), |value| Ok(value.borrow_mut()))
    }

    /// Get a reference to the shader.
    fn shader(&self) -> Result<Ref<'_, Shader>> {
        self.shader
            .as_ref()
            .map_or(Err(Error::NoShader), |value| Ok(value.borrow()))
    }

    /// Get a mutable reference to the shader.
    fn shader_mut(&self) -> Result<RefMut<'_, Shader>> {
        self.shader
            .as_ref()
            .map_or(Err(Error::NoShader), |value| Ok(value.borrow_mut()))
    }

    /// Get a mutable reference to the camera.
    fn camera_mut(&self) -> Result<RefMut<'_, Camera>> {
        self.camera
            .as_ref()
            .map_or(Err(Error::NoCamera), |value| Ok(value.borrow_mut()))
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
        let camera = Camera::new(size);

        self.window = Some(window);
        self.shader = Some(RefCell::new(shader));
        self.pixels = Some(RefCell::new(pixels));
        self.camera = Some(RefCell::new(camera));

        self.zoom(0.0)?;

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
        let mut shader = self.shader_mut()?;
        let mut camera = self.camera_mut()?;

        shader.params.set_size(size);
        camera.size = (size.width as f32, size.height as f32);

        pixels.resize_buffer(size.width, size.height)?;
        pixels.resize_surface(size.width, size.height)?;

        pixels.queue().write_buffer(
            &shader.uniform_buffer,
            0,
            bytemuck::cast_slice(&[shader.params]),
        );

        log::info!("Params: {:?}", shader.params);

        Ok(())
    }
}

impl App {
    /// Update the position of the mouse.
    pub fn update_mouse_position(&mut self, position: (f32, f32)) -> Result<()> {
        let mut camera = self.camera_mut()?;

        camera.mouse_position = position;

        Ok(())
    }

    /// Zoom the camera by the given delta.
    pub fn zoom(&mut self, delta: f32) -> Result<()> {
        let mut camera = self.camera_mut()?;
        let mut shader = self.shader_mut()?;
        let pixels = self.pixels()?;

        camera.zoom(delta);

        shader.params.set_scale(camera.scale);
        shader.params.set_center(camera.world_position);

        pixels.queue().write_buffer(
            &shader.uniform_buffer,
            0,
            bytemuck::cast_slice(&[shader.params]),
        );

        drop(pixels);
        drop(camera);
        drop(shader);

        self.render()?;

        Ok(())
    }
}
