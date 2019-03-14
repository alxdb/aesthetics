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
    normals: Vec<glm::Vec3>,
    texture: Vec<glm::Vec2>,
    indices: Option<Vec<IndexType>>,
    index_type: glium::index::PrimitiveType,
}

impl Component for MeshData {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl MeshData {
    pub fn get_points(&self) -> &Vec<glm::Vec3> {
        &self.points
    }

    pub fn get_normals(&self) -> &Vec<glm::Vec3> {
        &self.normals
    }

    pub fn get_tex_coords(&self) -> &Vec<glm::Vec2> {
        &self.texture
    }

    pub fn get_indices(&self) -> &Option<Vec<IndexType>> {
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

pub fn cube(dims: glm::Vec3) -> MeshData {
    // Points
    let mut cube_points = Vec::new();
    for (i, j, k) in iproduct!([-1, 1].iter(), [-1, 1].iter(), [-1, 1].iter()) {
        let point = glm::vec3(*i as f32, *j as f32, *k as f32);
        cube_points.push(point);
    }
    // Faces
    let mut sides = [[[0; 4]; 2]; 3];

    let mut cube_textures = Vec::new();
    for (dim, side_pair) in sides.iter_mut().enumerate() {
        for (i, side) in side_pair.iter_mut().enumerate() {
            for ((j, k), s) in iproduct!(0..=1, 0..=1).zip(side.iter_mut()) {
                match dim {
                    0 => *s = i + j * 2 + k * 4,
                    1 => *s = k + i * 2 + j * 4,
                    2 => *s = j + k * 2 + i * 4,
                    _ => panic!(
                        "something has gone terribly wrong, this is not a 4 dimensional universe"
                    ),
                }
                cube_textures.push(glm::vec2(j as f32, k as f32));
            }
        }
    }
    // Triangulation
    let mut points = Vec::new();
    let mut normals = Vec::new();
    let mut texture = Vec::new();
    for (dim, side_pair) in sides.iter().enumerate() {
        let mut normal = match dim {
            0 => -glm::Vec3::z(),
            1 => -glm::Vec3::y(),
            2 => -glm::Vec3::x(),
            _ => panic!("oh no..."),
        };
        for side in side_pair.iter() {
            for (o, i) in iproduct!(0..=1, 0..3) {
                let index = side[o + i];
                points.push(dims.component_mul(&cube_points[index]) / 2.0);
                texture.push(cube_textures[index]);
                normals.push(normal);
            }
            normal = normal * -1.0;
        }
    }

    MeshData {
        points,
        normals,
        texture,
        indices: None,
        index_type: glium::index::PrimitiveType::TrianglesList,
    }
}

pub fn sphere(radius: f32, segments: u16) -> MeshData {
    // Points
    let mut points = Vec::new();
    let mut normals = Vec::new();
    let mut texture = Vec::new();
    use std::f32::consts;
    for (u, v) in iproduct!(0..=segments, 0..=segments) {
        let norm_coord = glm::vec2(u as f32, v as f32) / segments as f32;
        let (azi, pol) = (norm_coord.x * consts::PI * 2.0, norm_coord.y * consts::PI);
        let point = glm::vec3(pol.sin() * azi.cos(), pol.sin() * azi.sin(), pol.cos());
        points.push(point * radius);
        normals.push(point);
        texture.push(norm_coord);
    }

    let mut indices: Vec<IndexType> = Vec::new();
    for (u, v) in iproduct!(0..=segments, 0..=segments) {
        // Faces
        let mut side = [0; 4];
        for ((i, j), s) in iproduct!(0..=1, 0..=1).zip(side.iter_mut()) {
            *s = (u + i) + (segments + 1) * (v + j);
        }
        // Triangulation
        for (o, i) in iproduct!(0..=1, 0..3) {
            indices.push(side[i + o] as IndexType);
        }
    }

    MeshData {
        points,
        normals,
        texture,
        indices: Some(indices),
        index_type: glium::index::PrimitiveType::TrianglesList,
    }
}
