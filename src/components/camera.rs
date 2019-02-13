use nalgebra_glm as glm;
use specs::{Component, Entity, HashMapStorage, NullStorage};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct ActiveCamera(pub Option<Entity>);

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub enum Camera {
    Ortho { size: f32, ratio: f32 },
    Persp { fov: f32, aspect: f32 },
}

impl Camera {
    pub fn get_projection_matrix(&self) -> glm::Mat4 {
        match self {
            &Camera::Ortho { size, ratio } => {
                glm::ortho(-size, size, -size * ratio, size * ratio, 0.1, 100.0)
            }
            &Camera::Persp { fov, aspect } => glm::perspective(aspect, fov, 0.1, 100.0),
        }
    }
}
