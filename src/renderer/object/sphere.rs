use super::*;
use glium;
use itertools::iproduct;
use nalgebra_glm as glm;
use std::f32::consts;

pub struct Sphere {
    mesh: MeshData,
}

impl Sphere {
    pub fn new(radius: f32, segments: u16) -> Self {
        let mut points = Vec::new();
        for (u, v) in iproduct!(0..=segments, 0..=segments) {
            let norm_coord = glm::vec2(u as f32, v as f32) / (segments - 1) as f32;
            let (azi, pol) = (norm_coord.x * consts::PI * 2.0, norm_coord.y * consts::PI);
            let point = glm::vec3(pol.sin() * azi.cos(), pol.sin() * azi.sin(), pol.cos());
            points.push(point * radius);
        }

        let mut indices = Vec::new();
        for (u, v) in iproduct!(0..segments, 0..segments) {
            let mut side: [IndexType; 4] = [0; 4];
            for ((i, j), s) in iproduct!(0..=1, 0..=1).zip(side.iter_mut()) {
                *s = (u + i) + segments * (v + j);
            }
            for (o, i) in iproduct!(0..=1, 0..3) {
                indices.push(side[i + o]);
            }
        }

        Sphere {
            mesh: MeshData {
                points,
                indices,
                index_type: glium::index::PrimitiveType::TrianglesList,
            },
        }
    }
}

impl MeshObject for Sphere {
    fn ref_mesh(&self) -> &MeshData {
        &self.mesh
    }
}
