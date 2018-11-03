use nalgebra_glm as glm;

pub mod cube;
pub use self::cube::Cube;
pub mod sphere;
pub use self::sphere::Sphere;

pub type IndexType = u16;

#[derive(Debug, Clone)]
pub struct MeshData {
    points: Vec<glm::Vec3>,
    indices: Vec<IndexType>,
    index_type: glium::index::PrimitiveType,
}

impl MeshData {
    pub fn get_points(&self) -> &Vec<glm::Vec3> {
        &self.points
    }
    pub fn get_indices(&self) -> (&Vec<IndexType>, glium::index::PrimitiveType) {
        (&self.indices, self.index_type)
    }
    pub fn update_points<F>(&mut self, update_function: F)
    where
        F: Fn(&glm::Vec3) -> glm::Vec3,
    {
        for point in self.points.iter_mut() {
            *point = update_function(point);
        }
    }
}

pub trait MeshObject {
    fn ref_mesh(&self) -> &MeshData;
}
