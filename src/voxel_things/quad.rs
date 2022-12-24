use super::{voxel, direction::Direction};
use cgmath::Vector3;
use rand::Rng;

pub struct Quad {
    pub corners: [Vector3<f32>; 4],
    pub color: [f32; 3],
    pub direction: Direction,
}

const HALF_SIZE: f32 = voxel::SIZE / 2.0;

impl Quad {
    pub fn new(direction: Direction, pos: Vector3<f32>) -> Self {
        let corners = match direction {
            Direction::Left => [
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
            ],
            Direction::Right => [
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
            ],
            Direction::Down => [
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
            ],
            Direction::Up => [
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z + HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
            ],
            Direction::Back => [
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
            ],
            Direction::Forward => [
                Vector3::new(pos.x + HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x + HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y + HALF_SIZE, pos.z - HALF_SIZE),
                Vector3::new(pos.x - HALF_SIZE, pos.y - HALF_SIZE, pos.z - HALF_SIZE),
            ],
        };

        let green = 0.4;
        let color = [
            rand::thread_rng().gen_range(0f32..0.1f32),
            (1.0 - green + rand::thread_rng().gen_range(0f32..1.0 - green)),
            rand::thread_rng().gen_range(0f32..0.1f32),
        ];

        Self { corners, color, direction }
    }
}
