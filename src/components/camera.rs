use nalgebra_glm as glm;
use specs::{Component, Entity, HashMapStorage, NullStorage};

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct ActiveCamera(pub Option<Entity>);

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub enum Camera {
    Ortho { size: f32, aspect: f32 },
    Persp { fov: f32, aspect: f32 },
}

impl Camera {
    pub fn get_projection_matrix(&self) -> glm::Mat4 {
        match self {
            &Camera::Ortho { size, aspect } => {
                glm::ortho(-size, size, -size * aspect, size * aspect, 0.1, 100.0)
            }
            &Camera::Persp { fov, aspect } => glm::perspective(aspect, fov, 0.1, 100.0),
        }
    }

    pub fn update_aspect(&mut self, new_aspect: f32) {
        match self {
            Camera::Ortho { aspect, .. } => *aspect = new_aspect,
            Camera::Persp { aspect, .. } => *aspect = new_aspect,
        }
    }
}
