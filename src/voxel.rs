use cgmath::Vector3;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
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

const SIZE: f32 = 16.0;

pub struct Voxel {
    pub corners: [Vertex; 8],
}

impl Voxel {
    pub fn new(pos: Vector3<f32>) -> Self {
        Self {
            corners: [
                Vertex {
                    position: [pos.x, pos.y, pos.z],
                },
                Vertex {
                    position: [pos.x + SIZE, pos.y, pos.z],
                },
                Vertex {
                    position: [pos.x + SIZE, pos.y + SIZE, pos.z],
                },
                Vertex {
                    position: [pos.x, pos.y + SIZE, pos.z],
                },
                Vertex {
                    position: [pos.x, pos.y, pos.z + SIZE],
                },
                Vertex {
                    position: [pos.x + SIZE, pos.y, pos.z + SIZE],
                },
                Vertex {
                    position: [pos.x + SIZE, pos.y + SIZE, pos.z + SIZE],
                },
                Vertex {
                    position: [pos.x, pos.y + SIZE, pos.z + SIZE],
                },
            ],
        }
    }

    #[rustfmt::skip]
    pub fn get_index_buffer(&self) -> [u16; 36] {
        [
            0, 1, 3,
            1, 2, 3,
            4, 0, 7,
            0, 3, 7,
            1, 5, 2,
            5, 6, 2,
            3, 2, 7,
            2, 6, 7,
            4, 5, 0,
            5, 1, 0,
            7, 6, 4,
            6, 5, 4,
        ]
    }
}
