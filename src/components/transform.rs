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
}