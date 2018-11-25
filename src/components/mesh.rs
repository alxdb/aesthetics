use glium;
use itertools::iproduct;
use nalgebra_glm as glm;
use specs::prelude::*;

// this is simpler otherwise will need separate storage for each index type
// points will take up more VRam anyway, potential optimization
pub type IndexType = u32;

#[derive(Debug, Clone)]
pub struct MeshData {
    points: Vec<glm::Vec3>,
    indices: Vec<IndexType>,
    index_type: glium::index::PrimitiveType,
}

impl Component for MeshData {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl MeshData {
    pub fn get_points(&self) -> &Vec<glm::Vec3> {
        &self.points
    }

    pub fn get_indices(&self) -> &Vec<IndexType> {
        &self.indices
    }

    pub fn get_index_type(&self) -> &glium::index::PrimitiveType {
        &self.index_type
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

pub fn cube(dims: (f32, f32, f32)) -> MeshData {
    // Points
    let mut points = Vec::new();
    for (i, j, k) in iproduct!(0..=1, 0..=1, 0..=1) {
        let point = glm::vec3(i as f32 * dims.0, j as f32 * dims.1, k as f32 * dims.2);
        points.push(point);
    }
    let mut indices: Vec<IndexType> = Vec::new();
    // Faces
    let mut sides = [[[0; 4]; 2]; 3];

    for (dim, side_pair) in sides.iter_mut().enumerate() {
        for (i, side) in side_pair.iter_mut().enumerate() {
            for ((j, k), s) in iproduct!(0..=1, 0..=1).zip(side.iter_mut()) {
                match dim {
                    0 => *s = i + j * 2 + k * 4,
                    1 => *s = k + i * 2 + j * 4,
                    2 => *s = j + k * 2 + i * 4,
                    _ => panic!("something has gone terribly wrong"),
                }
            }
        }
    }
    // Triangulation
    for side_pair in sides.iter() {
        for side in side_pair.iter() {
            for (o, i) in iproduct!(0..=1, 0..3) {
                indices.push(side[i + o] as IndexType);
            }
        }
    }

    MeshData {
        points,
        indices,
        index_type: glium::index::PrimitiveType::TrianglesList,
    }
}

pub fn sphere(radius: f32, segments: u16) -> MeshData {
    // Points
    let mut points = Vec::new();
    use std::f32::consts;
    for (u, v) in iproduct!(0..=segments, 0..=segments) {
        let norm_coord = glm::vec2(u as f32, v as f32) / (segments - 1) as f32;
        let (azi, pol) = (norm_coord.x * consts::PI * 2.0, norm_coord.y * consts::PI);
        let point = glm::vec3(pol.sin() * azi.cos(), pol.sin() * azi.sin(), pol.cos());
        points.push(point * radius);
    }

    let mut indices: Vec<IndexType> = Vec::new();
    for (u, v) in iproduct!(0..segments, 0..segments) {
        // Faces
        let mut side = [0; 4];
        for ((i, j), s) in iproduct!(0..=1, 0..=1).zip(side.iter_mut()) {
            *s = (u + i) + segments * (v + j);
        }
        // Triangulation
        for (o, i) in iproduct!(0..=1, 0..3) {
            indices.push(side[i + o] as IndexType);
        }
    }

    MeshData {
        points,
        indices,
        index_type: glium::index::PrimitiveType::TrianglesList,
    }
}
