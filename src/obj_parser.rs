use std::{collections::HashMap, fs};
use crate::triangles::{TriangleMesh, VertexAttribute, VertexAttributeLayout};

#[derive(Eq, Hash, PartialEq)]
pub enum ObjType {
    VERTEX,
    NORMAL,
    TEXTURE
}

pub struct FaceLayout {
    map : HashMap<ObjType, usize>
}

impl FaceLayout {
    pub fn new(vertex : Option<usize>, texture : Option<usize>, normal : Option<usize>) -> Self {
        let mut map = HashMap::new();

        if let Some(i) = vertex {
            map.insert(ObjType::VERTEX, i);
        }
        if let Some(i) = texture {
            map.insert(ObjType::TEXTURE, i);
        }
        if let Some(i) = normal {
            map.insert(ObjType::NORMAL, i);
        }

        FaceLayout { map }
    }

    fn update_indicies(&self, verts : &mut Vec<u32>, norms : &mut Vec<u32>, tex : &mut Vec<u32>, indicies : &Vec<u32>) {
        if let Some(indx) = self.map.get(&ObjType::VERTEX) {
            verts.push(indicies[*indx])
        }
        if let Some(indx) = self.map.get(&ObjType::NORMAL) {
            norms.push(indicies[*indx])
        }
        if let Some(indx) = self.map.get(&ObjType::TEXTURE) {
            tex.push(indicies[*indx])
        }
    }

    fn make_verticies(&self, 
        verts : &mut Vec<Vec<f32>>, norms : &mut Vec<Vec<f32>>, tex : &mut Vec<Vec<f32>>,
        vert_indicies : &Vec<u32>, norm_indicies : &Vec<u32>, tex_indicies : &Vec<u32>
    ) -> Vec<f32> {
        let mut verticies = Vec::new();

        for i in 0..vert_indicies.len() {
            if self.map.contains_key(&ObjType::VERTEX) {
                verticies.extend(verts[vert_indicies[i] as usize].clone());
            }
            if self.map.contains_key(&ObjType::NORMAL) {
                verticies.extend(norms[norm_indicies[i] as usize].clone());
            }
            if self.map.contains_key(&ObjType::TEXTURE) {
                verticies.extend(tex[tex_indicies[i] as usize].clone());
            }
        }

        verticies
    }
}

fn add_coordinate(elms : &Vec<&str>, arr : &mut Vec<Vec<f32>>, dim : usize) {
    let v = &elms[1..=dim];
    let v_f32 : Vec<f32> = v.iter()
        .map(|x| x.parse::<f32>().unwrap())
        .collect();

    arr.push(v_f32);
}

pub fn obj_to_mesh(filename : &str, face_layout : &FaceLayout) -> TriangleMesh {

    let content = fs::read_to_string(filename)
        .expect(&format!("Could not read file: {}", filename));

    let rows = content.split("\n");

    let mut verts : Vec<Vec<f32>> = Vec::new();
    let mut norms : Vec<Vec<f32>> = Vec::new();
    let mut tex : Vec<Vec<f32>> = Vec::new();

    let mut vertex_indicies : Vec<u32> = Vec::new();
    let mut norm_indicies : Vec<u32> = Vec::new();
    let mut tex_indicies : Vec<u32> = Vec::new();

    for row in rows {
        let elms : Vec<&str> = row.split_whitespace().collect();

        if elms.len() == 0 {continue;}

        match elms[0] {
            "v" => {
                add_coordinate(&elms, &mut verts, 3);
            },
            "vn" => {
                add_coordinate(&elms, &mut norms, 3);
            },
            "vt" => {
                add_coordinate(&elms, &mut tex, 2);
            }
            "f" => {
                let face_elms = &elms[1..4];
                
                // for each (a/b/c)
                for f in face_elms {
                    // indicies = [a, b, c]
                    let indicies : Vec<u32> = f
                        .split("/")
                        .map(|x| 
                            if let Ok(n) = x.parse::<u32>() {
                                n - 1
                            } else {
                                0
                            })
                        .collect();

                    face_layout.update_indicies(
                        &mut vertex_indicies,
                        &mut norm_indicies,
                        &mut tex_indicies,
                        &indicies
                    );
                }


            }
            _ => ()
        }
    }

    let verticies = face_layout.make_verticies(
        &mut verts, &mut norms, &mut tex,
        &vertex_indicies, &norm_indicies, &tex_indicies
    );

    println!("{:?}", &verticies[0..30]);

    let vec3_size = 3 * std::mem::size_of::<f32>() as i32;
    let vec2_size = 2 * std::mem::size_of::<f32>() as i32;

    let layout = VertexAttributeLayout::new(
        vec![
            VertexAttribute::new(0, 3, vec3_size, gl::FLOAT),
            // VertexAttribute::new(1, 3, vec3_size, gl::FLOAT),
            VertexAttribute::new(2, 2, vec2_size, gl::FLOAT)
        ]
    );

    TriangleMesh::from_array_indicies(
        verticies,
        vertex_indicies,
        norm_indicies,
        tex_indicies,
        layout
    )
}