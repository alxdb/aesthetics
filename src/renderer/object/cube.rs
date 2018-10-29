use super::*;
use glium;
use itertools::iproduct;
use nalgebra_glm as glm;

#[derive(Debug)]
pub struct Cube {
    mesh: MeshData<u8>,
}

impl Cube {
    pub fn new(dims: (f32, f32, f32)) -> Self {
        let mut points = Vec::new();
        for (i, j, k) in iproduct!(0..=1, 0..=1, 0..=1) {
            let point = glm::vec3(i as f32 * dims.0, j as f32 * dims.1, k as f32 * dims.2);
            points.push(point);
        }
        let mut indices = Vec::new();
        // Faces
        let mut sides: [[[u8; 4]; 2]; 3] = [[[0; 4]; 2]; 3];
        for (dim, side_pair) in sides.iter_mut().enumerate() {
            for (i, side) in side_pair.iter_mut().enumerate() {
                for ((j, k), s) in iproduct!(0..=1, 0..=1).zip(side.iter_mut()) {
                    match dim {
                        0 => *s = i as u8 + j * 2 + k * 4,
                        1 => *s = k + i as u8 * 2 + j * 4,
                        2 => *s = j + k * 2 + i as u8 * 4,
                        _ => panic!("oob"),
                    }
                }
            }
        }
        // Triangulation
        for side_pair in sides.iter() {
            for side in side_pair.iter() {
                for (o, i) in iproduct!(0..=1, 0..3) {
                    indices.push(side[i + o]);
                }
            }
        }

        Cube {
            mesh: MeshData {
                points,
                indices,
                index_type: glium::index::PrimitiveType::TrianglesList,
            },
        }
    }
}

impl Object for Cube {}

impl Mesh<u8> for Cube {
    fn get_mesh(&self) -> &MeshData<u8> {
        &self.mesh
    }
}
