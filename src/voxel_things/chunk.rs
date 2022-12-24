use noise::NoiseFn;

use super::voxel;

pub struct ChunkMesh {
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub num_indices: u16,
}

impl ChunkMesh {
    pub fn new() -> Self {
        Self {
            vertex_buffer: None,
            index_buffer: None,
            num_indices: 0,
        }
    }
}

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    pub voxels: [[[voxel::Voxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

struct ChunkIterator<'a> {
    chunk: &'a Chunk,
    x: usize,
    y: usize,
    z: usize,
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = (voxel::Voxel, usize, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let voxel = self.chunk.voxels[self.x][self.y][self.z];
        self.x += 1;
        if self.x >= CHUNK_SIZE {
            self.x = 0;
            self.y += 1;
            if self.y >= CHUNK_SIZE {
                self.y = 0;
                self.z += 1;
            }
        }
        Some((voxel, self.x, self.y, self.z))
    }
}

impl Chunk {
    fn iter_mut(&mut self) -> ChunkIterator {
        ChunkIterator {
            chunk: &mut self,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            voxels: [[[voxel::Voxel::new_empty(); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    pub fn build_voxels(&mut self) {
        let perlin = noise::Perlin::new(484);

        for (voxel, x, y, z) in self.iter_mut() {
            let density = perlin.get([x as f64, y as f64, z as f64]);

            voxel.block_type = if density > 0.3 {
                voxel::BlockType::Grass
            } else {
                voxel::BlockType::Empty
            };
        }
    }
}

pub fn build_chunk_mesh(chunk: &mut ChunkMesh, device: &wgpu::Device) {

}
