use crate::defs;

use pixels::wgpu;
use pixels::wgpu::util::DeviceExt as _;
use wgpu::BindGroup;
use wgpu::Buffer;
use wgpu::RenderPipeline;

pub struct Shader {
    pub render_pipeline: RenderPipeline,
    pub bind_group: BindGroup,
    pub uniform_buffer: Buffer,

    pub params: defs::Params,
}

impl Shader {
    pub fn new(size: winit::dpi::PhysicalSize<u32>, pixels: &pixels::Pixels) -> Self {
        let mut params = defs::Params::new();

        params.set_size(size);

        let uniform_buffer =
            pixels
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("create_buffer_init"),
                    contents: bytemuck::cast_slice(&[params]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let bind_group_layout =
            pixels
                .device()
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

        let bind_group = pixels
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("create_bind_group"),
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });

        let render_pipeline = {
            let shader = pixels
                .device()
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("create_shader_module"),
                    source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
                });

            let pipeline_layout =
                pixels
                    .device()
                    .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("Pipeline Layout"),
                        bind_group_layouts: &[&bind_group_layout],
                        push_constant_ranges: &[],
                    });

            pixels
                .device()
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
                })
        };

        Self {
            render_pipeline,
            bind_group,
            uniform_buffer,
            params,
        }
    }
}
