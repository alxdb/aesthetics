use super::*;
use glium;
use itertools::iproduct;
use nalgebra_glm as glm;

#[derive(Debug)]
pub struct Cube {
    mesh: MeshData,
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
        let mut sides: [[[IndexType; 4]; 2]; 3] = [[[0; 4]; 2]; 3];
        for (dim, side_pair) in sides.iter_mut().enumerate() {
            for (i, side) in side_pair.iter_mut().enumerate() {
                for ((j, k), s) in iproduct!(0..=1, 0..=1).zip(side.iter_mut()) {
                    match dim {
                        0 => *s = i as IndexType + j * 2 + k * 4,
                        1 => *s = k + i as IndexType * 2 + j * 4,
                        2 => *s = j + k * 2 + i as IndexType * 4,
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

impl MeshObject for Cube {
    fn ref_mesh(&self) -> &MeshData {
        &self.mesh
    }
}
