#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    Empty,
    Grass,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
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

pub const SIZE: f32 = 1.0;
pub const HALF_SIZE: f32 = 0.5;

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

    pub fn set_block_type(&mut self, block_type: BlockType) {
        self.block_type = block_type;
    }

    pub fn get_vertex(x: f32, y: f32, z: f32, i: usize) -> Option<Vertex> {
        match i {
            0 => {
                Some(Vertex { position: [x, y, z], color: [0.3, 0.3, 0.3] })
            },
            1 => {
                Some(Vertex { position: [x + SIZE, y, z], color: [0.3, 0.3, 0.3] })
            },
            2 => {
                Some(Vertex { position: [x + SIZE, y + SIZE, z], color: [0.3, 0.3, 0.3] })
            },
            3 => {
                Some(Vertex { position: [x, y + SIZE, z], color: [0.3, 0.3, 0.3] })
            },
            4 => {
                Some(Vertex { position: [x, y, z + SIZE], color: [0.3, 0.3, 0.3] })
            },
            5 => {
                Some(Vertex { position: [x + SIZE, y, z + SIZE], color: [0.3, 0.3, 0.3] })
            },
            6 => {
                Some(Vertex { position: [x + SIZE, y + SIZE, z + SIZE], color: [0.3, 0.3, 0.3] })
            },
            7 => {
                Some(Vertex { position: [x, y + SIZE, z + SIZE], color: [0.3, 0.3, 0.3] })
            },
            _ => None,
        }
    }

    /*
    pub fn new(pos: Vector3<f32>) -> Self {
        Self {
            corners: [
                Vertex {
                    position: [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                    color: [0.3, 0.2, 0.1],
                },
                Vertex {
                    position: [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                    color: [0.4, 0.5, 0.6],
                },
                Vertex {
                    position: [pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                    color: [0.7, 0.7, 0.7],
                },
                Vertex {
                    position: [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                    color: [0.4, 0.4, 0.4],
                },
                Vertex {
                    position: [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                    color: [0.1, 0.3, 0.9],
                },
                Vertex {
                    position: [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                    color: [0.8, 0.4, 0.5],
                },
                Vertex {
                    position: [pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE],
                    color: [0.3, 0.9, 0.3],
                },
                Vertex {
                    position: [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE],
                    color: [0.3, 0.7, 0.1],
                },
            ],
        }
    }
    */

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
