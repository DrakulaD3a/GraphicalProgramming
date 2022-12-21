use crate::chunk::{Chunk, self};

pub fn build_chunk_mesh(chunk: &mut Chunk, chunk_world_pos: &cgmath::Vector3<f32>) {
    let chunk_size = chunk::CHUNK_SIZE as i32;
    for x in 0..chunk_size {
        for y in 0..chunk_size {
            for z in 0..chunk_size {
                if let Ok((voxel, back, left, down)) = get_adjacent_voxels(x, y, z) {
                    process_voxel(&voxel, 
                }
            }
        }
    }
}
