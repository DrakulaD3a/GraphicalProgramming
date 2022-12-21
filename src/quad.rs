use cgmath::Vector3;
use rand::Rng;

use crate::voxel;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Forward,
    Back,
}

pub struct Quad {
    pub corners: [[f32; 3]; 4],
    pub color: [f32; 3],
}

const HALF_SIZE: f32 = voxel::SIZE / 2.0;

impl Quad {
    pub fn new(direction: Direction, pos: Vector3<f32>) -> Self {
        let corners = match direction {
            Direction::Left => [
                [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE],
                [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
            ],
            Direction::Right => [
                [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE],
                [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
            ],
            // assuming it's correct this is under i believe
            Direction::Down => [
                [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
            ],
            Direction::Up => [
                [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE],
                [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
            ],
            Direction::Back => [
                [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                [pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
            ],
            Direction::Forward => [
                [pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
                [pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                [pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE],
                [pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE],
            ],
        };

        let color = [rand::thread_rng().gen_range(0f32..0.1f32), 0.6f32 + rand::thread_rng().gen_range(0f32..0.4f32), rand::thread_rng().gen_range(0f32..0.1f32)];

        Self {
            corners,
            color,
        }
    }
}
