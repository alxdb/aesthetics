use nalgebra_glm as glm;
use specs::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Transform {
    pub pos: glm::Vec3,
    pub rot: glm::Quat,
}

impl Transform {
    pub fn new(pos: glm::Vec3) -> Self {
        Transform {
            pos,
            rot: glm::Quat::identity(),
        }
    }

    pub fn make_model_matrix(&self) -> glm::Mat4 {
        glm::translation(&self.pos) * glm::quat_to_mat4(&self.rot)
    }

    pub fn make_view_matrix(&self) -> glm::Mat4 {
        glm::quat_to_mat4(&glm::quat_conjugate(&self.rot)) * glm::translation(&(-self.pos))
    }
}
