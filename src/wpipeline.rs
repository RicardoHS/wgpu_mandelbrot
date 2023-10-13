use std::borrow::Cow;

use wgpu::util::DeviceExt;
use wgpu::RenderPipeline;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
pub struct WPipeline {
    pub pipeline: RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl WPipeline {
    fn init_vertices() -> (&'static [Vertex], &'static [u16]) {
        let vertices: &[Vertex] = &[
            Vertex {
                position: [-1.0, 1.0, 0.0],
                color: [1., 0.0, 0.],
            }, // A
            Vertex {
                position: [-1.0, -1.0, 0.0],
                color: [0.0, 1.0, 0.],
            }, // B
            Vertex {
                position: [1.0, 1.0, 0.0],
                color: [0.0, 0.0, 1.0],
            }, // C
            Vertex {
                position: [1.0, -1.0, 0.0],
                color: [0.1, 0.2, 0.3],
            }, // D
        ];

        let indices: &[u16] = &[0, 1, 2, 1, 3, 2];

        (vertices, indices)
    }

    pub fn new_render_pipeline(
        device: &wgpu::Device,
        bind_groups_layouts: &[&wgpu::BindGroupLayout],
        shader_code: Cow<'_, str>,
        texture_format: wgpu::TextureFormat,
        label: &str,
    ) -> WPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&format!("{}: Render Pipeline Layout", label)[..]),
                bind_group_layouts: bind_groups_layouts,
                push_constant_ranges: &[],
            });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{}: Shader Code", label)[..]),
            source: wgpu::ShaderSource::Wgsl(shader_code),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("{}: Render Pipeline", label)[..]),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        let (vertices, indices) = Self::init_vertices();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{}: Vertex Buffer", label)[..]),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{}: Index Buffer", label)[..]),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::UNIFORM,
        });
        let num_indices = indices.len() as u32;

        WPipeline {
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }
}
