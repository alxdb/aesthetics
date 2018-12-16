use specs::{Component, HashMapStorage};

#[derive(Component)]
#[storage(HashMapStorage)]
pub enum Camera {
    Ortho {
        size: f32
    },
    Persp {
        fov: f32
    }
}