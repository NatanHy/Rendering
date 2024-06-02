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
    triangles : Vec<Triangle>,
    pub verticies : Vec<f32>,
    pub indicies : Vec<u16>
}

impl TriangleMesh {
    pub fn new(triangles : Vec<Triangle>) -> Self {
        let mut verts = Vec::new();
        let mut elms = Vec::with_capacity(3 * triangles.len());

        for tri in &triangles {
            for i in 0..3 {
                verts.extend(tri.verticies[i]);
                verts.extend(tri.colors[i]);
            }
        }


        for i in 0..3 * triangles.len() {
            elms.push(i as u16);
        }

        TriangleMesh { triangles , verticies : verts, indicies : elms}
    }

    pub fn from_array_indicies(verticies : Vec<f32>, indicies : Vec<u16>) -> Self {
        TriangleMesh {triangles : Vec::new(), verticies, indicies}
    }
}