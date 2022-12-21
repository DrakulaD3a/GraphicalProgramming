use crate::voxel;
use crate::quad::{Quad, Direction};

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    pub voxels: [[[voxel::Voxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub vertices: Vec<voxel::Vertex>,
    pub indices: Vec<u16>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            voxels: [[[voxel::Voxel::new_empty(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn get_voxel(&self, x: usize, y: usize, z: usize) -> Option<voxel::Voxel> {
        if !(0..16).contains(&x) && !(0..16).contains(&y) && !(0..16).contains(&z) {
            return None;
        }
        Some(self.voxels[x][y][z])
    }

    pub fn build_voxels(&mut self, world_pos: &cgmath::Vector3<f32>) {
        use noise::{NoiseFn, Perlin};
        let perlin = Perlin::new(484);
        let down_scale = 0.027f64;

        let mut vert_index = 0;

        for (x, voxel) in self.voxels.iter_mut().enumerate() {
            for (y, voxel) in voxel.iter_mut().enumerate() {
                for (z, voxel) in voxel.iter_mut().enumerate() {
                    let (tmp_x, tmp_y, tmp_z) = (
                        (world_pos.x as f64 + x as f64) * down_scale,
                        (world_pos.y as f64 + y as f64) * down_scale,
                        (world_pos.z as f64 + z as f64) * down_scale,
                    );
                    let density = perlin.get([tmp_x, tmp_y, tmp_z]);
                    if density > 0.3f64 {
                        voxel.set_block_type(voxel::BlockType::Grass);

                        for i in 0..8 {
                            self.vertices.push(voxel::Voxel::get_vertex(x as f32, y as f32, z as f32, i).unwrap())
                        }

                        /*
                        self.indices.push(vert_index);
                        self.indices.push(vert_index + 1);
                        self.indices.push(vert_index + 2);
                        self.indices.push(vert_index);
                        self.indices.push(vert_index + 2);
                        self.indices.push(vert_index + 3);
                        */
                        self.indices.push(vert_index + 0);self.indices.push(vert_index + 1);self.indices.push(vert_index + 3);
                        self.indices.push(vert_index + 1);self.indices.push(vert_index + 2);self.indices.push(vert_index + 3);
                        self.indices.push(vert_index + 4);self.indices.push(vert_index + 0);self.indices.push(vert_index + 7);
                        self.indices.push(vert_index + 0);self.indices.push(vert_index + 3);self.indices.push(vert_index + 7);
                        self.indices.push(vert_index + 1);self.indices.push(vert_index + 5);self.indices.push(vert_index + 2);
                        self.indices.push(vert_index + 5);self.indices.push(vert_index + 6);self.indices.push(vert_index + 2);
                        self.indices.push(vert_index + 3);self.indices.push(vert_index + 2);self.indices.push(vert_index + 7);
                        self.indices.push(vert_index + 2);self.indices.push(vert_index + 6);self.indices.push(vert_index + 7);
                        self.indices.push(vert_index + 4);self.indices.push(vert_index + 5);self.indices.push(vert_index + 0);
                        self.indices.push(vert_index + 5);self.indices.push(vert_index + 1);self.indices.push(vert_index + 0);
                        self.indices.push(vert_index + 7);self.indices.push(vert_index + 6);self.indices.push(vert_index + 4);
                        self.indices.push(vert_index + 6);self.indices.push(vert_index + 5);self.indices.push(vert_index + 4);
                    }

                    vert_index += 8;
                }
            }
        }
    }

    pub fn create_buffers(&mut self, world_pos: &cgmath::Vector3<f32>) {
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {

                }
            }
        }
    }
}
