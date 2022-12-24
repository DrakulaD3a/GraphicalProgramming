use crate::{render_utilities, texture};

use super::{vertex::Vertex, vertex_desc::VertexDesc};

#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    Empty,
    Grass,
}

pub const SIZE: f32 = 1.0;

#[derive(Debug, Clone, Copy)]
pub struct Voxel {
    pub block_type: BlockType,
}

impl Voxel {
    pub fn new_empty() -> Self {
        Self {
            block_type: BlockType::Empty,
        }
    }
}

pub fn create_voxel_pipeline(device: wgpu::Device, texture_format: wgpu::TextureFormat, light_bind_group_layout: &wgpu::BindGroupLayout, shader_module: wgpu::ShaderModule) -> wgpu::RenderPipeline {
    let visibility = wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT;
    let camera_bind_group_layout = render_utilities::create_bind_group_layout(&device, "camera_bind_group_layout", 0, visibility);

    let bind_group_layouts = &[&camera_bind_group_layout, &light_bind_group_layout];
    let pipeline_layout = render_utilities::create_pipeline_layout(&device, "voxel_pipeline", bind_group_layouts);

    render_utilities::create_render_pipeline(&device, &pipeline_layout, texture_format, Some(texture::Texture::DEPTH_FORMAT), &[Vertex::desc()], shader_module, "voxel pipeline")
}
