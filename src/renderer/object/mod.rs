use nalgebra_glm as glm;

pub mod cube;
pub use self::cube::Cube;
pub mod sphere;
pub use self::sphere::Sphere;

pub trait Object {}

#[derive(Debug)]
pub struct MeshData<T: glium::index::Index> {
    points: Vec<glm::Vec3>,
    indices: Vec<T>,
    index_type: glium::index::PrimitiveType,
}

impl<T: glium::index::Index> MeshData<T> {
    pub fn get_points(&self) -> &Vec<glm::Vec3> {
        &self.points
    }
    pub fn get_indices(&self) -> (&Vec<T>, glium::index::PrimitiveType) {
        (&self.indices, self.index_type)
    }
}

pub trait Mesh<T: glium::index::Index>: Object {
    fn get_mesh(&self) -> &MeshData<T>;
}
