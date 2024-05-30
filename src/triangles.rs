pub struct Triangle {
    verticies : [[f32; 3];3],
    colors : [[f32; 3];3]
}

impl Triangle {
    pub fn new(p1 : [f32;3], c1 : [f32;3], p2 : [f32;3], c2 : [f32;3], p3 : [f32;3], c3 : [f32;3]) -> Self {
        Triangle {
            verticies : [
                p1, p2, p3
            ],
            colors : [
                c1, c2, c3
            ]
        }
    }
}

pub struct TriangleMesh {
    triangles : Vec<Triangle>
}

impl TriangleMesh {
    pub fn new(triangles : Vec<Triangle>) -> Self {
        TriangleMesh { triangles }
    }

    pub fn from_array_indicies(array : Vec<f32>, indicies : Vec<u16>) -> Self {
        let num_tris = indicies.len() / 3;

        for i in 0..num_tris {
            let i1 = indicies[3 * i] as usize;
            let i2 = indicies[3 * i + 1] as usize;
            let i3 = indicies[3 * i + 2] as usize;
        }

        todo!()
    }

    pub fn verticies(&self) -> Vec<f32> {
        let mut verts = Vec::new();

        for tri in &self.triangles {
            for i in 0..3 {
                verts.extend(tri.verticies[i]);
                verts.extend(tri.colors[i]);
            }
        }

        verts
    }

    pub fn indicies(&self) -> Vec<u16> {
        let mut elms = Vec::with_capacity(3 * self.triangles.len());

        for i in 0..3 * self.triangles.len() {
            elms.push(i as u16);
        }

        elms
    }
}