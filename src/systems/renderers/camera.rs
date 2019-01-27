use specs::{Component, HashMapStorage};

#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub enum Camera {
    Ortho {
        size: f32
    },
    Persp {
        fov: f32
    }
}