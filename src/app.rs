use crate::defs::{self, Result};
use crate::error::Error;

use log::error;
use pixels::wgpu::util::DeviceExt as _;
use pixels::wgpu::{BindGroup, Buffer, RenderPipeline};
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

    render_pipeline: Option<RenderPipeline>,
    bind_group: Option<BindGroup>,
    uniform_buffer: Option<Buffer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(e) = self.init(event_loop) {
            error!("Failed to initialize app: {e}");
            event_loop.exit();
            return;
        }

        if let Err(e) = self.draw() {
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
                if let Err(e) = self.draw() {
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

    /// Get a reference to the render pipeline.
    fn render_pipeline(&self) -> Result<&RenderPipeline> {
        self.render_pipeline.as_ref().ok_or(Error::NoRenderPipeline)
    }

    /// Get a reference to the bind group.
    fn bind_group(&self) -> Result<&BindGroup> {
        self.bind_group.as_ref().ok_or(Error::NoBindGroup)
    }

    /// Get a reference to the uniform buffer.
    fn uniform_buffer(&self) -> Result<&Buffer> {
        self.uniform_buffer.as_ref().ok_or(Error::NoUniformBuffer)
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

        let pixels_ctx = pixels.context();

        let params = defs::Params {
            max_iter: 256,
            width: size.width,
            height: size.height,
        };

        let uniform_buffer =
            pixels_ctx
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("create_buffer_init"),
                    contents: bytemuck::cast_slice(&[params]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let bind_group_layout =
            pixels_ctx
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("create_bind_group_layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let bind_group = pixels_ctx
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("create_bind_group"),
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });

        let pipeline_layout =
            pixels_ctx
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let shader = pixels_ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("create_shader_module"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });

        let render_pipeline =
            pixels_ctx
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("create_render_pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "vs_main",
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: pixels.surface_texture_format(),
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                });

        self.window = Some(window);
        self.pixels = Some(RefCell::new(pixels));

        self.render_pipeline = Some(render_pipeline);
        self.bind_group = Some(bind_group);

        self.uniform_buffer = Some(uniform_buffer);

        Ok(())
    }

    /// Draw the fractal to the pixels buffer.
    fn draw(&mut self) -> Result<()> {
        let window = self.window()?;
        let pixels = self.pixels()?;

        let render_pipeline = self.render_pipeline()?;
        let bind_group = self.bind_group()?;

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

        pixels.resize_buffer(size.width, size.height)?;
        pixels.resize_surface(size.width, size.height)?;

        pixels.queue().write_buffer(
            self.uniform_buffer()?,
            0,
            bytemuck::cast_slice(&[defs::Params {
                max_iter: 256,
                width: size.width,
                height: size.height,
            }]),
        );

        Ok(())
    }
}
