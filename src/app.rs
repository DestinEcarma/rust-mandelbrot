use crate::camera::Camera;
use crate::defs::{self, Result};
use crate::error::Error;
use crate::shader::Shader;

use log::error;
use pixels::{wgpu, Pixels, SurfaceTexture};
use std::cell::{Ref, RefCell, RefMut};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{CursorIcon, Window};

#[derive(Default)]
pub struct App<'a> {
    window: Option<Arc<Window>>,
    shader: Option<RefCell<Shader>>,
    camera: Option<RefCell<Camera>>,
    pixels: Option<RefCell<Pixels<'a>>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.init(event_loop) {
            error!("Failed to initialize app: {e}");
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
            // TODO: Implement changing the number of iterations.
            // TODO: Implement changing the color scheme.
            // TODO: Implement saving the fractal to an image file.
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                let position = (position.x as f32, position.y as f32);

                if let Err(e) = self.pan((position.0, position.1)) {
                    error!("Failed to pan: {e}");
                    event_loop.exit();
                    return;
                }

                if let Err(e) = self.update_mouse_position(position) {
                    error!("Failed to update mouse position: {e}");
                    event_loop.exit();
                }
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                if let Err(e) = self.prepare_pan(button, state) {
                    error!("Failed to prepare pan: {e}");
                    event_loop.exit();
                }
            }
            WindowEvent::MouseWheel {
                delta,
                device_id: _,
                phase: _,
            } => {
                let y = match delta {
                    MouseScrollDelta::LineDelta(_x, y) => y,
                    MouseScrollDelta::PixelDelta(delta) => {
                        if delta.y > 0.0 {
                            1.0
                        } else {
                            -1.0
                        }
                    }
                };

                if let Err(e) = self.zoom(y) {
                    error!("Failed to zoom: {e}");
                    event_loop.exit();
                }
            }
            _ => (),
        }
    }
}

impl App<'_> {
    /// Create a new app with the given number of iterations.
    pub fn new(_iterations: u32) -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl<'a> App<'a> {
    /// Get a reference to the window.
    fn window(&self) -> Result<Arc<Window>> {
        self.window
            .as_ref()
            .map_or(Err(Error::NoWindow), |value| Ok(value.clone()))
    }

    /// Get a reference to the pixels buffer.
    fn pixels(&self) -> Result<Ref<'_, Pixels<'a>>> {
        self.pixels
            .as_ref()
            .map_or(Err(Error::NoPixels), |value| Ok(value.borrow()))
    }

    /// Get a mutable reference to the pixels buffer.
    fn pixels_mut(&self) -> Result<RefMut<'_, Pixels<'a>>> {
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

impl App<'_> {
    /// Initialize the app.
    fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        let window = Arc::new(event_loop.create_window(defs::init_window())?);

        let size = window.inner_size();

        let pixels = {
            let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
            Pixels::new(size.width, size.height, surface_texture).expect("Failed to create pixels")
        };

        let shader = Shader::new(size, &pixels);
        let camera = Camera::new(size);

        self.window = Some(window);
        self.shader = Some(RefCell::new(shader));
        self.pixels = Some(RefCell::new(pixels));
        self.camera = Some(RefCell::new(camera));

        self.render()?;
        self.window()?.set_visible(true);

        Ok(())
    }

    /// Render the app, drawing the fractal to the pixels buffer.
    fn render(&mut self) -> Result<()> {
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
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(render_pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.draw(0..3, 0..1);

            Ok(())
        })?;

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

        Ok(())
    }
}

impl App<'_> {
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

        camera.zoom(delta);

        shader.params.set_scale(camera.scale);
        shader.params.set_center(camera.world_position);

        self.pixels()?.queue().write_buffer(
            &shader.uniform_buffer,
            0,
            bytemuck::cast_slice(&[shader.params]),
        );

        self.window()?.request_redraw();

        Ok(())
    }

    /// Prepare the panning of the camera.
    pub fn prepare_pan(&self, button: MouseButton, state: ElementState) -> Result<()> {
        let mut camera = self.camera_mut()?;
        let window = self.window()?;

        match (button, state) {
            (MouseButton::Left, ElementState::Pressed) => {
                window.set_cursor(CursorIcon::Grabbing);
                camera.mouse_pressed = true;
            }
            (MouseButton::Left, ElementState::Released) => {
                window.set_cursor(CursorIcon::Default);
                camera.mouse_pressed = false;
            }
            _ => (),
        }

        Ok(())
    }

    /// Pan the camera given position. The delta will be calculated based on the current
    pub fn pan(&mut self, position: (f32, f32)) -> Result<()> {
        let mut camera = self.camera_mut()?;

        if !camera.mouse_pressed {
            return Ok(());
        }

        let mut shader = self.shader_mut()?;
        let pixels = self.pixels()?;

        let delta = (
            position.0 - camera.mouse_position.0,
            position.1 - camera.mouse_position.1,
        );

        camera.pan(delta);

        shader.params.set_center(camera.world_position);

        pixels.queue().write_buffer(
            &shader.uniform_buffer,
            0,
            bytemuck::cast_slice(&[shader.params]),
        );

        self.window()?.request_redraw();

        Ok(())
    }
}
